use hashbrown::HashMap;
use rayon::prelude::*;
use std::collections::BTreeSet;

use crate::{
    fossick::{FossickedData, FossickedWord, MetaFossickedWord},
    index::index_metadata::MetaFilter,
    utils::full_hash,
    SearchOptions,
};
use anyhow::{bail, Result};
use index_filter::{FilterIndex, PackedValue};
use index_metadata::{MetaChunk, MetaIndex, MetaPage};
use index_words::{PackedPage, PackedVariant, PackedWord, WordIndex};

use self::index_metadata::MetaSort;

mod index_filter;
mod index_metadata;
mod index_words;

pub struct PagefindIndexes {
    pub word_indexes: HashMap<String, Vec<u8>>,
    pub filter_indexes: HashMap<String, Vec<u8>>,
    pub meta_index: (String, Vec<u8>),
    pub fragments: Vec<(String, String)>,
    pub sorts: Vec<String>,
    pub language: String,
    pub word_count: usize,
}

#[derive(Clone)]
struct IntermediaryPageData {
    full_hash: String,
    encoded_data: String,
    word_count: usize,
    page_number: usize,
}

#[derive(Debug)]
enum SortType {
    String,
    Number,
}

/// Results from processing a single page in parallel.
/// Contains the word map, filter entries, and encoded page data for one page.
struct PageProcessingResult {
    word_map: HashMap<String, PackedWord>,
    filter_entries: Vec<(String, String, usize)>, // (filter, value, page_number)
    encoded_page: IntermediaryPageData,
}

