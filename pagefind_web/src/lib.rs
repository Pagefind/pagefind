#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::collections::{BTreeMap, HashMap};

use pagefind_microjson::JSONValue;
use search::{stems_from_term, BM25Params, ScoringMetrics};
use util::*;
use wasm_bindgen::prelude::*;

use crate::search::BalancedWordScore;

mod filter;
mod filter_index;
mod index;
mod metadata;
mod search;
mod util;

pub struct PageWord {
    page: u32,
    locs: Vec<(u8, u32)>,
    meta_locs: Vec<(u16, u32)>,
}

pub struct WordVariant {
    pub form: String,
    pub pages: Vec<PageWord>,
}

pub struct WordData {
    pub pages: Vec<PageWord>,
    pub additional_variants: Vec<WordVariant>,
}

pub struct IndexChunk {
    from: String,
    to: String,
    hash: String,
}

pub struct Page {
    hash: String,
    word_count: u32,
    group_hash: String,
}

pub struct SearchIndex {
    web_version: &'static str,
    playground_mode: bool,
    generator_version: Option<String>,
    pages: Vec<Page>,
    average_page_length: f32,
    chunks: Vec<IndexChunk>,
    filter_chunks: BTreeMap<String, String>,
    words: BTreeMap<String, WordData>,
    filters: BTreeMap<String, BTreeMap<String, Vec<u32>>>,
    sorts: BTreeMap<String, Vec<u32>>,
    meta_fields: Vec<String>,
    ranking_weights: RankingWeights,
}

#[derive(Debug, Clone)]
pub struct RankingWeights {
    /// Controls page ranking based on similarity of terms to the search query (in length).
    /// Increasing this number means pages rank higher when they contain words very close to the query,
    /// e.g. if searching for `part` then `party` will boost a page higher than one containing `partition`.
    /// As this number trends to zero, then `party` and `partition` would be viewed equally.
    /// Must be >= 0
    pub term_similarity: f32,
    /// Controls how much effect the average page length has on ranking.
    /// At 1.0, ranking will strongly favour pages that are shorter than the average page on the site.
    /// At 0.0, ranking will exclusively look at term frequency, regardless of how long a document is.
    /// Must be clamped to 0..=1
    pub page_length: f32,
    /// Controls how quickly a term saturates on the page and reduces impact on the ranking.
    /// At 2.0, pages will take a long time to saturate, and pages with very high term frequencies will take over.
    /// As this number trends to 0, it does not take many terms to saturate and allow other paramaters to influence the ranking.
    /// At 0.0, terms will saturate immediately and results will not distinguish between one term and many.
    /// Must be clamped to 0..=2
    pub term_saturation: f32,
    /// Controls how much ranking uses term frequency versus raw term count.
    /// At 1.0, term frequency fully applies and is the main ranking factor.
    /// At 0.0, term frequency does not apply, and pages are ranked based on the raw sum of words and weights.
    /// Reducing this number is a good way to boost longer documents in your search results,
    /// as they no longer get penalized for having a low term frequency.
    /// Numbers between 0.0 and 1.0 will interpolate between the two ranking methods.
    /// Must be clamped to 0..=1
    pub term_frequency: f32,
    /// Controls how much boost is applied when the search query diacritics match the indexed word exactly.
    /// At 1.0, searching for "café" will boost pages containing "café" by 100% over pages containing "cafe".
    /// At 0.0, no boost is applied and all diacritic variants are treated equally.
    /// Must be >= 0
    pub diacritic_similarity: f32,
    /// Controls boost weights for metadata field matches.
    /// Keys are meta field names (e.g., "title", "description").
    /// Default: {"title": 5.0} meaning title matches get 5x boost.
    /// Fields not in this map default to 1.0 (no boost).
    pub meta_weights: HashMap<String, f32>,
}

impl Default for RankingWeights {
    fn default() -> Self {
        let mut meta_weights = HashMap::new();
        meta_weights.insert("title".to_string(), 5.0);
        Self {
            term_similarity: 1.0,
            page_length: 0.75,
            term_saturation: 1.4,
            term_frequency: 1.0,
            diacritic_similarity: 0.8,
            meta_weights,
        }
    }
}

#[cfg(debug_assertions)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(debug_assertions)]
fn debug_log(s: &str) {
    log(&format!("From WASM: {}", s));
}

