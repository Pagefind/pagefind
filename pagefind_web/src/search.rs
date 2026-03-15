use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
    ops::{Add, AddAssign, Div},
};

use crate::{util::*, PageWord, RankingWeights, WordData};
use bit_set::BitSet;
use pagefind_stem::Stemmer;

use crate::SearchIndex;

pub struct PageSearchResult {
    pub page: String,
    pub group_hash: String,
    pub page_index: usize,
    pub page_length: u32,
    pub page_score: f32,
    pub word_locations: Vec<BalancedWordScore>,
    pub verbose_scores: Option<Vec<(String, ScoringMetrics, BM25Params)>>,
    pub matched_meta_fields: Vec<String>,
    pub verbose_meta_scores: Option<Vec<VerboseMetaScore>>,
}

/// Verbose metadata scoring info for the playground.
/// Shows how each metadata field contributes to the meta boost.
#[derive(Debug, Clone)]
pub struct VerboseMetaScore {
    pub field_name: String,
    pub field_weight: f32,
    pub matched_terms: Vec<String>,
    pub matched_idf: f32,
    pub query_total_idf: f32,
    pub coverage: f32,
    pub coverage_boost: f32,
}

/// Query term IDF breakdown for the playground.
/// Shows the IDF contribution of each search term.
#[derive(Debug, Clone)]
pub struct QueryTermIdf {
    pub term: String,
    pub idf: f32,
}

struct MatchingPageWord<'a> {
    word: &'a PageWord,
    word_str: &'a str,
    length_bonus: f32,
    diacritic_bonus: f32,
    /// Index into the original query terms
    /// (for looking up the combined IDF score of the original term)
    query_term_index: usize,
}

#[derive(Debug, Clone)]
struct VerboseWordLocation<'a> {
    word_str: &'a str,
    weight: u8,
    word_location: u32,
    length_bonus: f32,
    query_term_index: usize,
}

#[derive(Debug, Clone)]
pub struct BalancedWordScore {
    pub weight: u8,
    pub balanced_score: f32,
    pub word_location: u32,
    pub verbose_word_info: Option<VerboseWordInfo>,
}

#[derive(Debug, Clone)]
pub struct VerboseWordInfo {
    pub word: String,
    pub length_bonus: f32,
}

#[derive(Debug)]
pub struct BM25Params {
    pub weighted_term_frequency: f32,
    pub document_length: f32,
    pub average_page_length: f32,
    pub total_pages: usize,
    pub pages_containing_term: usize,
    pub length_bonus: f32,
}

#[derive(Clone, Copy)]
pub struct ScoringMetrics {
    pub idf: f32,
    pub bm25_tf: f32,
    pub raw_tf: f32,
    pub pagefind_tf: f32,
    pub score: f32,
}

/// Returns a multiplier based on how well the original query matches the word variant diacritics.
fn diacritic_bonus(
    original_query_term: &str,
    variant_form: &str,
    diacritic_similarity: f32,
) -> f32 {
    if diacritic_similarity > 0.0 && diacritics_match(original_query_term, variant_form) {
        1.0 + diacritic_similarity
    } else {
        1.0
    }
}

fn diacritics_match(original_query_term: &str, variant_form: &str) -> bool {
    variant_form == original_query_term
        || variant_form.starts_with(original_query_term)
        || original_query_term.starts_with(variant_form)
}

/// Returns a score between 0.0 and 1.0 for the given word.
/// 1.0 implies the word is the exact length we need,
/// and that decays as the word becomes longer or shorter than the query word.
/// As `term_similarity_ranking` trends to zero, all output trends to 1.0.
/// As `term_similarity_ranking` increases, the score decays faster as differential grows.
fn word_length_bonus(differential: u8, term_similarity_ranking: f32) -> f32 {
    let std_dev = 2.0_f32;
    let base = (-0.5 * (differential as f32).powi(2) / std_dev.powi(2)).exp();
    let max_value = term_similarity_ranking.exp();
    (base * term_similarity_ranking).exp() / max_value
}

/// Calculate the Inverse Document Frequency for a term.
fn calculate_idf(total_pages: usize, pages_containing_term: usize) -> f32 {
    (total_pages as f32 - pages_containing_term as f32 + 0.5)
        .div(pages_containing_term as f32 + 0.5)
        .add(1.0)
        .ln()
}