pub async fn build_indexes(
    mut pages: Vec<FossickedData>,
    language: String,
    options: &SearchOptions,
) -> Result<PagefindIndexes> {
    let mut meta = MetaIndex {
        version: options.version.into(),
        pages: Vec::new(),
        index_chunks: Vec::new(),
        filters: Vec::new(),
        sorts: Vec::new(),
        meta_fields: Vec::new(),
    };

    /*
        - Collect all sort keys
        - Sort `pages` by one of them and set `default_sort`
        - Do the main enumerate loop as an iter_mut and set page numbers
        - Later on, for each other sort key:
            - Sort the `pages` array and output the page numbers to `alternate_sorts`
    */

    let mut word_map: HashMap<String, PackedWord> = HashMap::new();
    let mut filter_map: HashMap<String, HashMap<String, Vec<usize>>> = HashMap::new();
    let mut fragment_hashes: HashMap<String, IntermediaryPageData> = HashMap::new();
    let mut fragments: Vec<(usize, (String, IntermediaryPageData))> = Vec::new();

    for (page_number, page) in pages.iter_mut().enumerate() {
        page.fragment.page_number = page_number;
    }

    // Get all possible sort keys
    let mut sorts: Vec<_> = pages
        .iter()
        .flat_map(|page| page.sort.keys().cloned())
        .collect();
    sorts.sort_unstable();
    sorts.dedup();

    // Determine the best sorting parser that fits all available values for each given key
    let mut sort_types: HashMap<String, SortType> = HashMap::new();
    for sort in sorts.iter() {
        let mut sort_values = pages.iter().flat_map(|page| page.sort.get(sort));
        sort_types.insert(
            sort.clone(),
            if sort_values.all(|v| parse_int_sort(v).is_some() || parse_float_sort(v).is_some()) {
                SortType::Number
            } else {
                SortType::String
            },
        );
    }

    for (sort_key, sort_type) in sort_types {
        let mut page_values: Vec<_> = pages
            .iter()
            .flat_map(|page| {
                page.sort
                    .get(&sort_key)
                    .map(|v| (v, page.fragment.page_number))
            })
            .collect();
        options.logger.v_info(format!(
            "Prebuilding sort order for {sort_key}, processed as type: {sort_type:#?}"
        ));
        match sort_type {
            SortType::String => page_values.sort_by_key(|p| p.0),
            SortType::Number => page_values.sort_by(|p1, p2| {
                let p1 = parse_int_sort(p1.0)
                    .map(|i| i as f32)
                    .unwrap_or_else(|| parse_float_sort(p1.0).unwrap_or_default());
                let p2 = parse_int_sort(p2.0)
                    .map(|i| i as f32)
                    .unwrap_or_else(|| parse_float_sort(p2.0).unwrap_or_default());

                p1.total_cmp(&p2)
            }),
        }
        meta.sorts.push(MetaSort {
            sort: sort_key,
            pages: page_values.into_iter().map(|p| p.1).collect(),
        });
    }

    let mut meta_fields_set: BTreeSet<String> = BTreeSet::new();
    for page in pages.iter() {
        meta_fields_set.extend(page.fragment.data.meta.keys().cloned());
    }
    meta.meta_fields = meta_fields_set.into_iter().collect();

    // Clone meta_fields for use in parallel processing
    let meta_fields_ref = meta.meta_fields.clone();
    let language_ref = language.clone();

    // Process pages in parallel - each page builds its own word map
    let page_results: Vec<PageProcessingResult> = pages
        .into_par_iter()
        .map(|page| {
            let mut local_word_map: HashMap<String, PackedWord> = HashMap::new();

            // Meta field IDs were assigned per-page,
            // but need to be remapped to the global meta_fields order.
            let page_field_order: Vec<&String> = page.fragment.data.meta.keys().collect();
            let field_id_map: Vec<u16> = page_field_order
                .iter()
                .map(|name| meta_fields_ref.iter().position(|f| f == *name).unwrap() as u16)
                .collect();

            for (word, positions) in page.word_data {
                // Group positions by original_word for this page
                let mut normalized_positions: Vec<FossickedWord> = Vec::new();
                let mut variant_positions: HashMap<String, Vec<FossickedWord>> = HashMap::new();

                for fossicked in positions {
                    if fossicked.original_word.is_none() {
                        // No diacritics - original matches normalized form
                        normalized_positions.push(fossicked);
                    } else {
                        // Original form differs (has diacritics) - stored instead in additional_variants
                        variant_positions
                            .entry(fossicked.original_word.clone().unwrap())
                            .or_default()
                            .push(fossicked);
                    }
                }

                let packed_word =
                    local_word_map
                        .entry(word.clone())
                        .or_insert_with(|| PackedWord {
                            word: word.clone(),
                            pages: Vec::new(),
                            additional_variants: Vec::new(),
                        });

                if !normalized_positions.is_empty() {
                    packed_word.pages.push(positions_to_packed_page(
                        normalized_positions,
                        page.fragment.page_number,
                    ));
                }

                for (variant_form, variant_pos) in variant_positions {
                    let variant_page =
                        positions_to_packed_page(variant_pos, page.fragment.page_number);

                    if let Some(existing_variant) = packed_word
                        .additional_variants
                        .iter_mut()
                        .find(|v| v.form == variant_form)
                    {
                        existing_variant.pages.push(variant_page);
                    } else {
                        packed_word.additional_variants.push(PackedVariant {
                            form: variant_form,
                            pages: vec![variant_page],
                        });
                    }
                }
            }

            for (word, meta_positions) in page.meta_word_data {
                let mut normalized_meta_positions: Vec<MetaFossickedWord> = Vec::new();
                let mut variant_meta_positions: HashMap<String, Vec<MetaFossickedWord>> =
                    HashMap::new();

                for mut meta_fossicked in meta_positions {
                    meta_fossicked.field_id = field_id_map[meta_fossicked.field_id as usize];
                    if let Some(original) = meta_fossicked.original_word.clone() {
                        variant_meta_positions
                            .entry(original)
                            .or_default()
                            .push(meta_fossicked);
                    } else {
                        normalized_meta_positions.push(meta_fossicked);
                    }
                }

                let packed_word =
                    local_word_map
                        .entry(word.clone())
                        .or_insert_with(|| PackedWord {
                            word: word.clone(),
                            pages: Vec::new(),
                            additional_variants: Vec::new(),
                        });

                if !normalized_meta_positions.is_empty() {
                    let meta_locs = meta_positions_to_packed(normalized_meta_positions);

                    if let Some(existing_page) = packed_word
                        .pages
                        .iter_mut()
                        .find(|p| p.page_number == page.fragment.page_number)
                    {
                        existing_page.meta_locs = meta_locs;
                    } else {
                        packed_word.pages.push(PackedPage {
                            page_number: page.fragment.page_number,
                            locs: vec![],
                            meta_locs,
                        });
                    }
                }

                // Handle diacritic variants in meta
                for (variant_form, variant_pos) in variant_meta_positions {
                    let meta_locs = meta_positions_to_packed(variant_pos);

                    if let Some(existing_variant) = packed_word
                        .additional_variants
                        .iter_mut()
                        .find(|v| v.form == variant_form)
                    {
                        if let Some(existing_page) = existing_variant
                            .pages
                            .iter_mut()
                            .find(|p| p.page_number == page.fragment.page_number)
                        {
                            existing_page.meta_locs = meta_locs;
                        } else {
                            existing_variant.pages.push(PackedPage {
                                page_number: page.fragment.page_number,
                                locs: vec![],
                                meta_locs,
                            });
                        }
                    } else {
                        packed_word.additional_variants.push(PackedVariant {
                            form: variant_form,
                            pages: vec![PackedPage {
                                page_number: page.fragment.page_number,
                                locs: vec![],
                                meta_locs,
                            }],
                        });
                    }
                }
            }

            // Collect filter entries for this page
            let filter_entries: Vec<(String, String, usize)> = page
                .fragment
                .data
                .filters
                .iter()
                .flat_map(|(filter, values)| {
                    values.iter().map(move |value| {
                        (filter.clone(), value.clone(), page.fragment.page_number)
                    })
                })
                .collect();

            // Compute encoded page data
            let encoded_data = serde_json::to_string(&page.fragment.data).unwrap();
            let encoded_page = IntermediaryPageData {
                full_hash: format!("{}_{}", language_ref, full_hash(encoded_data.as_bytes())),
                word_count: page.fragment.data.word_count,
                page_number: page.fragment.page_number,
                encoded_data,
            };

            PageProcessingResult {
                word_map: local_word_map,
                filter_entries,
                encoded_page,
            }
        })
        .collect();

    // Merge results sequentially
    for result in page_results {
        // Merge word maps using Entry API to avoid unnecessary clones
        for (word, packed) in result.word_map {
            match word_map.entry(word) {
                hashbrown::hash_map::Entry::Occupied(mut entry) => {
                    let existing = entry.get_mut();
                    existing.pages.extend(packed.pages); // Move, not clone
                                                         // Merge additional_variants
                    for variant in packed.additional_variants {
                        if let Some(existing_variant) = existing
                            .additional_variants
                            .iter_mut()
                            .find(|v| v.form == variant.form)
                        {
                            existing_variant.pages.extend(variant.pages);
                        } else {
                            existing.additional_variants.push(variant);
                        }
                    }
                }
                hashbrown::hash_map::Entry::Vacant(entry) => {
                    entry.insert(packed);
                }
            }
        }

        // Merge filter entries using or_default for cleaner code
        for (filter, value, page_number) in result.filter_entries {
            filter_map
                .entry(filter)
                .or_default()
                .entry(value)
                .or_default()
                .push(page_number);
        }

        // Handle fragment hashing (must be sequential due to collision handling)
        let encoded_page = result.encoded_page;
        let mut short_hash = &encoded_page.full_hash[0..=(language.len() + 7)];

        // If we hit a collision, extend one until we stop colliding
        // TODO: There are some collision issues here.
        // If two builds match a collision in different orders the hashes will swap,
        // which could return incorrect data due to files being cached.
        while let Some(collision) = fragment_hashes.get(short_hash) {
            if collision.full_hash == encoded_page.full_hash {
                // These pages are identical. Add both under the same hash.
                fragments.push((
                    collision.word_count,
                    (collision.full_hash.clone(), collision.clone()),
                ));
            } else {
                let new_length = short_hash.len();
                short_hash = &encoded_page.full_hash[0..=new_length];
            }
        }
        fragment_hashes.insert(short_hash.to_string(), encoded_page);
    }

    // Sort pages within each word by page_number to maintain correct order after parallel processing
    for packed_word in word_map.values_mut() {
        packed_word.pages.sort_by_key(|p| p.page_number);
        for variant in &mut packed_word.additional_variants {
            variant.pages.sort_by_key(|p| p.page_number);
        }
    }

    fragments.extend(
        fragment_hashes
            .into_iter()
            .map(|(hash, frag)| (frag.word_count, (hash, frag))),
    );
    fragments.sort_by_cached_key(|(_, (_, fragment))| fragment.page_number);

    meta.pages
        .extend(fragments.iter().map(|(word_count, (hash, _))| MetaPage {
            hash: hash.clone(),
            word_count: *word_count as u32,
        }));

    // TODO: Change filter indexes to BTree to give them a stable hash.
    // Encode filter indexes in parallel
    // Convert hashbrown HashMap to Vec for rayon compatibility
    let filter_map_vec: Vec<_> = filter_map.into_iter().collect();
    let encoded_filters: Vec<(String, Vec<u8>, String)> = filter_map_vec
        .into_par_iter()
        .map(|(filter, values)| {
            let mut filter_index: Vec<u8> = Vec::new();
            let _ = minicbor::encode::<FilterIndex, &mut Vec<u8>>(
                FilterIndex {
                    filter: filter.clone(),
                    values: values
                        .into_iter()
                        .map(|(value, pages)| PackedValue { value, pages })
                        .collect(),
                },
                filter_index.as_mut(),
            );
            let hash = format!("{}_{}", language, full_hash(&filter_index));
            (filter, filter_index, hash)
        })
        .collect();

    // Handle hash collisions sequentially (required for correctness)
    let mut filter_indexes = HashMap::new();
    for (filter, filter_index, hash) in encoded_filters {
        let mut short_hash = &hash[0..=(language.len() + 7)];

        // If we hit a collision, extend one hash until we stop colliding
        // TODO: DRY
        while filter_indexes.contains_key(short_hash) {
            let new_length = short_hash.len() + 1;
            short_hash = &hash[0..=new_length];

            if short_hash.len() == hash.len() {
                break;
            }
        }
        filter_indexes.insert(short_hash.to_string(), filter_index);
        meta.filters.push(MetaFilter {
            filter,
            hash: short_hash.to_string(),
        })
    }

    if TryInto::<u32>::try_into(meta.pages.len()).is_err() {
        options.logger.error(format!(
            "Language {} has too many documents to index, must be < {}",
            language,
            u32::MAX
        ));
        bail!(
            "Language {language} has too many documents to index, must be < {}",
            u32::MAX
        );
    }

    // TODO: Parameterize these chunk sizes via byte size rather than word count
    let word_count = word_map.len();
    let chunks = chunk_index(word_map, options.index_chunk_size);
    meta.index_chunks = chunk_meta(&chunks);

    // Encode word index chunks in parallel (delta encoding + CBOR serialization)
    let encoded_chunks: Vec<(usize, Vec<u8>, String)> = chunks
        .into_par_iter()
        .enumerate()
        .map(|(i, chunk)| {
            // Delta-encode page numbers within each word's page list
            let delta_chunk: Vec<PackedWord> = chunk
                .into_iter()
                .map(|mut word| {
                    let mut last_page: usize = 0;
                    for page in &mut word.pages {
                        let delta = page.page_number - last_page;
                        last_page = page.page_number;
                        page.page_number = delta;
                    }
                    // Also handle additional_variants
                    for variant in &mut word.additional_variants {
                        let mut last_page: usize = 0;
                        for page in &mut variant.pages {
                            let delta = page.page_number - last_page;
                            last_page = page.page_number;
                            page.page_number = delta;
                        }
                    }
                    word
                })
                .collect();

            let mut word_index: Vec<u8> = Vec::new();
            let _ = minicbor::encode::<WordIndex, &mut Vec<u8>>(
                WordIndex { words: delta_chunk },
                word_index.as_mut(),
            );

            let hash = format!("{}_{}", language, full_hash(&word_index));
            (i, word_index, hash)
        })
        .collect();

    // Handle hash collisions sequentially (required for correctness)
    let mut word_indexes: HashMap<String, Vec<u8>> = HashMap::new();
    for (i, word_index, hash) in encoded_chunks {
        let mut short_hash = &hash[0..=(language.len() + 7)];

        // If we hit a collision, extend one hash until we stop colliding
        while word_indexes.contains_key(short_hash) {
            let new_length = short_hash.len() + 1;
            short_hash = &hash[0..=new_length];

            if short_hash.len() == hash.len() {
                break;
            }
        }
        word_indexes.insert(short_hash.to_string(), word_index);
        meta.index_chunks[i].hash = short_hash.into();
    }

    let mut meta_index: Vec<u8> = Vec::new();
    let _ = minicbor::encode::<MetaIndex, &mut Vec<u8>>(meta, meta_index.as_mut());

    let meta_hash = format!(
        "{}_{}",
        language,
        &full_hash(&meta_index)[0..=(language.len() + 7)]
    );

    Ok(PagefindIndexes {
        word_indexes,
        filter_indexes,
        sorts,
        meta_index: (meta_hash, meta_index),
        fragments: fragments
            .into_iter()
            .map(|(_, (hash, frag))| (hash, frag.encoded_data))
            .collect(),
        language,
        word_count,
    })
}

