use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PagefindEntryMeta {
    pub version: &'static str,
    pub languages: BTreeMap<String, PagefindEntryLanguage>,
    pub include_characters: Vec<char>,
}

#[derive(Serialize, Debug)]
pub struct PagefindEntryLanguage {
    pub hash: String,
    pub wasm: Option<String>,
    pub page_count: usize,
}
