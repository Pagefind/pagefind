use std::{cmp::Ordering, collections::BTreeMap, path::PathBuf};

use anyhow::{bail, Result};
use either::Either;
use fossick::{FossickedData, Fossicker};
use futures::future::join_all;
use index::PagefindIndexes;
use options::{PagefindInboundConfig, SearchOptions};
use output::SyntheticFile;
use rayon::prelude::*;
pub use service::api;
use wax::walk::Entry;
use wax::Glob;

use crate::index::build_indexes;

mod fossick;
mod fragments;
mod index;
#[macro_use]
mod logging;
pub mod options;
mod output;
mod playground;
pub mod runner;
#[cfg(feature = "serve")]
mod serve;
mod service;
mod utils;

const PAGEFIND_VERSION: &str = env!("CARGO_PKG_VERSION");

struct SearchState {
    options: SearchOptions,
    fossicked_pages: Vec<FossickedData>,
    built_indexes: Vec<PagefindIndexes>,
}

impl SearchState {
    pub fn new(options: SearchOptions) -> Self {
        Self {
            options,
            fossicked_pages: vec![],
            built_indexes: vec![],
        }
    }

    pub fn walk_for_files(&self, dir: PathBuf, glob: String) -> Result<Vec<Fossicker>> {
        let log = &self.options.logger;

        log.status("[Walking source directory]");
        if let Ok(glob) = Glob::new(&glob) {
            let mut paths: Vec<PathBuf> = glob
                .walk(&dir)
                .filter_map(Result::ok)
                .map(|e| e.into_path())
                .collect();
            paths.sort_unstable();
            Ok(paths
                .into_iter()
                .map(|file_path| Fossicker::new_relative_to(file_path, dir.clone()))
                .collect())
        } else {
            log.error(format!(
                "Error: Provided glob \"{}\" did not parse as a valid glob.",
                self.options.glob
            ));
            bail!(
                "Error: Provided glob \"{}\" did not parse as a valid glob.",
                self.options.glob
            );
        }
    }

    /// Fossick files in parallel using rayon
    pub fn fossick_many(&mut self, dir: PathBuf, glob: String) -> Result<usize> {
        let files = self.walk_for_files(dir.clone(), glob)?;
        let log = &self.options.logger;

        log.info(format!(
            "Found {} file{} matching {}",
            files.len(),
            plural!(files.len()),
            self.options.glob
        ));
        log.status("[Parsing files]");

        // Use rayon for parallel HTML parsing - this is the main performance win
        // Partition results into successes and failures to report errors
        let (results, errors): (Vec<_>, Vec<_>) = files
            .into_par_iter()
            .map(|f| f.fossick_sync(&self.options))
            .partition_map(|r| match r {
                Ok(data) => Either::Left(data),
                Err(e) => Either::Right(e),
            });

        // Report any errors that occurred during parallel processing
        if !errors.is_empty() {
            log.warn(format!(
                "{} file{} failed to index",
                errors.len(),
                plural!(errors.len())
            ));
            for err in errors.iter().take(5) {
                log.warn(format!("  - {}", err));
            }
            if errors.len() > 5 {
                log.warn(format!("  ... and {} more", errors.len() - 5));
            }
        }

        let existing_page_count = self.fossicked_pages.len();
        self.fossicked_pages.extend(results);

        Ok(self.fossicked_pages.len() - existing_page_count)
    }

    pub fn fossick_one(&mut self, file: Fossicker) -> Result<FossickedData> {
        let result = file.fossick_sync(&self.options);
        if let Some(result) = result.as_ref().ok() {
            let existing = self
                .fossicked_pages
                .iter()
                .position(|page| page.url == result.url);
            if let Some(existing) = existing {
                *self.fossicked_pages.get_mut(existing).unwrap() = result.clone();
            } else {
                self.fossicked_pages.push(result.clone());
            }
        }
        result
    }