fn calculate_bm25_word_score(
    BM25Params {
        weighted_term_frequency,
        document_length,
        average_page_length,
        total_pages,
        pages_containing_term,
        length_bonus,
    }: BM25Params,
    ranking: &RankingWeights,
) -> ScoringMetrics {
    let weighted_with_length = weighted_term_frequency * length_bonus;

    let k1 = ranking.term_saturation;
    let b = ranking.page_length;

    let idf = calculate_idf(total_pages, pages_containing_term);

    let bm25_tf = (k1 + 1.0) * weighted_with_length
        / (k1 * (1.0 - b + b * (document_length / average_page_length)) + weighted_with_length);

    // Use ranking.term_frequency to interpolate between only caring about BM25's term frequency,
    // and only caring about the original weighted word count on the page.
    // Attempting to scale the original weighted word count to roughly the same bounds as the BM25 output (k1 + 1)
    let raw_count_scalar = average_page_length / 5.0;
    let raw_tf = (weighted_with_length / raw_count_scalar).min(k1 + 1.0);
    let pagefind_tf = (1.0 - ranking.term_frequency) * raw_tf + ranking.term_frequency * bm25_tf;

    debug!({
        format! {"TF is {pagefind_tf:?}, IDF is {idf:?}"}
    });

    ScoringMetrics {
        idf,
        bm25_tf,
        raw_tf,
        pagefind_tf,
        score: idf * pagefind_tf,
    }
}

fn calculate_individual_word_score(
    VerboseWordLocation {
        word_str,
        weight,
        length_bonus,
        word_location,
        query_term_index: _,
    }: VerboseWordLocation,
    playground_mode: bool,
) -> BalancedWordScore {
    let balanced_score = (weight as f32).powi(2) * length_bonus;

    BalancedWordScore {
        weight,
        balanced_score,
        word_location,
        verbose_word_info: if playground_mode {
            Some(VerboseWordInfo {
                word: word_str.to_string(),
                length_bonus,
            })
        } else {
            None
        },
    }
}