/// Convert fossicked word positions to a packed page representation.
/// Sorts by weight (with common weight 25 first) then by position for delta encoding.
fn positions_to_packed_page(mut positions: Vec<FossickedWord>, page_number: usize) -> PackedPage {
    // A page weight of 1 is encoded as 25. Since most words should be this weight,
    // we want to sort them to be first in the locations array to reduce filesize
    // when we inline weight changes.
    // We then sort by position within each weight group,
    // which helps us delta encode the index.
    positions.sort_by_cached_key(|p| (if p.weight == 25 { 0 } else { p.weight }, p.position));

    let mut current_weight = 25;
    let mut weighted_positions = Vec::with_capacity(positions.len());
    let mut last_position = 0;

    // Calculate our output list of positions with weights.
    // This is a vec of page positions, with a change in weight for subsequent positions
    // denoted by a negative integer.
    for FossickedWord {
        position, weight, ..
    } in positions
    {
        if weight != current_weight {
            // Weight change: emit marker + absolute position, new delta base
            weighted_positions.push(-(weight as i32) - 1);
            weighted_positions.push(position as i32);
            last_position = position;
            current_weight = weight;
        } else {
            // emit delta from previous position
            weighted_positions.push((position - last_position) as i32);
            last_position = position;
        }
    }

    PackedPage {
        page_number,
        locs: weighted_positions,
        meta_locs: vec![],
    }
}