    pub async fn build_indexes(&mut self) -> Result<()> {
        let log = &self.options.logger;

        let used_custom_body = self.fossicked_pages.iter().any(|page| page.has_custom_body);
        if used_custom_body {
            log.info("Found a data-pagefind-body element on the site.\n↳ Ignoring pages without this tag.");
        } else {
            log.info(
                "Did not find a data-pagefind-body element on the site.\n↳ Indexing all <body> elements on the site."
            );
        }

        if self.options.root_selector == "html" {
            let pages_without_html = self
                .fossicked_pages
                .iter()
                .filter(|p| !p.has_html_element)
                .map(|p| format!("  * {:?} has no <html> element", p.fragment.data.url))
                .collect::<Vec<_>>();
            if !pages_without_html.is_empty() {
                log.warn(format!(
                    "{} page{} found without an <html> element. \n\
                    Pages without an outer <html> element will not be processed by default. \n\
                    If adding this element is not possible, use the root selector config to target a different root element. \n\
                    Run Pagefind with --verbose for more information.",
                    pages_without_html.len(),
                    plural!(pages_without_html.len())
                ));
                log.v_warn(pages_without_html.join("\n"));
            }
        }

        log.status("[Reading languages]");

        let pages_with_data = self.fossicked_pages.iter().filter(|d| {
            if used_custom_body && !d.has_custom_body && !d.force_inclusion {
                return false;
            }
            !d.word_data.is_empty()
        });

        let mut language_map: BTreeMap<String, Vec<fossick::FossickedData>> = BTreeMap::new();
        for page in pages_with_data {
            let language = page.language.clone();
            if let Some(lang_pages) = language_map.get_mut(&language) {
                lang_pages.push(page.clone());
            } else {
                language_map.insert(language, vec![page.clone()]);
            }
        }

        log.info(format!(
            "Discovered {} language{}: {}",
            language_map.len(),
            plural!(language_map.len()),
            language_map.keys().cloned().collect::<Vec<_>>().join(", ")
        ));
        log.v_info(
            language_map
                .iter()
                .map(|(k, v)| format!("  * {}: {} page{}", k, v.len(), plural!(v.len())))
                .collect::<Vec<_>>()
                .join("\n"),
        );

        let primary_language = language_map
            .iter()
            .filter(|(k, _)| k.as_str() != "unknown")
            .max_by(|(lang_a, pages_a), (lang_b, pages_b)| {
                let size = pages_a.len().cmp(&pages_b.len());
                if matches!(size, Ordering::Equal) {
                    return lang_b.cmp(lang_a);
                }
                size
            })
            .map(|(k, _)| k.clone())
            .unwrap_or_else(|| "unknown".into());

        if let Some(mut unknown_pages) = language_map.remove("unknown") {
            if !language_map.is_empty() {
                log.warn(format!(
                    "{} page{} found without an html lang attribute. \n\
                    Merging these pages with the {} language, as that is the main language on this site. \n\
                    Run Pagefind with --verbose for more information.",
                    unknown_pages.len(),
                    plural!(unknown_pages.len()),
                    primary_language
                ));

                log.v_warn(
                    unknown_pages
                        .iter()
                        .map(|p| {
                            format!("  * {:?} has no html lang attribute", p.fragment.data.url)
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                );

                if let Some(primary) = language_map.get_mut(&primary_language) {
                    primary.append(&mut unknown_pages);
                } else {
                    language_map.insert(primary_language, unknown_pages);
                }
            } else {
                language_map.insert(primary_language, unknown_pages);
            }
        }

        log.status("[Building search indexes]");

        let indexes: Vec<_> = language_map
            .into_iter()
            .map(|(language, pages)| async { build_indexes(pages, language, &self.options).await })
            .collect();
        let built_indexes = join_all(indexes).await;
        self.built_indexes = built_indexes.into_iter().flat_map(|i| i.ok()).collect();

        let stats = self.built_indexes.iter().fold((0, 0, 0, 0), |mut stats, index| {
            log.v_info(format!(
                "Language {}: \n  Indexed {} page{}\n  Indexed {} word{}\n  Indexed {} filter{}\n  Indexed {} sort{}\n",
                index.language,
                index.page_count,
                plural!(index.page_count),
                index.word_count,
                plural!(index.word_count),
                index.filter_indexes.len(),
                plural!(index.filter_indexes.len()),
                index.sorts.len(),
                plural!(index.sorts.len())
            ));

            #[cfg(not(feature = "extended"))]
            match index.language.split('-').next() {
                Some("zh") => log.warn("⚠ Indexing Chinese in non-extended mode. \n\
                                        In this mode, Pagefind will not segment words that are not whitespace separated. \n\
                                        Running the extended Pagefind binary will include this segmentation. \n\
                                        Either download the pagefind_extended binary, or run via npx pagefind."),
                Some("ja") => log.warn("⚠ Indexing Japanese in non-extended mode. \n\
                                        In this mode, Pagefind will not segment words that are not whitespace separated. \n\
                                        Running the extended Pagefind binary will include this segmentation. \n\
                                        Either download the pagefind_extended binary, or run via npx pagefind."),
                _ => {}
            };

            stats.0 += index.page_count;
            stats.1 += index.word_count;
            stats.2 += index.filter_indexes.len();
            stats.3 += index.sorts.len();
            stats
        });

        log.info(format!(
            "Total: \n  Indexed {} language{}\n  Indexed {} page{}\n  Indexed {} word{}\n  Indexed {} filter{}\n  Indexed {} sort{}",
            self.built_indexes.len(),
            plural!(self.built_indexes.len()),
            stats.0,
            plural!(stats.0),
            stats.1,
            plural!(stats.1),
            stats.2,
            plural!(stats.2),
            stats.3,
            plural!(stats.3)
        ));

        if stats.1 == 0 && !self.options.running_as_service {
            log.error(
                "Error: Pagefind was not able to build an index. \n\
                Most likely, the directory passed to Pagefind was empty \
                or did not contain any html files.",
            );
            bail!(
                "Error: Pagefind was not able to build an index. \n\
                Most likely, the directory passed to Pagefind was empty \
                or did not contain any html files."
            );
        }
        Ok(())
    }

    pub async fn write_files(&self, custom_outdir: Option<PathBuf>) -> PathBuf {
        let outdir = custom_outdir.unwrap_or(self.options.bundle_output.clone());

        let index_entries: Vec<_> = self
            .built_indexes
            .iter()
            .map(|indexes| indexes.get_lang_meta(&self.options))
            .collect();

        join_all(
            self.built_indexes
                .iter()
                .map(|indexes| async { indexes.write_files_to_disk(&self.options, &outdir).await }),
        )
        .await;

        output::write_common_to_disk(
            index_entries,
            self.options.write_playground,
            &outdir,
            &self.options,
        )
        .await;

        outdir
    }

    pub async fn get_files(&self) -> Vec<SyntheticFile> {
        let outdir = &self.options.bundle_output;

        let index_entries: Vec<_> = self
            .built_indexes
            .iter()
            .map(|indexes| indexes.get_lang_meta(&self.options))
            .collect();

        let mut files: Vec<_> =
            join_all(self.built_indexes.iter().map(|indexes| async {
                indexes.write_files_to_memory(&self.options, outdir).await
            }))
            .await
            .into_iter()
            .flatten()
            .collect();

        files.extend(
            output::write_common_to_memory(
                index_entries,
                self.options.write_playground,
                outdir,
                &self.options,
            )
            .await
            .into_iter(),
        );

        // SyntheticFiles should only return the relative path to the file
        // _within_ the bundle directory — placing them in a final location
        // is left to the API consumer.
        for file in files.iter_mut() {
            if let Ok(relative_path) = file.filename.strip_prefix(outdir) {
                file.filename = relative_path.to_path_buf();
            }
        }

        files
    }

    pub fn log_start(&self) {
        let log = &self.options.logger;

        #[cfg(not(feature = "extended"))]
        log.status(&format!("Running Pagefind v{}", self.options.version));
        #[cfg(feature = "extended")]
        log.status(&format!(
            "Running Pagefind v{} (Extended)",
            self.options.version
        ));
        log.v_info("Running in verbose mode");

        log.info(format!(
            "Running from: {:?}",
            self.options.working_directory
        ));
        log.info(format!(
            "Source:       {:?}",
            self.options
                .site_source
                .strip_prefix(&self.options.working_directory)
                .unwrap_or(&self.options.site_source)
        ));
        log.info(format!(
            "Output:       {:?}",
            self.options
                .bundle_output
                .strip_prefix(&self.options.working_directory)
                .unwrap_or(&self.options.bundle_output)
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_options() -> SearchOptions {
        let mut config: PagefindInboundConfig =
            serde_json::from_str("{}").expect("all fields have serde defaults");
        config.service = true;
        config.silent = true;
        SearchOptions::load(config).expect("default service options should load")
    }

    #[test]
    fn walk_for_files_returns_paths_in_sorted_order() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "pagefind_walk_test_{}_{}",
            std::process::id(),
            unique
        ));
        std::fs::create_dir_all(root.join("z")).unwrap();
        std::fs::create_dir_all(root.join("a/sub")).unwrap();
        std::fs::create_dir_all(root.join("m")).unwrap();

        // Create files intentionally out of sorted order
        for rel in [
            "z/index.html",
            "m/page.html",
            "a/sub/second.html",
            "a/first.html",
            "index.html",
        ] {
            std::fs::write(
                root.join(rel),
                format!("<html lang=\"en\"><body><p>{rel}</p></body></html>"),
            )
            .unwrap();
        }

        let options = test_options();
        let state = SearchState::new(options.clone());
        let walked = state
            .walk_for_files(root.clone(), "**/*.html".into())
            .expect("walk_for_files should succeed");
        let urls: Vec<String> = walked
            .into_iter()
            .map(|f| f.fossick_sync(&options).unwrap().url)
            .collect();

        let _ = std::fs::remove_dir_all(&root);

        assert_eq!(
            urls,
            vec![
                "/a/first.html",
                "/a/sub/second.html",
                "/",
                "/m/page.html",
                "/z/",
            ]
        );
    }

    #[tokio::test]
    async fn pagefind_index_get_files_is_byte_identical_regardless_of_insertion_order() {
        use crate::api::PagefindIndex;
        use std::collections::BTreeMap;

        async fn build_bundle(reverse: bool) -> Vec<(PathBuf, Vec<u8>)> {
            let mut index = PagefindIndex::new(None).unwrap();

            // Mix explicit languages (en, fr), missing lang ("unknown" -> merged into en),
            // HTML files, custom records, diacritic variants, filters, and sorts.
            enum Entry {
                Html(&'static str, &'static str),
                Custom(&'static str, &'static str, &'static str),
            }

            let mut entries = vec![
                Entry::Html(
                    "/z-en/",
                    r#"<html lang="en"><body><h1 data-pagefind-meta="title">Zeta</h1><p data-pagefind-filter="tag:shared" data-pagefind-sort="n:2">café shared alpha</p></body></html>"#,
                ),
                Entry::Html(
                    "/a-unknown/",
                    r#"<html><body><h1 data-pagefind-meta="title">NoLang</h1><p data-pagefind-filter="tag:shared" data-pagefind-sort="n:1">cafè shared beta</p></body></html>"#,
                ),
                Entry::Html(
                    "/m-en/",
                    r#"<html lang="en"><body><h1 data-pagefind-meta="title">Mid</h1><p data-pagefind-filter="tag:mid" data-pagefind-sort="n:1">cafê shared gamma</p></body></html>"#,
                ),
                Entry::Custom("/b-fr/", "bonjour café monde", "fr"),
                Entry::Custom("/a-fr/", "salut cafè monde", "fr"),
            ];

            if reverse {
                entries.reverse();
            }

            for entry in entries {
                match entry {
                    Entry::Html(url, html) => {
                        index
                            .add_html_file(None, Some(url.to_string()), html.to_string())
                            .await
                            .unwrap();
                    }
                    Entry::Custom(url, content, lang) => {
                        let mut filters = BTreeMap::new();
                        filters.insert("lang_group".to_string(), vec![lang.to_string()]);
                        index
                            .add_custom_record(
                                url.to_string(),
                                content.to_string(),
                                lang.to_string(),
                                None,
                                Some(filters),
                                None,
                            )
                            .await
                            .unwrap();
                    }
                }
            }

            index
                .get_files()
                .await
                .unwrap()
                .into_iter()
                .map(|f| (f.filename, f.contents))
                .collect()
        }

        let files_forward = build_bundle(false).await;
        let files_reversed = build_bundle(true).await;

        assert_eq!(files_forward.len(), files_reversed.len());
        let forward_names: Vec<_> = files_forward.iter().map(|(p, _)| p).collect();
        let reversed_names: Vec<_> = files_reversed.iter().map(|(p, _)| p).collect();
        assert_eq!(
            forward_names, reversed_names,
            "generated file list and hashes must match in exact order"
        );
        assert_eq!(
            files_forward, files_reversed,
            "generated file contents must be byte-for-byte identical"
        );
    }

    #[tokio::test]
    async fn add_directory_produces_identical_bundle_regardless_of_file_creation_order() {
        use crate::api::PagefindIndex;

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir_a = std::env::temp_dir().join(format!(
            "pagefind_dir_det_a_{}_{}",
            std::process::id(),
            unique
        ));
        let dir_b = std::env::temp_dir().join(format!(
            "pagefind_dir_det_b_{}_{}",
            std::process::id(),
            unique
        ));

        let files = [
            (
                "z/index.html",
                r#"<html lang="en"><body><h1>Zeta</h1><p data-pagefind-filter="kind:post" data-pagefind-sort="order:2">café shared</p></body></html>"#,
            ),
            (
                "a/sub/index.html",
                r#"<html lang="en"><body><h1>Alpha</h1><p data-pagefind-filter="kind:post" data-pagefind-sort="order:1">cafè shared</p></body></html>"#,
            ),
            (
                "m/page.html",
                r#"<html><body><h1>Unknown Lang</h1><p data-pagefind-filter="kind:note" data-pagefind-sort="order:1">cafê shared</p></body></html>"#,
            ),
            (
                "fr/bonjour.html",
                r#"<html lang="fr"><body><h1>Bonjour</h1><p>café français</p></body></html>"#,
            ),
        ];

        let write_site = |root: &PathBuf, reverse: bool| {
            let iter: Vec<_> = if reverse {
                files.iter().rev().collect()
            } else {
                files.iter().collect()
            };
            for (rel, content) in iter {
                let path = root.join(rel);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, content).unwrap();
            }
        };

        write_site(&dir_a, false);
        write_site(&dir_b, true);

        let mut idx_a = PagefindIndex::new(None).unwrap();
        idx_a
            .add_directory(dir_a.to_string_lossy().into_owned(), None)
            .await
            .unwrap();
        let bundle_a: Vec<(PathBuf, Vec<u8>)> = idx_a
            .get_files()
            .await
            .unwrap()
            .into_iter()
            .map(|f| (f.filename, f.contents))
            .collect();

        let mut idx_b = PagefindIndex::new(None).unwrap();
        idx_b
            .add_directory(dir_b.to_string_lossy().into_owned(), None)
            .await
            .unwrap();
        let bundle_b: Vec<(PathBuf, Vec<u8>)> = idx_b
            .get_files()
            .await
            .unwrap()
            .into_iter()
            .map(|f| (f.filename, f.contents))
            .collect();

        let _ = std::fs::remove_dir_all(&dir_a);
        let _ = std::fs::remove_dir_all(&dir_b);

        assert_eq!(bundle_a, bundle_b);
    }
}
