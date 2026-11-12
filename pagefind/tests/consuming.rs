//! Consuming finalization must preserve the reusable API's complete output.
use pagefind::{
    api::{PagefindIndex, SyntheticFile},
    options::PagefindServiceConfig,
};
use std::{collections::BTreeMap, path::PathBuf};

async fn add_page(index: &mut PagefindIndex, n: usize, multilingual: bool) {
    let language = if multilingual {
        ["en", "fr", "de", ""][n % 4]
    } else {
        "en"
    };
    index
        .add_html_file(
            None,
            Some(format!("/articles/{n}")),
            format!(
                "<html lang=\"{language}\"><body><main data-pagefind-body>\
         <h1 data-pagefind-meta=\"title\" data-pagefind-weight=\"2\">Café {n}</h1>\
         <h2 id=\"section\">Customers and cafés</h2>\
         <p>Customers order coffee, cafés, naïve pastries and résumé examples.</p>\
         <span data-pagefind-filter=\"category\">group{}</span>\
         <span data-pagefind-filter=\"kind\">article</span>\
         <span data-pagefind-sort=\"ordinal\">{n}</span>\
         <span data-pagefind-sort=\"label\">letter{}</span>\
         <aside data-pagefind-ignore>Excluded passage.</aside>\
         </main></body></html>",
                n % 3,
                n % 5
            ),
        )
        .await
        .unwrap();
}

async fn fixture(pages: usize, multilingual: bool, grouped: bool) -> PagefindIndex {
    let options = if grouped {
        PagefindServiceConfig::builder().max_fragments(3).build()
    } else {
        PagefindServiceConfig::builder().build()
    };
    let mut index = PagefindIndex::new(Some(options)).unwrap();
    for n in 0..pages {
        add_page(&mut index, n, multilingual).await;
    }
    // A custom body elsewhere excludes this ordinary HTML page.
    index
        .add_html_file(
            None,
            Some("/excluded".into()),
            "<html><body>Entire excluded page.</body></html>".into(),
        )
        .await
        .unwrap();
    // Custom records are force-included even without a custom body.
    index
        .add_custom_record(
            "/records/example".into(),
            "A performer serving café customers.".into(),
            "en".into(),
            Some(BTreeMap::from([(
                "title".into(),
                "Example Performer".into(),
            )])),
            Some(BTreeMap::from([
                ("category".into(), vec!["group0".into()]),
                ("kind".into(), vec!["record".into()]),
            ])),
            Some(BTreeMap::from([
                ("ordinal".into(), "10000".into()),
                ("label".into(), "record".into()),
            ])),
        )
        .await
        .unwrap();
    index
}

fn bundle(files: Vec<SyntheticFile>) -> BTreeMap<PathBuf, Vec<u8>> {
    files
        .into_iter()
        .map(|file| (file.filename, file.contents))
        .collect()
}

#[tokio::test]
async fn consuming_finalization_preserves_every_bundle_byte_across_batches() {
    let retained = bundle(fixture(257, false, false).await.get_files().await.unwrap());
    let consumed = bundle(fixture(257, false, false).await.into_files().await.unwrap());
    assert_eq!(retained, consumed);
    assert!(consumed
        .keys()
        .any(|p| p.extension().is_some_and(|e| e == "pf_fragment")));
}

#[tokio::test]
async fn consuming_multilingual_grouped_index_preserves_every_bundle_byte() {
    let retained = bundle(fixture(24, true, true).await.get_files().await.unwrap());
    let consumed = bundle(fixture(24, true, true).await.into_files().await.unwrap());
    assert_eq!(retained, consumed);
}

#[tokio::test]
async fn reusable_index_can_be_rebuilt_and_appended() {
    let mut index = fixture(12, true, false).await;
    let first = bundle(index.get_files().await.unwrap());
    assert_eq!(first, bundle(index.get_files().await.unwrap()));
    add_page(&mut index, 999, true).await;
    let appended = bundle(index.get_files().await.unwrap());
    let mut fresh = fixture(12, true, false).await;
    add_page(&mut fresh, 999, true).await;
    assert_eq!(appended, bundle(fresh.into_files().await.unwrap()));
    assert_ne!(first, appended);
}

#[tokio::test]
async fn consuming_empty_index_preserves_every_bundle_byte() {
    let mut retained = PagefindIndex::new(None).unwrap();
    let consumed = PagefindIndex::new(None).unwrap();
    assert_eq!(
        bundle(retained.get_files().await.unwrap()),
        bundle(consumed.into_files().await.unwrap())
    );
}