fn chunk_index(word_map: HashMap<String, PackedWord>, chunk_size: usize) -> Vec<Vec<PackedWord>> {
    // TODO: Use ye olde BTree
    let mut words = word_map
        .into_iter()
        .map(|(_, w)| w)
        .collect::<Vec<PackedWord>>();
    words.sort_by_key(|w| w.word.clone());

    let mut index_chunks = Vec::new();

    let mut index_chunk = Vec::new();
    let mut index_chunk_size = 0;
    for word in words.into_iter() {
        index_chunk_size += word
            .pages
            .iter()
            .map(|p| p.locs.len() + p.meta_locs.len() + 1)
            .sum::<usize>();
        index_chunk.push(word);
        if index_chunk_size >= chunk_size {
            index_chunks.push(index_chunk.clone());
            index_chunk.clear();
            index_chunk_size = 0;
        }
    }
    if !index_chunk.is_empty() {
        index_chunks.push(index_chunk);
    }

    index_chunks
}

fn chunk_meta(indexes: &[Vec<PackedWord>]) -> Vec<MetaChunk> {
    let mut named_chunks: Vec<MetaChunk> = Vec::new();

    for chunk in indexes.iter() {
        named_chunks.push(MetaChunk {
            from: chunk.first().map_or("".into(), |w| w.word.clone()),
            to: chunk.last().map_or("".into(), |w| w.word.clone()),
            hash: "".into(),
        });
    }
    if named_chunks.len() > 1 {
        for i in 0..named_chunks.len() - 1 {
            let chunks = &mut named_chunks[i..=i + 1];
            let prefixes = get_prefixes((&chunks[0].to, &chunks[1].from));
            // Only trim 'to' if it won't create an invalid range (to < from)
            if prefixes.0 >= chunks[0].from {
                chunks[0].to = prefixes.0;
            }
            // Always trim the next chunk's 'from'
            chunks[1].from = prefixes.1;
        }
    }

    named_chunks
}