impl SearchIndex {
    pub fn exact_term(
        &self,
        term: &str,
        original_query: &str,
        filter_results: Option<BitSet>,
        exact_diacritics: bool,
    ) -> (Vec<usize>, Vec<PageSearchResult>) {
        debug!({
            format! {"Searching {:?}", term}
        });

        let original_terms: Vec<&str> = original_query.split(' ').collect();
        let split_term = stems_from_term(term);

        let mut unfiltered_results: Vec<usize> = vec![];
        let mut maps = Vec::new();
        let mut words = Vec::new();

        for (term_idx, term) in split_term.iter().enumerate() {
            if let Some(word_data) = self.words.get(term.as_ref()) {
                let original_term = original_terms.get(term_idx).copied().unwrap_or("");
                let mut set = BitSet::new();

                if !exact_diacritics || diacritics_match(original_term, term) {
                    words.extend(&word_data.pages);
                    for page in &word_data.pages {
                        set.insert(page.page as usize);
                    }
                }
                for variant in &word_data.additional_variants {
                    if !exact_diacritics || diacritics_match(original_term, &variant.form) {
                        words.extend(&variant.pages);
                        for page in &variant.pages {
                            set.insert(page.page as usize);
                        }
                    }
                }
                maps.push(set);
            } else {
                // If we can't find this word, there are obviously no exact matches
                return (vec![], vec![]);
            }
        }

        if !maps.is_empty() {
            let map = match intersect_maps(maps) {
                Some(maps) => maps,
                // Results must exist at this point.
                None => std::process::abort(),
            };
            unfiltered_results.extend(map.iter());
            maps = vec![map];
        }

        if let Some(filter) = filter_results {
            maps.push(filter);
        }

        let results = match intersect_maps(maps) {
            Some(map) => map,
            None => return (vec![], vec![]),
        };

        let mut pages: Vec<PageSearchResult> = vec![];

        for page_index in results.iter() {
            let word_locations: Vec<Vec<(u8, u32)>> = words
                .iter()
                .filter_map(|p| {
                    if p.page as usize == page_index {
                        Some(p.locs.iter().map(|d| *d).collect())
                    } else {
                        None
                    }
                })
                .collect();
            debug!({
                format! {"Word locations {:?}", word_locations}
            });

            let mut found_match = false;

            if let (Some(loc_0), Some(loc_rest)) = (word_locations.get(0), word_locations.get(1..))
            {
                'indexes: for (_, pos) in loc_0 {
                    let mut i = *pos;
                    for subsequent in loc_rest {
                        i += 1;
                        // Test each subsequent word map to try and find a contiguous block
                        if !subsequent.iter().any(|(_, p)| *p == i) {
                            continue 'indexes;
                        }
                    }
                    let page = match self.pages.get(page_index) {
                        Some(p) => p,
                        None => std::process::abort(),
                    };
                    let search_result = PageSearchResult {
                        page: page.hash.clone(),
                        group_hash: page.group_hash.clone(),
                        page_index,
                        page_score: 1.0,
                        page_length: page.word_count,
                        word_locations: ((*pos..=i).map(|w| BalancedWordScore {
                            weight: 1,
                            balanced_score: 1.0,
                            word_location: w,
                            verbose_word_info: None, // TODO: bring playground info to quoted searches
                        }))
                        .collect(),
                        verbose_scores: None, // TODO: bring playground info to quoted searches
                        matched_meta_fields: vec![],
                        verbose_meta_scores: None, // TODO: bring playground info to quoted searches
                    };
                    pages.push(search_result);
                    found_match = true;
                    break 'indexes;
                }
            }

            if !found_match {
                let meta_word_locations: Vec<HashMap<u16, Vec<u32>>> = words
                    .iter()
                    .filter_map(|p| {
                        if p.page as usize == page_index && !p.meta_locs.is_empty() {
                            let mut by_field: HashMap<u16, Vec<u32>> = HashMap::new();
                            for &(field_id, position) in &p.meta_locs {
                                by_field.entry(field_id).or_default().push(position);
                            }
                            Some(by_field)
                        } else {
                            None
                        }
                    })
                    .collect();

                if meta_word_locations.len() >= split_term.len() {
                    let all_field_ids: HashSet<u16> = meta_word_locations
                        .iter()
                        .flat_map(|m| m.keys().copied())
                        .collect();

                    'fields: for field_id in all_field_ids {
                        let field_positions: Vec<&Vec<u32>> = meta_word_locations
                            .iter()
                            .filter_map(|m| m.get(&field_id))
                            .collect();

                        if field_positions.len() != meta_word_locations.len() {
                            continue 'fields;
                        }

                        if let (Some(loc_0), Some(loc_rest)) =
                            (field_positions.get(0), field_positions.get(1..))
                        {
                            'meta_indexes: for pos in *loc_0 {
                                let mut i = *pos;
                                for subsequent in loc_rest {
                                    i += 1;
                                    if !subsequent.iter().any(|p| *p == i) {
                                        continue 'meta_indexes;
                                    }
                                }
                                let page = match self.pages.get(page_index) {
                                    Some(p) => p,
                                    None => std::process::abort(),
                                };

                                let field_name = self.meta_fields.get(field_id as usize);
                                let meta_boost = field_name
                                    .and_then(|name| {
                                        self.ranking_weights.meta_weights.get(name).copied()
                                    })
                                    .unwrap_or(1.0);

                                let search_result = PageSearchResult {
                                    page: page.hash.clone(),
                                    group_hash: page.group_hash.clone(),
                                    page_index,
                                    page_score: meta_boost,
                                    page_length: page.word_count,
                                    word_locations: vec![],
                                    verbose_scores: None,
                                    matched_meta_fields: field_name
                                        .map(|n| vec![n.clone()])
                                        .unwrap_or_default(),
                                    verbose_meta_scores: None,
                                };
                                pages.push(search_result);
                                found_match = true;
                                break 'fields;
                            }
                        }
                    }
                }
            }

            // Single word handling - only for single-word exact searches
            if !found_match && split_term.len() == 1 {
                let page = match self.pages.get(page_index) {
                    Some(p) => p,
                    None => std::process::abort(),
                };
                if let Some(loc_0) = word_locations.get(0) {
                    let search_result = PageSearchResult {
                        page: page.hash.clone(),
                        group_hash: page.group_hash.clone(),
                        page_index,
                        page_score: 1.0,
                        page_length: page.word_count,
                        word_locations: loc_0
                            .iter()
                            .map(|(weight, word_location)| BalancedWordScore {
                                weight: *weight,
                                balanced_score: *weight as f32,
                                word_location: *word_location,
                                verbose_word_info: None, // TODO: bring playground info to quoted searches
                            })
                            .collect(),
                        verbose_scores: None, // TODO: bring playground info to quoted searches
                        matched_meta_fields: vec![],
                        verbose_meta_scores: None, // TODO: bring playground info to quoted searches
                    };
                    pages.push(search_result);
                }
            }
        }

        (unfiltered_results, pages)
    }

    pub fn search_term(
        &self,
        term: &str,
        original_query: &str,
        filter_results: Option<BitSet>,
        exact_diacritics: bool,
    ) -> (Vec<usize>, Vec<PageSearchResult>, Option<Vec<QueryTermIdf>>) {
        debug!({
            format! {"Searching {:?}", term}
        });

        let total_pages = self.pages.len();

        let mut unfiltered_results: Vec<usize> = vec![];
        let mut maps = Vec::new();
        let mut words: Vec<MatchingPageWord> = Vec::new();
        let split_term = stems_from_term(term);
        let original_terms: Vec<&str> = original_query.split(' ').collect();
        // Track combined page count for each original query term.
        // We use this to calculate a true-minimum IDF score when ranking,
        // for example we calculate the IDF of all to* words.
        let mut combined_page_counts: Vec<usize> = Vec::with_capacity(split_term.len());

        for (term_idx, term) in split_term.iter().enumerate() {
            let original_term = original_terms.get(term_idx).copied().unwrap_or("");

            let mut word_maps = Vec::new();
            for (word, word_data) in self.find_word_extensions(&term) {
                let length_differential: u8 = (word.len().abs_diff(term.len()) + 1)
                    .try_into()
                    .unwrap_or(std::u8::MAX);
                let length_bonus =
                    word_length_bonus(length_differential, self.ranking_weights.term_similarity);

                let mut matching_word_variants = Vec::new();
                if !exact_diacritics || diacritics_match(original_term, word) {
                    matching_word_variants.push((word, &word_data.pages));
                }
                for variant in &word_data.additional_variants {
                    if !exact_diacritics || diacritics_match(original_term, &variant.form) {
                        matching_word_variants.push((&variant.form, &variant.pages));
                    }
                }

                let mut set = BitSet::new();

                for (form, pages) in matching_word_variants {
                    let boost = diacritic_bonus(
                        original_term,
                        form,
                        self.ranking_weights.diacritic_similarity,
                    );
                    words.extend(pages.iter().map(|pageword| MatchingPageWord {
                        word: pageword,
                        word_str: word,
                        length_bonus,
                        diacritic_bonus: boost,
                        query_term_index: term_idx,
                    }));
                    for page in pages {
                        set.insert(page.page as usize);
                    }
                }

                word_maps.push(set);
            }
            if let Some(result) = union_maps(word_maps) {
                combined_page_counts.push(result.len());
                maps.push(result);
            } else {
                combined_page_counts.push(0);
            }
        }
        // In the case where a search term was passed, but not found,
        // make sure we cause the entire search to return no results.
        if !split_term.is_empty() && maps.is_empty() {
            maps.push(BitSet::new());
        }

        if !maps.is_empty() {
            let map = match intersect_maps(maps) {
                Some(maps) => maps,
                // Results must exist at this point.
                None => std::process::abort(),
            };
            unfiltered_results.extend(map.iter());
            maps = vec![map];
        }

        if let Some(filter) = filter_results {
            maps.push(filter);
        } else if maps.is_empty() {
            let mut all_filter = BitSet::new();
            for i in 0..self.pages.len() {
                all_filter.insert(i);
            }
            maps.push(all_filter);
        }

        let results = match intersect_maps(maps) {
            Some(map) => map,
            None => return (vec![], vec![], None),
        };

        // Calculate total IDF for original query terms.
        // Each term's IDF is based on the combined page count of all its extensions.
        // (in other words: "how common is any word starting with this prefix").
        let query_total_idf: f32 = combined_page_counts
            .iter()
            .map(|&count| calculate_idf(total_pages, count))
            .sum();

        // Collect verbose query IDF breakdown for playground mode
        let verbose_query_idfs = if self.playground_mode {
            Some(
                split_term
                    .iter()
                    .zip(combined_page_counts.iter())
                    .map(|(term, &count)| QueryTermIdf {
                        term: term.to_string(),
                        idf: calculate_idf(total_pages, count),
                    })
                    .collect(),
            )
        } else {
            None
        };

        let mut pages: Vec<PageSearchResult> = vec![];

        for (page_index, page) in results
            .iter()
            .flat_map(|p| self.pages.get(p).map(|page| (p, page)))
        {
            let mut word_locations: Vec<_> = words
                .iter()
                .filter_map(|w| {
                    if w.word.page as usize == page_index {
                        Some(
                            w.word
                                .locs
                                .iter()
                                .map(|(weight, location)| VerboseWordLocation {
                                    word_str: w.word_str,
                                    weight: *weight,
                                    word_location: *location,
                                    length_bonus: w.length_bonus,
                                    query_term_index: w.query_term_index,
                                }),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
                .collect();

            let mut meta_field_matches: HashMap<u16, HashMap<&str, f32>> = HashMap::new();
            for w in words.iter() {
                if w.word.page as usize == page_index && !w.word.meta_locs.is_empty() {
                    let idf = calculate_idf(total_pages, combined_page_counts[w.query_term_index]);
                    for &(field_id, _position) in &w.word.meta_locs {
                        meta_field_matches
                            .entry(field_id)
                            .or_default()
                            .insert(w.word_str, idf);
                    }
                }
            }
            word_locations
                .sort_unstable_by_key(|VerboseWordLocation { word_location, .. }| *word_location);

            debug!({
                format! {"Found the raw word locations {:?}", word_locations}
            });

            let mut unique_word_locations: Vec<BalancedWordScore> =
                Vec::with_capacity(word_locations.len());
            let mut weighted_words: BTreeMap<&str, usize> = BTreeMap::new();

            // Group words by position to handle compound words properly.
            // When multiple query terms match the same position (e.g., "git" and "absorb"
            // both matching "git-absorb"), each term should contribute to weighted_words.
            let mut position_groups: Vec<Vec<VerboseWordLocation>> = vec![];
            let mut current_position: Option<u32> = None;

            for word in word_locations.into_iter() {
                if current_position == Some(word.word_location) {
                    position_groups.last_mut().unwrap().push(word);
                } else {
                    current_position = Some(word.word_location);
                    position_groups.push(vec![word]);
                }
            }

            for group in position_groups {
                let mut seen_terms: BTreeMap<usize, (&str, u8, f32)> = BTreeMap::new();

                for word in &group {
                    seen_terms.entry(word.query_term_index).or_insert((
                        word.word_str,
                        word.weight,
                        word.length_bonus,
                    ));
                }

                let min_weight: u8 = seen_terms.values().map(|(_, w, _)| *w).min().unwrap_or(1);
                let is_compound = seen_terms.len() > 1;
                let effective_weight = if is_compound {
                    // Boost for matching multiple query terms at same position
                    min_weight.saturating_mul(seen_terms.len() as u8)
                } else {
                    min_weight
                };

                for (_, (word_str, _, _)) in &seen_terms {
                    weighted_words
                        .entry(*word_str)
                        .or_default()
                        .add_assign(effective_weight as usize);
                }

                let first = group.first().unwrap();
                let combined_length_bonus: f32 = seen_terms.values().map(|(_, _, lb)| *lb).sum();

                unique_word_locations.push(calculate_individual_word_score(
                    VerboseWordLocation {
                        word_str: first.word_str,
                        weight: effective_weight,
                        word_location: first.word_location,
                        length_bonus: combined_length_bonus,
                        query_term_index: first.query_term_index,
                    },
                    self.playground_mode,
                ));
            }

            debug!({
                format! {"Coerced to unique locations {:?}", unique_word_locations}
            });
            debug!({
                format! {"Words have the final weights {:?}", weighted_words}
            });

            let mut verbose_scores = if self.playground_mode {
                Some(vec![])
            } else {
                None
            };
            let word_scores =
                weighted_words
                    .into_iter()
                    .map(|(word_str, weighted_term_frequency)| {
                        let matched_word = words
                            .iter()
                            .find(|w| w.word_str == word_str && w.word.page as usize == page_index)
                            .expect("word should be in the initial set");

                        let pages_containing_original_query_term =
                            combined_page_counts[matched_word.query_term_index];

                        let params = || BM25Params {
                            weighted_term_frequency: (weighted_term_frequency as f32) / 24.0,
                            document_length: page.word_count as f32,
                            average_page_length: self.average_page_length,
                            total_pages,
                            pages_containing_term: pages_containing_original_query_term,
                            length_bonus: matched_word.length_bonus,
                        };

                        debug!({
                            format! {"Calculating BM25 with the params {:?}", params()}
                        });
                        debug!({
                            format! {"And the weights {:?}", self.ranking_weights}
                        });

                        let score = calculate_bm25_word_score(params(), &self.ranking_weights);

                        debug!({
                            format! {"BM25 gives us the score {:?}", score.score}
                        });

                        if let Some(verbose_scores) = verbose_scores.as_mut() {
                            verbose_scores.push((word_str.to_string(), score, params()));
                        }

                        score.score * matched_word.diacritic_bonus
                    });

            let base_page_score: f32 = word_scores.sum();

            let mut meta_boost: f32 = 0.0;
            let mut verbose_meta_scores = if self.playground_mode {
                Some(vec![])
            } else {
                None
            };
            for (field_id, word_idfs) in &meta_field_matches {
                if let Some(field_name) = self.meta_fields.get(*field_id as usize) {
                    let field_weight = self
                        .ranking_weights
                        .meta_weights
                        .get(field_name)
                        .copied()
                        .unwrap_or(1.0);
                    let matched_idf: f32 = word_idfs.values().sum();
                    let coverage = matched_idf / query_total_idf;
                    let coverage_boost = if query_total_idf > 0.0 {
                        // Squared coverage to penalize partial meta matches.
                        // That is, matching just the word "The" in a title is nearly meaningless.
                        field_weight * matched_idf * coverage * coverage
                    } else {
                        0.0
                    };
                    meta_boost += coverage_boost;
                    debug!({
                        format!(
                            "Meta boost: field '{}' matched {} words (IDF {:.2} of {:.2} total), weight {}, coverage boost {:.2}",
                            field_name,
                            word_idfs.len(),
                            matched_idf,
                            query_total_idf,
                            field_weight,
                            coverage_boost
                        )
                    });

                    if let Some(ref mut scores) = verbose_meta_scores {
                        scores.push(VerboseMetaScore {
                            field_name: field_name.clone(),
                            field_weight,
                            matched_terms: word_idfs.keys().map(|s| s.to_string()).collect(),
                            matched_idf,
                            query_total_idf,
                            coverage,
                            coverage_boost,
                        });
                    }
                }
            }

            let page_score = base_page_score + meta_boost;

            let matched_meta_fields: Vec<String> = meta_field_matches
                .keys()
                .filter_map(|field_id| self.meta_fields.get(*field_id as usize).cloned())
                .collect();

            let search_result = PageSearchResult {
                page: page.hash.clone(),
                group_hash: page.group_hash.clone(),
                page_index,
                page_score,
                page_length: page.word_count,
                word_locations: unique_word_locations,
                verbose_scores,
                matched_meta_fields,
                verbose_meta_scores,
            };

            debug!({
                format! {"Page {} has {} matching terms (in {} total words), and has the boosted word frequency of {:?}", search_result.page, search_result.word_locations.len(), page.word_count, search_result.page_score}
            });

            pages.push(search_result);
        }

        debug!({ "Sorting by score" });
        pages.sort_unstable_by(|a, b| {
            b.page_score
                .partial_cmp(&a.page_score)
                .unwrap_or(Ordering::Equal)
        });

        (unfiltered_results, pages, verbose_query_idfs)
    }

    fn find_word_extensions(&self, term: &str) -> Vec<(&String, &WordData)> {
        let mut extensions = vec![];
        let mut longest_prefix = None;
        for (key, results) in self.words.iter() {
            if key.starts_with(term) {
                debug!({
                    format! {"Adding {:#?} to the query", key}
                });
                extensions.push((key, results));
            } else if term.starts_with(key)
                && key.len() > longest_prefix.map(String::len).unwrap_or_default()
            {
                longest_prefix = Some(key);
            }
        }
        if extensions.is_empty() {
            debug!({ "No word extensions found, checking the inverse" });
            if let Some(longest_prefix) = longest_prefix {
                if let Some(results) = self.words.get(longest_prefix) {
                    debug!({
                        format! {"Adding the prefix {:#?} to the query", longest_prefix}
                    });
                    extensions.push((longest_prefix, results));
                }
            }
        }
        extensions
    }
}

pub fn stems_from_term<'a>(term: &'a str) -> Vec<Cow<'a, str>> {
    if term.trim().is_empty() {
        return vec![];
    }
    let stemmer = Stemmer::try_create_default();
    term.split(' ')
        .map(|word| match &stemmer {
            Ok(stemmer) => stemmer.stem(word),
            // If we wound up without a stemmer,
            // charge ahead without stemming.
            Err(_) => word.into(),
        })
        .collect()
}

fn intersect_maps(mut maps: Vec<BitSet>) -> Option<BitSet> {
    let mut maps = maps.drain(..);
    if let Some(mut base) = maps.next() {
        for map in maps {
            base.intersect_with(&map);
        }
        Some(base)
    } else {
        None
    }
}

fn union_maps(mut maps: Vec<BitSet>) -> Option<BitSet> {
    let mut maps = maps.drain(..);
    if let Some(mut base) = maps.next() {
        for map in maps {
            base.union_with(&map);
        }
        Some(base)
    } else {
        None
    }
}