#[wasm_bindgen]
pub fn init_pagefind(metadata_bytes: &[u8]) -> *mut SearchIndex {
    #[cfg(debug_assertions)]
    debug_log("Initializing Pagefind");
    let mut search_index = SearchIndex {
        web_version: env!("CARGO_PKG_VERSION"),
        playground_mode: false,
        generator_version: None,
        pages: Vec::new(),
        average_page_length: 0.0,
        chunks: Vec::new(),
        filter_chunks: BTreeMap::new(),
        words: BTreeMap::new(),
        filters: BTreeMap::new(),
        sorts: BTreeMap::new(),
        meta_fields: Vec::new(),
        ranking_weights: RankingWeights::default(),
    };

    match search_index.decode_metadata(metadata_bytes) {
        Ok(_) => Box::into_raw(Box::new(search_index)),
        #[allow(unused_variables)]
        Err(e) => {
            #[cfg(debug_assertions)]
            debug_log(&format!("{:#?}", e));
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn enter_playground_mode(ptr: *mut SearchIndex) -> *mut SearchIndex {
    debug!({ "Entering Pagefind Playground Mode" });

    let mut search_index = unsafe { Box::from_raw(ptr) };

    search_index.playground_mode = true;

    Box::into_raw(search_index)
}

#[wasm_bindgen]
pub fn set_ranking_weights(ptr: *mut SearchIndex, weights: &str) -> *mut SearchIndex {
    debug!({ "Loading Ranking Weights" });

    let Ok(weights) = JSONValue::parse(weights) else {
        return ptr;
    };

    let mut search_index = unsafe { Box::from_raw(ptr) };

    if let Ok(term_similarity) = weights
        .get_key_value("term_similarity")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.term_similarity = term_similarity.max(0.0);
    }

    if let Ok(page_length) = weights
        .get_key_value("page_length")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.page_length = page_length.clamp(0.0, 1.0);
    }

    if let Ok(term_saturation) = weights
        .get_key_value("term_saturation")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.term_saturation = term_saturation.clamp(0.0, 2.0);
    }

    if let Ok(term_frequency) = weights
        .get_key_value("term_frequency")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.term_frequency = term_frequency.clamp(0.0, 1.0);
    }

    if let Ok(diacritic_similarity) = weights
        .get_key_value("diacritic_similarity")
        .and_then(|v| v.read_float())
    {
        search_index.ranking_weights.diacritic_similarity = diacritic_similarity.max(0.0);
    }

    if let Ok(meta_weights_obj) = weights.get_key_value("meta_weights") {
        if let Ok(obj_iter) = meta_weights_obj.iter_object() {
            for (key, value) in obj_iter.filter_map(|o| o.ok()) {
                if let Ok(weight) = value.read_float() {
                    search_index
                        .ranking_weights
                        .meta_weights
                        .insert(key.to_string(), weight.max(0.0));
                }
            }
        }
    }

    Box::into_raw(search_index)
}