fn get_prefixes((a, b): (&str, &str)) -> (String, String) {
    let common_prefix_length: usize = b
        .chars()
        .zip(a.chars())
        .take_while(|&(a, b)| a == b)
        .count();

    let a_prefix = a.chars().take(common_prefix_length + 1).collect::<String>();
    let b_prefix = b.chars().take(common_prefix_length + 1).collect::<String>();

    (a_prefix, b_prefix)
}

fn parse_int_sort(value: &str) -> Option<i32> {
    lexical_core::parse::<i32>(value.as_bytes()).ok()
}

fn parse_float_sort(value: &str) -> Option<f32> {
    lexical_core::parse::<f32>(value.as_bytes()).ok()
}

/// Encode meta positions with field ID markers.
/// Negative numbers switch field IDs, positive numbers are delta-encoded positions.
/// e.g., [-1, 0, 5, -4, 2] means: field 0 positions [0, 5], field 3 positions [2]
fn meta_positions_to_packed(mut positions: Vec<MetaFossickedWord>) -> Vec<i32> {
    if positions.is_empty() {
        return vec![];
    }

    positions.sort_by_key(|p| (p.field_id, p.position));

    let mut current_field: Option<u16> = None;
    let mut last_position: u32 = 0;
    let mut result = Vec::with_capacity(positions.len() + 8);

    for MetaFossickedWord {
        field_id, position, ..
    } in positions
    {
        if Some(field_id) != current_field {
            result.push(-((field_id as i32) + 1));
            current_field = Some(field_id);
            last_position = 0;
        }
        result.push((position - last_position) as i32);
        last_position = position;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    trait Mock {
        fn word(&mut self, word: &str, page_number: usize, locs: Vec<i32>);
    }
    impl Mock for HashMap<String, PackedWord> {
        fn word(&mut self, word: &str, page_number: usize, locs: Vec<i32>) {
            let page = PackedPage {
                page_number,
                locs,
                meta_locs: vec![],
            };
            match self.get_mut(word) {
                Some(w) => w.pages.push(page),
                None => {
                    let _ = self.insert(
                        word.into(),
                        PackedWord {
                            word: word.into(),
                            pages: vec![page],
                            additional_variants: vec![],
                        },
                    );
                }
            }
        }
    }

    fn test_words() -> HashMap<String, PackedWord> {
        let mut words = HashMap::new();
        words.word("apple", 1, vec![20, 40, 60]);
        words.word("apple", 5, vec![3, 6, 9]);
        words.word("apricot", 5, vec![45, 3432, 6003]);
        words.word("banana", 5, vec![100, 500, 900, 566]);
        words.word("peach", 5, vec![383, 2, 678]);

        words
    }

    #[test]
    fn build_index_chunks() {
        let chunks = chunk_index(test_words(), 8);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0][0].word, "apple");
        assert_eq!(chunks[1][0].word, "apricot");
        assert_eq!(chunks[1][1].word, "banana");
        assert_eq!(chunks[2][0].word, "peach");
    }

    #[test]
    fn build_chunk_meta() {
        let chunks = chunk_index(test_words(), 8);
        let meta = chunk_meta(&chunks);
        assert_eq!(meta.len(), 3);
        assert_eq!(
            meta[0],
            MetaChunk {
                from: "apple".into(),
                to: "apple".into(), // Not trimmed to "app" since that would make to < from
                hash: "".into(),
            }
        );
        assert_eq!(
            meta[1],
            MetaChunk {
                from: "apr".into(),
                to: "b".into(),
                hash: "".into(),
            }
        );
        assert_eq!(
            meta[2],
            MetaChunk {
                from: "p".into(),
                to: "peach".into(),
                hash: "".into(),
            }
        );
    }

    #[test]
    fn common_prefix() {
        assert_eq!(
            get_prefixes(("apple", "apricot")),
            ("app".into(), "apr".into())
        );
        assert_eq!(
            get_prefixes(("cataraman", "yacht")),
            ("c".into(), "y".into())
        );
        assert_eq!(
            get_prefixes(("cath", "cathartic")),
            ("cath".into(), "catha".into())
        );
        // This should be an invalid state, but just in case:
        assert_eq!(
            get_prefixes(("catha", "cath")),
            ("catha".into(), "cath".into())
        );
    }
}
