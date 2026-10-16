use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::index::PagefindIndexes;
use crate::{SearchOptions, PAGEFIND_VERSION};
use flate2::write::GzEncoder; // TODO: Replace flate2 with async-compression since we
use flate2::Compression; //   // require that crate for the input compression anyway.
use futures::{stream, StreamExt};
use include_dir::{include_dir, Dir};
use minifier::js::minify;
use std::collections::BTreeMap;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

mod entry;

const CREATE_ATTEMPTS: usize = 50;

fn write_concurrency(options: &SearchOptions) -> usize {
    #[cfg(test)]
    if let Ok(limit) = tests::WRITE_TRACKER.try_with(|tracker| tracker.limit) {
        return limit;
    }

    options.write_concurrency
}

const GENERIC_WEB_WASM: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/wasm/pagefind_web_bg.unknown.",
    env!("CARGO_PKG_VERSION"),
    ".wasm.gz"
));
const WEB_WASM_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/vendor/wasm");

const WEB_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_web.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_UI_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_ui.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_UI_CSS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_ui.",
    env!("CARGO_PKG_VERSION"),
    ".css"
));
const WEB_MODULAR_UI_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_modular_ui.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WEB_MODULAR_UI_CSS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_modular_ui.",
    env!("CARGO_PKG_VERSION"),
    ".css"
));
const COMPONENT_UI_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_component_ui.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const COMPONENT_UI_CSS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_component_ui.",
    env!("CARGO_PKG_VERSION"),
    ".css"
));
const SEARCH_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_public_search_api.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const HIGHLIGHT_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_highlight.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));
const WORKER_JS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/pagefind_worker.",
    env!("CARGO_PKG_VERSION"),
    ".js"
));

pub struct LanguageMeta {
    pub page_count: usize,
    pub language: String,
    pub hash: String,
    pub wasm: Option<String>,
}

pub async fn write_common_to_disk(
    language_indexes: Vec<LanguageMeta>,
    output_playground: bool,
    outdir: &PathBuf,
    options: &SearchOptions,
) {
    write_common(language_indexes, output_playground, outdir, options, false).await;
}

pub async fn write_common_to_memory(
    language_indexes: Vec<LanguageMeta>,
    output_playground: bool,
    outdir: &PathBuf,
    options: &SearchOptions,
) -> Vec<SyntheticFile> {
    write_common(language_indexes, output_playground, outdir, options, true)
        .await
        .unwrap()
}