#[wasm_bindgen]
pub fn load_index_chunk(ptr: *mut SearchIndex, chunk_bytes: &[u8]) -> *mut SearchIndex {
    debug!({ "Loading Index Chunk" });
    let mut search_index = unsafe { Box::from_raw(ptr) };

    match search_index.decode_index_chunk(chunk_bytes) {
        Ok(_) => Box::into_raw(search_index),
        #[allow(unused_variables)]
        Err(e) => {
            debug!({ format!("{:#?}", e) });
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn load_filter_chunk(ptr: *mut SearchIndex, chunk_bytes: &[u8]) -> *mut SearchIndex {
    debug!({ "Loading Filter Chunk" });
    let mut search_index = unsafe { Box::from_raw(ptr) };

    match search_index.decode_filter_index_chunk(chunk_bytes) {
        Ok(_) => Box::into_raw(search_index),
        #[allow(unused_variables)]
        Err(e) => {
            debug!({ format!("{:#?}", e) });
            std::ptr::null_mut::<SearchIndex>()
        }
    }
}

#[wasm_bindgen]
pub fn add_synthetic_filter(ptr: *mut SearchIndex, filter: &str) -> *mut SearchIndex {
    debug!({
        format! {"Creating a synthetic index filter for {:?}", filter}
    });

    let mut search_index = unsafe { Box::from_raw(ptr) };
    search_index.decode_synthetic_filter(filter);
    Box::into_raw(search_index)
}

#[wasm_bindgen]
pub fn request_indexes(ptr: *mut SearchIndex, query: &str) -> String {
    debug!({ format!("Finding the index chunks needed for {:?}", query) });

    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes = Vec::new();

    for raw_term in query.split(' ') {
        // Check both the raw term and its stemmed versions, as the
        // chunk boundaries sit on the _stemmed_ words, which in some cases can get funky.
        let stemmed = stems_from_term(raw_term);
        let mut terms_to_check = vec![raw_term];
        for stem in &stemmed {
            if stem != raw_term {
                terms_to_check.push(stem.as_ref());
            }
        }

        for term in terms_to_check {
            let strict_chunks: Vec<_> = search_index
                .chunks
                .iter()
                .filter(|chunk| term >= &chunk.from && term <= &chunk.to)
                .collect();

            if !strict_chunks.is_empty() {
                for chunk in strict_chunks {
                    debug!({ format!("Need {:?} for {:?} (strict)", chunk.hash, term) });
                    indexes.push(chunk.hash.clone());
                }
            } else {
                // No strict match - try loose matching for prefix/extension matches.
                debug!({ format!("No strict match for {:?}, trying loose match", term) });
                for chunk in search_index.chunks.iter().filter(|chunk| {
                    let from_char_count = term.chars().count().min(chunk.from.chars().count());
                    let to_char_count = term.chars().count().min(chunk.to.chars().count());

                    let term_pre: String = term.chars().take(from_char_count).collect();
                    let chunk_pre: String = chunk.from.chars().take(from_char_count).collect();
                    let term_post: String = term.chars().take(to_char_count).collect();
                    let chunk_post: String = chunk.to.chars().take(to_char_count).collect();

                    term_pre >= chunk_pre && term_post <= chunk_post
                }) {
                    debug!({ format!("Need {:?} for {:?} (loose)", chunk.hash, term) });
                    indexes.push(chunk.hash.clone());
                }
            }
        }
    }

    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();

    let mut output = String::new();
    {
        let mut arr = write_json::array(&mut output);
        indexes.into_iter().for_each(|i| {
            arr.string(&i);
        });
    }

    output
}

#[wasm_bindgen]
pub fn request_filter_indexes(ptr: *mut SearchIndex, filters: &str) -> String {
    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes = search_index.filter_chunks(filters).unwrap_or_default();
    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    let mut output = String::new();
    {
        let mut arr = write_json::array(&mut output);
        indexes.into_iter().for_each(|i| {
            arr.string(&i);
        });
    }

    output
}

#[wasm_bindgen]
pub fn request_all_filter_indexes(ptr: *mut SearchIndex) -> String {
    debug!({ "Finding all filter chunks" });

    let search_index = unsafe { Box::from_raw(ptr) };
    let mut indexes: Vec<String> = search_index
        .filter_chunks
        .iter()
        .map(|(_, chunk)| chunk.into())
        .collect();

    let _ = Box::into_raw(search_index);
    indexes.sort();
    indexes.dedup();
    let mut output = String::new();
    {
        let mut arr = write_json::array(&mut output);
        indexes.into_iter().for_each(|i| {
            arr.string(&i);
        });
    }

    output
}

#[wasm_bindgen]
pub fn filters(ptr: *mut SearchIndex) -> String {
    debug!({ "Returning all loaded filters" });

    let search_index = unsafe { Box::from_raw(ptr) };

    let mut output = String::new();
    {
        let mut obj = write_json::object(&mut output);
        search_index.get_filters(&mut obj, None);
    }

    let _ = Box::into_raw(search_index);
    output
}

#[wasm_bindgen]
pub fn search(
    ptr: *mut SearchIndex,
    query: &str,
    original_query: &str,
    filter: &str,
    sort: &str,
    exact: bool,
    exact_diacritics: bool,
) -> String {
    let search_index = unsafe { Box::from_raw(ptr) };
    let mut output = String::new();
    {
        let mut output_obj = write_json::object(&mut output);

        if let Some(generator_version) = search_index.generator_version.as_ref() {
            if generator_version != search_index.web_version {
                // TODO: Return this as a warning alongside a search result if possible
                // let _ = Box::into_raw(search_index);
                // return "ERROR: Version mismatch".into();
            }
        }

        let filter_set = search_index.filter(filter);
        let (unfiltered_results, mut results, verbose_query_idfs) = if exact {
            let (u, r) =
                search_index.exact_term(query, original_query, filter_set, exact_diacritics);
            (u, r, None)
        } else {
            search_index.search_term(query, original_query, filter_set, exact_diacritics)
        };
        let unfiltered_total = unfiltered_results.len();
        debug!({ format!("Raw total of {} results", unfiltered_total) });
        debug!({ format!("Filtered total of {} results", query.len()) });

        {
            let mut filter_obj = output_obj.object("filtered_counts");
            search_index.get_filters(
                &mut filter_obj,
                Some(results.iter().map(|r| r.page_index).collect()),
            );
        }
        {
            let mut unfilter_obj = output_obj.object("total_counts");
            search_index.get_filters(&mut unfilter_obj, Some(unfiltered_results));
        }

        if let Some((sort, direction)) = sort.split_once(':') {
            debug!({ format!("Trying to sort by {sort} ({direction})") });
            if let Some(sorted_pages) = search_index.sorts.get(sort) {
                debug!({ format!("Found {} pages sorted by {sort}", sorted_pages.len()) });
                results.retain(|result| sorted_pages.contains(&(result.page_index as u32)));

                for result in results.iter_mut() {
                    result.page_score = sorted_pages
                        .iter()
                        .position(|p| p == &(result.page_index as u32))
                        .expect("Sorted pages should contain all remaining results")
                        as f32;
                    if direction == "asc" {
                        result.page_score = 0.0 - result.page_score;
                    }
                }
            }
        }

        {
            debug!({ "Building the result string" });
            let mut arr = output_obj.array("results");

            for result in results {
                let mut page_obj = arr.object();
                page_obj
                    .string("p", &result.page)
                    .string("g", &result.group_hash)
                    .number("s", result.page_score as f64);
                if search_index.playground_mode {
                    let mut params_obj = page_obj.object("params");

                    params_obj
                        .number("tp", search_index.pages.len() as f64)
                        .number("apl", search_index.average_page_length as f64)
                        .number("dl", result.page_length as f64);
                }
                if let Some(verbose_scores) = result.verbose_scores {
                    let mut score_arr = page_obj.array("scores");
                    for (
                        word,
                        ScoringMetrics {
                            idf,
                            bm25_tf,
                            raw_tf,
                            pagefind_tf,
                            score,
                        },
                        params,
                    ) in verbose_scores
                    {
                        let mut score_obj = score_arr.object();

                        score_obj
                            .string("w", &word)
                            .number("idf", idf as f64)
                            .number("b_tf", bm25_tf as f64)
                            .number("r_tf", raw_tf as f64)
                            .number("p_tf", pagefind_tf as f64)
                            .number("s", score as f64);

                        {
                            let mut params_obj = score_obj.object("params");

                            let BM25Params {
                                weighted_term_frequency,
                                document_length: _,
                                average_page_length: _,
                                total_pages: _,
                                pages_containing_term,
                                length_bonus,
                            } = params;

                            params_obj
                                .number("w_tf", weighted_term_frequency as f64)
                                .number("pct", pages_containing_term as f64)
                                .number("lb", length_bonus as f64);
                        }
                    }
                }
                {
                    let mut locs_arr = page_obj.array("l");

                    for BalancedWordScore {
                        weight,
                        balanced_score,
                        word_location,
                        verbose_word_info,
                    } in result.word_locations
                    {
                        let mut locs_obj = locs_arr.object();
                        locs_obj
                            .number("w", weight as f64)
                            .number("s", balanced_score as f64)
                            .number("l", word_location as f64);
                        if let Some(verbose_word_info) = verbose_word_info {
                            locs_obj
                                .object("v")
                                .string("ws", &verbose_word_info.word)
                                .number("lb", verbose_word_info.length_bonus as f64);
                        }
                    }
                }
                // Include matched metadata fields if any
                if !result.matched_meta_fields.is_empty() {
                    let mut mf_arr = page_obj.array("mf");
                    for field_name in result.matched_meta_fields {
                        mf_arr.string(&field_name);
                    }
                }
                // for playground mode
                if let Some(verbose_meta_scores) = result.verbose_meta_scores {
                    let mut vms_arr = page_obj.array("vms");
                    for score in verbose_meta_scores {
                        let mut score_obj = vms_arr.object();
                        score_obj
                            .string("fn", &score.field_name)
                            .number("fw", score.field_weight as f64)
                            .number("mi", score.matched_idf as f64)
                            .number("ti", score.query_total_idf as f64)
                            .number("cv", score.coverage as f64)
                            .number("cb", score.coverage_boost as f64);
                        let mut mt_arr = score_obj.array("mt");
                        for term in score.matched_terms {
                            mt_arr.string(&term);
                        }
                    }
                }
            }
        }

        output_obj.number("unfiltered_total", unfiltered_total as f64);

        if search_index.playground_mode {
            {
                let mut arr = output_obj.array("search_keywords");
                for term in stems_from_term(query) {
                    arr.string(&term);
                }
            }

            if let Some(query_idfs) = verbose_query_idfs {
                let mut qi_arr = output_obj.array("query_term_idfs");
                for query_idf in query_idfs {
                    let mut qi_obj = qi_arr.object();
                    qi_obj
                        .string("t", &query_idf.term)
                        .number("i", query_idf.idf as f64);
                }
            }
        }
    }

    debug!({ "Boxing and returning the result" });
    let _ = Box::into_raw(search_index);

    #[cfg(debug_assertions)]
    debug_log(&format! {"{:?}", output});

    output
}