async fn write_common(
    language_indexes: Vec<LanguageMeta>,
    output_playground: bool,
    outdir: &PathBuf,
    options: &SearchOptions,
    synthetic: bool,
) -> Option<Vec<SyntheticFile>> {
    let js_version = format!("const pagefind_version = \"{PAGEFIND_VERSION}\";");
    let mut js = vec![];
    minify(&format!("{js_version}\n{WEB_JS}\n{SEARCH_JS}"))
        .expect("Minifying Pagefind JS failed")
        .write(&mut js)
        .expect("Writing minified Pagefind JS failed");

    let mut worker_js = vec![];
    minify(&format!(
        "{js_version}\n{WEB_JS}\n{}",
        String::from_utf8_lossy(WORKER_JS)
    ))
    .expect("Minifying Pagefind Worker JS failed")
    .write(&mut worker_js)
    .expect("Writing minified Pagefind Worker JS failed");

    let entry_meta = entry::PagefindEntryMeta {
        version: PAGEFIND_VERSION,
        languages: BTreeMap::from_iter(language_indexes.into_iter().map(|i| {
            (
                i.language,
                entry::PagefindEntryLanguage {
                    hash: i.hash,
                    wasm: i.wasm,
                    page_count: i.page_count,
                },
            )
        })),
        include_characters: options.include_characters.clone(),
    };
    let encoded_entry_meta = serde_json::to_string(&entry_meta).unwrap();

    let write_behavior = if synthetic {
        WriteBehavior::Synthetic
    } else {
        WriteBehavior::Disk
    };

    let mut files = vec![
        write(
            outdir.join("pagefind.js"),
            vec![&js],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-highlight.js"),
            vec![HIGHLIGHT_JS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-worker.js"),
            vec![&worker_js],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-ui.js"),
            vec![WEB_UI_JS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-ui.css"),
            vec![WEB_UI_CSS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-modular-ui.js"),
            vec![WEB_MODULAR_UI_JS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-modular-ui.css"),
            vec![WEB_MODULAR_UI_CSS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-component-ui.js"),
            vec![COMPONENT_UI_JS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-component-ui.css"),
            vec![COMPONENT_UI_CSS],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("wasm.unknown.pagefind"),
            vec![GENERIC_WEB_WASM],
            Compress::None,
            write_behavior,
        ),
        write(
            outdir.join("pagefind-entry.json"),
            vec![encoded_entry_meta.as_bytes()],
            Compress::None,
            write_behavior,
        ),
    ];

    if output_playground {
        files.extend([
            write(
                outdir.join("playground/index.html"),
                vec![crate::playground::PLAYGROUND_HTML.as_bytes()],
                Compress::None,
                write_behavior,
            ),
            write(
                outdir.join("playground/pagefind-playground.js"),
                vec![crate::playground::PLAYGROUND_JS.as_bytes()],
                Compress::None,
                write_behavior,
            ),
            write(
                outdir.join("playground/pagefind-playground.css"),
                vec![crate::playground::PLAYGROUND_CSS.as_bytes()],
                Compress::None,
                write_behavior,
            ),
        ]);
    }

    let output_files = stream::iter(files)
        .buffered(write_concurrency(options))
        .collect::<Vec<_>>()
        .await;

    if synthetic {
        Some(output_files.into_iter().flatten().collect())
    } else {
        None
    }
}

impl PagefindIndexes {
    fn lang_wasm_path(&self) -> Option<String> {
        let base_language = self.language.split('-').next().unwrap();
        let wasm_path = format!(
            "pagefind_web_bg.{}.{}.wasm.gz",
            base_language,
            env!("CARGO_PKG_VERSION")
        );

        if WEB_WASM_FILES.contains(&wasm_path) {
            Some(wasm_path)
        } else {
            None
        }
    }

    pub fn get_lang_meta(&self, options: &SearchOptions) -> LanguageMeta {
        let mut wasm_file = None;

        if self.language != "unknown" {
            if self.lang_wasm_path().is_some() {
                wasm_file = Some(self.language.to_string());
            } else {
                options.logger.v_warn(format!(
                    "Note: Pagefind doesn't support stemming for the language {}. \n\
                    Search will still work, but will not match across root words.",
                    self.language
                ));
            }
        }

        LanguageMeta {
            page_count: self.fragments.len(),
            language: self.language.clone(),
            hash: self.meta_index.0.clone(),
            wasm: wasm_file,
        }
    }

    pub async fn write_files_to_disk(&self, options: &SearchOptions, outdir: &PathBuf) {
        self.write_files(options, outdir, false).await;
    }

    pub async fn write_files_to_memory(
        &self,
        options: &SearchOptions,
        outdir: &PathBuf,
    ) -> Vec<SyntheticFile> {
        self.write_files(options, outdir, true).await.unwrap()
    }

    async fn write_files(
        &self,
        options: &SearchOptions,
        outdir: &PathBuf,
        synthetic: bool,
    ) -> Option<Vec<SyntheticFile>> {
        let immutable_write_behaviour = if synthetic {
            WriteBehavior::Synthetic
        } else {
            WriteBehavior::Immutable
        };

        let mut files = vec![write(
            outdir.join(format!("pagefind.{}.pf_meta", &self.meta_index.0)),
            vec![&self.meta_index.1],
            Compress::GZ,
            immutable_write_behaviour,
        )];

        if self.language != "unknown" {
            if let Some(wasm_path) = self.lang_wasm_path() {
                files.push(write(
                    outdir.join(format!("wasm.{}.pagefind", self.language)),
                    vec![WEB_WASM_FILES
                        .get_file(wasm_path)
                        .expect("WASM should exist")
                        .contents()],
                    Compress::None,
                    if synthetic {
                        WriteBehavior::Synthetic
                    } else {
                        WriteBehavior::Disk
                    },
                ));
            } else {
                options.logger.v_warn(format!(
                    "Note: Pagefind doesn't support stemming for the language {}. \n\
                    Search will still work, but will not match across root words.",
                    self.language
                ));
            }
        }

        files.extend(self.fragments.iter().map(|(hash, fragment)| {
            write(
                outdir.join(format!("fragment/{}.pf_fragment", hash)),
                vec![fragment.as_bytes()],
                Compress::GZ,
                immutable_write_behaviour,
            )
        }));

        files.extend(self.word_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("index/{}.pf_index", hash)),
                vec![index],
                Compress::GZ,
                immutable_write_behaviour,
            )
        }));

        files.extend(self.filter_indexes.iter().map(|(hash, index)| {
            write(
                outdir.join(format!("filter/{}.pf_filter", hash)),
                vec![index],
                Compress::GZ,
                immutable_write_behaviour,
            )
        }));

        let output_files = stream::iter(files)
            .buffered(write_concurrency(options))
            .collect::<Vec<_>>()
            .await;
        if synthetic {
            Some(output_files.into_iter().flatten().collect())
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
enum Compress {
    GZ,
    None,
}

#[derive(Copy, Clone)]
enum WriteBehavior {
    Synthetic,
    Immutable,
    Disk,
}

#[derive(Clone)]
pub struct SyntheticFile {
    pub filename: PathBuf,
    pub contents: Vec<u8>,
}

async fn write(
    filename: PathBuf,
    content_chunks: Vec<&[u8]>,
    compression: Compress,
    write_behavior: WriteBehavior,
) -> Option<SyntheticFile> {
    #[cfg(test)]
    let _in_flight = tests::track_write().await;

    // For "immutable" (hashed) files, don't re-write them as the contents _should_ be unchanged.
    if matches!(write_behavior, WriteBehavior::Immutable) && filename.exists() {
        return None;
    }

    let contents = match compression {
        Compress::GZ => {
            let mut gz = GzEncoder::new(Vec::new(), Compression::best());
            for chunk in content_chunks {
                gz.write_all(b"pagefind_dcd").unwrap();
                gz.write_all(chunk).unwrap();
            }
            gz.finish().unwrap()
        }
        Compress::None => content_chunks.into_iter().flatten().cloned().collect(),
    };

    if matches!(write_behavior, WriteBehavior::Synthetic) {
        return Some(SyntheticFile { filename, contents });
    }

    if let Some(parent) = filename.parent() {
        create_dir_all(parent).await.unwrap();
    }

    let mut output_file = create_file(&filename).await;
    for _ in 1..CREATE_ATTEMPTS {
        if !matches!(&output_file, Err(error) if transient_create_error(error)) {
            break;
        }
        sleep(Duration::from_millis(100)).await;
        output_file = create_file(&filename).await;
    }
    let mut file = output_file.unwrap_or_else(|error| {
        if !transient_create_error(&error) {
            panic!("Failed to create {}: {error}", filename.display());
        }
        panic!(
            "Failed to create {} after {CREATE_ATTEMPTS} attempts: {error}",
            filename.display()
        )
    });
    file.write_all(&contents).await.unwrap();
    // `write_all` resolving only means the bytes are in tokio's
    // buffer; the blocking-pool write to disk may still be in
    // flight. Flush before the file drops, or a service-mode
    // consumer can receive the WriteFiles acknowledgement and
    // close the process while a file is half written, shipping
    // a truncated asset.
    file.flush().await.unwrap();
    None
}

fn transient_create_error(error: &std::io::Error) -> bool {
    if matches!(
        error.kind(),
        std::io::ErrorKind::Interrupted | std::io::ErrorKind::WouldBlock
    ) {
        return true;
    }
    #[cfg(unix)]
    // ENFILE, EMFILE
    if matches!(error.raw_os_error(), Some(23 | 24)) {
        return true;
    }
    #[cfg(windows)]
    // ERROR_TOO_MANY_OPEN_FILES, ERROR_SHARING_VIOLATION, ERROR_LOCK_VIOLATION (e.g. antivirus scanners).
    if matches!(error.raw_os_error(), Some(4 | 32 | 33)) {
        return true;
    }
    false
}

async fn create_file(filename: &Path) -> std::io::Result<File> {
    #[cfg(test)]
    if let Some(error) = tests::create_error() {
        return Err(error);
    }
    File::create(filename).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashMap;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    tokio::task_local! {
        pub(super) static WRITE_TRACKER: Arc<WriteTracker>;
        static CREATE_TRACKER: Arc<CreateTracker>;
    }

    pub(super) struct WriteTracker {
        active: AtomicUsize,
        maximum: AtomicUsize,
        entered: AtomicUsize,
        pub(super) limit: usize,
    }

    impl WriteTracker {
        fn new(limit: usize) -> Arc<Self> {
            Arc::new(Self {
                active: AtomicUsize::new(0),
                maximum: AtomicUsize::new(0),
                entered: AtomicUsize::new(0),
                limit,
            })
        }

        fn check(&self, expected: usize) {
            assert_eq!(self.entered.load(Ordering::SeqCst), expected);
            assert_eq!(self.active.load(Ordering::SeqCst), 0);
            let maximum = self.maximum.load(Ordering::SeqCst);
            assert_eq!(maximum, self.limit.min(expected));
        }
    }

    pub(super) struct InFlightWrite(Arc<WriteTracker>);

    impl Drop for InFlightWrite {
        fn drop(&mut self) {
            self.0.active.fetch_sub(1, Ordering::SeqCst);
        }
    }

    pub(super) async fn track_write() -> Option<InFlightWrite> {
        let tracked = WRITE_TRACKER
            .try_with(|tracker| {
                let active = tracker.active.fetch_add(1, Ordering::SeqCst) + 1;
                let position = tracker.entered.fetch_add(1, Ordering::SeqCst);
                tracker.maximum.fetch_max(active, Ordering::SeqCst);
                // Fail fast if the limit is exceeded, before any file is opened.
                assert!(
                    active <= tracker.limit,
                    "write concurrency {active} exceeded limit {} before opening files",
                    tracker.limit
                );
                // Delay the first file so completion order differs from input order.
                let yields = if position == 0 { 8 } else { 1 };
                (InFlightWrite(tracker.clone()), yields)
            })
            .ok();
        if let Some((guard, yields)) = tracked {
            for _ in 0..yields {
                tokio::task::yield_now().await;
            }
            Some(guard)
        } else {
            None
        }
    }

    struct CreateTracker {
        attempts: AtomicUsize,
        failures: usize,
        error: fn() -> std::io::Error,
    }

    impl CreateTracker {
        fn new(failures: usize, error: fn() -> std::io::Error) -> Arc<Self> {
            Arc::new(Self {
                attempts: AtomicUsize::new(0),
                failures,
                error,
            })
        }
    }

    pub(super) fn create_error() -> Option<std::io::Error> {
        CREATE_TRACKER
            .try_with(|tracker| {
                let attempt = tracker.attempts.fetch_add(1, Ordering::SeqCst);
                (attempt < tracker.failures).then(|| (tracker.error)())
            })
            .ok()
            .flatten()
    }

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "pagefind-output-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::SeqCst)
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn options() -> SearchOptions {
        SearchOptions::load(
            serde_json::from_value(serde_json::json!({
                "site": ".", "silent": true, "write_playground": true
            }))
            .unwrap(),
        )
        .unwrap()
    }

    fn indexes(language: &str, fragments: usize) -> PagefindIndexes {
        PagefindIndexes {
            word_indexes: HashMap::new(),
            filter_indexes: HashMap::new(),
            meta_index: (format!("meta-{language}"), vec![1, 2, 3]),
            fragments: (0..fragments)
                .map(|i| (format!("{language}-{i:04}"), format!("fragment {i}")))
                .collect(),
            sorts: vec![],
            language: language.into(),
            word_count: 0,
        }
    }

    #[tokio::test]
    async fn writes_are_bounded() {
        let directory = TempDir::new();
        let index = indexes("unknown", 2048);
        let tracker = WriteTracker::new(4);
        WRITE_TRACKER
            .scope(
                tracker.clone(),
                index.write_files_to_disk(&options(), &directory.0),
            )
            .await;
        tracker.check(2049);
        assert_eq!(
            std::fs::read_dir(directory.0.join("fragment"))
                .unwrap()
                .count(),
            2048
        );
        assert!(directory.0.join("pagefind.meta-unknown.pf_meta").is_file());
    }

    #[tokio::test]
    async fn language_writes_share_the_bound() {
        let directory = TempDir::new();
        let state = crate::SearchState {
            options: options(),
            fossicked_pages: vec![],
            built_indexes: vec![indexes("en", 64), indexes("fr", 64)],
        };
        let tracker = WriteTracker::new(4);
        WRITE_TRACKER
            .scope(
                tracker.clone(),
                state.write_files(Some(directory.0.clone())),
            )
            .await;
        tracker.check(146); // Two languages, each with 64 fragments, metadata and WASM; 14 common files.
    }

    #[tokio::test]
    async fn synthetic_file_order_is_preserved() {
        let directory = TempDir::new();
        let mut index = indexes("unknown", 4);
        index.word_indexes.insert("words".into(), vec![4, 5]);
        index.filter_indexes.insert("filter".into(), vec![6, 7]);
        let tracker = WriteTracker::new(4);
        let files = WRITE_TRACKER
            .scope(
                tracker.clone(),
                index.write_files_to_memory(&options(), &directory.0),
            )
            .await;
        tracker.check(7);
        let expected = [
            "pagefind.meta-unknown.pf_meta",
            "fragment/unknown-0000.pf_fragment",
            "fragment/unknown-0001.pf_fragment",
            "fragment/unknown-0002.pf_fragment",
            "fragment/unknown-0003.pf_fragment",
            "index/words.pf_index",
            "filter/filter.pf_filter",
        ];
        assert_eq!(
            files
                .iter()
                .map(|file| file.filename.clone())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|name| directory.0.join(name))
                .collect::<Vec<_>>()
        );
        let tracker = WriteTracker::new(4);
        let common = WRITE_TRACKER
            .scope(
                tracker.clone(),
                write_common_to_memory(vec![], true, &directory.0, &options()),
            )
            .await;
        tracker.check(14);
        let expected = [
            "pagefind.js",
            "pagefind-highlight.js",
            "pagefind-worker.js",
            "pagefind-ui.js",
            "pagefind-ui.css",
            "pagefind-modular-ui.js",
            "pagefind-modular-ui.css",
            "pagefind-component-ui.js",
            "pagefind-component-ui.css",
            "wasm.unknown.pagefind",
            "pagefind-entry.json",
            "playground/index.html",
            "playground/pagefind-playground.js",
            "playground/pagefind-playground.css",
        ];
        assert_eq!(
            common
                .iter()
                .map(|file| file.filename.clone())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|name| directory.0.join(name))
                .collect::<Vec<_>>()
        );
        assert_eq!(std::fs::read_dir(&directory.0).unwrap().count(), 0);
    }

    #[tokio::test]
    async fn writes_flush_complete_contents_and_skip_immutable_files() {
        use std::io::Read;

        {
            let directory = TempDir::new();
            for (name, compression, decoded) in [
                (
                    "compressed",
                    Compress::GZ,
                    b"pagefind_dcdfirstpagefind_dcdsecond".as_slice(),
                ),
                ("plain", Compress::None, b"firstsecond".as_slice()),
            ] {
                let filename = directory.0.join(name);
                let synthetic = write(
                    filename.clone(),
                    vec![b"first", b"second"],
                    compression,
                    WriteBehavior::Synthetic,
                )
                .await
                .unwrap();
                assert!(write(
                    filename.clone(),
                    vec![b"first", b"second"],
                    compression,
                    WriteBehavior::Disk,
                )
                .await
                .is_none());
                let contents = std::fs::read(&filename).unwrap();
                assert_eq!(contents, synthetic.contents);
                let decoded_contents = match compression {
                    Compress::GZ => {
                        let mut data = Vec::new();
                        flate2::read::GzDecoder::new(contents.as_slice())
                            .read_to_end(&mut data)
                            .unwrap();
                        data
                    }
                    Compress::None => contents.clone(),
                };
                assert_eq!(decoded_contents, decoded);
                assert!(write(
                    filename.clone(),
                    vec![b"replacement"],
                    compression,
                    WriteBehavior::Immutable,
                )
                .await
                .is_none());
                assert_eq!(std::fs::read(filename).unwrap(), contents);
            }
        }
    }

    #[tokio::test]
    async fn create_failure_reports_path_and_error_after_bounded_retries() {
        use futures::FutureExt;

        let directory = TempDir::new();
        let filename = directory.0.join("exhausted");
        let expected_error = std::io::Error::from(std::io::ErrorKind::WouldBlock).to_string();
        let tracker = CreateTracker::new(usize::MAX, || {
            std::io::Error::from(std::io::ErrorKind::WouldBlock)
        });
        let started = std::time::Instant::now();
        let result = tokio::time::timeout(
            Duration::from_secs(15),
            std::panic::AssertUnwindSafe(CREATE_TRACKER.scope(
                tracker.clone(),
                write(
                    filename.clone(),
                    vec![b"contents"],
                    Compress::None,
                    WriteBehavior::Disk,
                ),
            ))
            .catch_unwind(),
        )
        .await
        .expect("create retries did not stop within 15 seconds");
        assert_eq!(tracker.attempts.load(Ordering::SeqCst), 50);
        assert!(
            started.elapsed() >= Duration::from_millis(4900),
            "create retries skipped the 49 x 100 ms delay"
        );
        let Err(panic) = result else {
            panic!("persistent transient errors should exhaust the create budget");
        };
        let message = panic.downcast_ref::<String>().unwrap();
        assert!(
            message.contains(&filename.display().to_string()),
            "{message}"
        );
        assert!(message.contains("after 50 attempts"), "{message}");
        assert!(message.contains(&expected_error), "{message}");
    }

    #[test]
    fn only_transient_create_errors_are_retryable() {
        use std::io::{Error, ErrorKind};

        #[cfg(unix)]
        for code in [23, 24] {
            assert!(transient_create_error(&Error::from_raw_os_error(code)));
        }
        #[cfg(windows)]
        {
            for code in [4, 32, 33] {
                assert!(
                    transient_create_error(&Error::from_raw_os_error(code)),
                    "{code}"
                );
            }
            for code in [2, 3, 5, 23, 24, 87] {
                assert!(
                    !transient_create_error(&Error::from_raw_os_error(code)),
                    "{code}"
                );
            }
        }
        for kind in [ErrorKind::Interrupted, ErrorKind::WouldBlock] {
            assert!(transient_create_error(&Error::from(kind)));
        }
        for kind in [
            ErrorKind::PermissionDenied,
            ErrorKind::AlreadyExists,
            ErrorKind::InvalidInput,
        ] {
            assert!(!transient_create_error(&Error::from(kind)), "{kind:?}");
        }
    }

    #[test]
    fn default_write_concurrency_is_32() {
        if std::env::var_os("PAGEFIND_UNSTABLE_WRITE_CONCURRENCY").is_some() {
            return;
        }
        assert_eq!(write_concurrency(&options()), 32);
    }

    #[tokio::test]
    async fn transient_create_errors_retry_and_recover() {
        use std::io::{Error, ErrorKind};

        let directory = TempDir::new();
        let errors: &[fn() -> Error] = &[
            #[cfg(unix)]
            || Error::from_raw_os_error(24),
            #[cfg(unix)]
            || Error::from_raw_os_error(23),
            #[cfg(windows)]
            || Error::from_raw_os_error(4),
            #[cfg(windows)]
            || Error::from_raw_os_error(32),
            #[cfg(windows)]
            || Error::from_raw_os_error(33),
            || Error::from(ErrorKind::Interrupted),
            || Error::from(ErrorKind::WouldBlock),
        ];
        for (index, error) in errors.iter().copied().enumerate() {
            let filename = directory.0.join(format!("recovered-{index}"));
            let tracker = CreateTracker::new(2, error);
            CREATE_TRACKER
                .scope(
                    tracker.clone(),
                    write(
                        filename.clone(),
                        vec![b"recovered"],
                        Compress::None,
                        WriteBehavior::Disk,
                    ),
                )
                .await;
            assert_eq!(tracker.attempts.load(Ordering::SeqCst), 3);
            assert_eq!(std::fs::read(filename).unwrap(), b"recovered");
        }
    }

    #[tokio::test]
    async fn permanent_create_error_fails_without_retrying() {
        use futures::FutureExt;

        let directory = TempDir::new();
        let filename = directory.0.join("denied");
        let expected_error = std::io::Error::from(std::io::ErrorKind::PermissionDenied).to_string();
        // A generic permission error has no transient platform-specific raw code.
        let tracker = CreateTracker::new(usize::MAX, || {
            std::io::Error::from(std::io::ErrorKind::PermissionDenied)
        });
        let started = std::time::Instant::now();
        let result = tokio::time::timeout(
            Duration::from_secs(2),
            std::panic::AssertUnwindSafe(CREATE_TRACKER.scope(
                tracker.clone(),
                write(
                    filename.clone(),
                    vec![b"contents"],
                    Compress::None,
                    WriteBehavior::Disk,
                ),
            ))
            .catch_unwind(),
        )
        .await
        .expect("permanent create failure slept instead of failing promptly");
        assert_eq!(tracker.attempts.load(Ordering::SeqCst), 1);
        assert!(started.elapsed() < Duration::from_secs(2));
        let Err(panic) = result else {
            panic!("permanent permission errors should fail");
        };
        let message = panic.downcast_ref::<String>().unwrap();
        assert!(
            message.contains(&filename.display().to_string()),
            "{message}"
        );
        assert!(message.contains(&expected_error), "{message}");
        assert!(!message.contains("attempts"), "{message}");
    }
}
