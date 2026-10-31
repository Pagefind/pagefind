use convert_case::{Case, Casing};
use emojis;
use hashbrown::HashSet;
use lazy_static::lazy_static;
use pagefind_stem::Stemmer;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

const MAX_KATAKANA_SUFFIX_CHARS: usize = 16;
const MAX_DOTTED_TAIL_CHARS: usize = 64;

lazy_static! {
    static ref EMOJI: Regex = Regex::new("\\p{Emoji}").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexableWord {
    pub stemmed: String,
    pub is_compound_part: bool,
    /// The original word before diacritic normalization (if different)
    pub original: Option<String>,
}

/// Returns all words that should be indexed for a given input word.
/// This includes:
/// - The primary normalized+stemmed word
/// - Compound word parts (for hyphenated/camelCase/etc words)
/// - Emoji characters
pub fn get_indexable_words(
    raw_word: &str,
    stemmer: Option<&Stemmer>,
    include_characters: &[char],
) -> Vec<IndexableWord> {
    let mut results = Vec::new();

    let normalized_word = normalize_word(raw_word, include_characters);

    if !normalized_word.is_empty() {
        results.push(stem_word(&normalized_word, stemmer, false));
    }

    for suffix in katakana_suffixes(raw_word) {
        let suffix_diacritic = normalize_diacritics(suffix);
        results.push(IndexableWord {
            stemmed: suffix_diacritic
                .clone()
                .unwrap_or_else(|| suffix.to_string()),
            original: suffix_diacritic.map(|_| suffix.to_string()),
            is_compound_part: true,
        });
    }

    let possibly_compound = raw_word.chars().any(|c| !c.is_alphanumeric())
        || raw_word.chars().skip(1).any(|c| c.is_uppercase());

    if !possibly_compound {
        return results;
    }

    let (word_parts, extras) = get_discrete_words(raw_word);
    if !normalized_word.is_empty()
        && (word_parts.contains(|c: char| c.is_whitespace())
            || !normalized_word.starts_with(&word_parts))
    {
        let part_words: Vec<_> = word_parts.split_whitespace().collect();

        // Only index two+ character words
        for part_word in part_words.into_iter().filter(|w| w.len() > 1) {
            let part_diacritic = normalize_diacritics(part_word);

            let stemmed_part = if let Some(stemmer) = stemmer {
                stemmer
                    .stem(part_diacritic.as_deref().unwrap_or(part_word))
                    .into_owned()
            } else {
                part_diacritic
                    .clone()
                    .unwrap_or_else(|| part_word.to_string())
            };

            results.push(IndexableWord {
                stemmed: stemmed_part,
                original: part_diacritic.map(|_| part_word.to_string()),
                is_compound_part: true,
            });
        }
    }

    if let Some(extras) = extras {
        for extra in extras {
            results.push(IndexableWord {
                stemmed: extra,
                original: None,
                is_compound_part: false,
            });
        }
    }

    results
}

/// Pulling out a useful compound word from a dot-segmented word,
/// so that we index the `is_python_build` in `sysconfig.is_python_build`
/// without completely decomposing it.
pub fn get_dotted_tails(
    raw_word: &str,
    stemmer: Option<&Stemmer>,
    include_characters: &[char],
    existing: &[IndexableWord],
) -> Vec<IndexableWord> {
    let mut tails = Vec::new();
    if !raw_word.contains('.') {
        return tails;
    }
    let seen: HashSet<&str> = existing.iter().map(|w| w.stemmed.as_str()).collect();
    let mut seen_tails: HashSet<String> = HashSet::new();
    for (i, _) in raw_word.match_indices('.') {
        let rest = &raw_word[i + 1..];
        let tail = match rest.char_indices().nth(MAX_DOTTED_TAIL_CHARS) {
            Some((end, _)) => &rest[..end],
            None => rest,
        };
        let tail_is_compound = tail.chars().any(|c| !c.is_alphanumeric())
            || tail.chars().skip(1).any(|c| c.is_uppercase());
        if !tail_is_compound {
            // A plain tail is already indexed, as the final compound part.
            continue;
        }
        let tail_normalized = normalize_word(tail, include_characters);
        if tail_normalized.chars().count() < 2 {
            continue;
        }
        let tail_word = stem_word(&tail_normalized, stemmer, true);
        if !seen.contains(tail_word.stemmed.as_str())
            && seen_tails.insert(tail_word.stemmed.clone())
        {
            tails.push(tail_word);
        }
    }
    tails
}

/// Returns the words to index for the pairs of adjacent grapheme clusters of a
/// segment, for pages in a language that is segmented into words.
pub fn get_indexable_bigrams(
    raw_word: &str,
    stemmer: Option<&Stemmer>,
    existing: &[IndexableWord],
) -> Vec<IndexableWord> {
    let mut seen: HashSet<String> = existing.iter().map(|w| w.stemmed.clone()).collect();
    let mut bigrams = Vec::new();
    for bigram in pair_bigrams(raw_word, None) {
        let bigram = stem_word(&bigram, stemmer, true);
        if seen.insert(bigram.stemmed.clone()) {
            bigrams.push(bigram);
        }
    }
    bigrams
}

/// Returns the word to index for the pair of grapheme clusters spanning the
/// boundary between `previous_cluster` and the start of `raw_word`, if that pair
/// isn't already indexable.
pub fn get_indexable_cross_bigram(
    raw_word: &str,
    previous_cluster: Option<&str>,
    stemmer: Option<&Stemmer>,
    existing: &[IndexableWord],
) -> Option<IndexableWord> {
    let bigram = stem_word(&cross_pair(raw_word, previous_cluster)?, stemmer, true);
    if existing.iter().any(|w| w.stemmed == bigram.stemmed) {
        return None;
    }
    Some(bigram)
}

fn normalize_word(raw_word: &str, include_characters: &[char]) -> String {
    let mut normalized_word = String::with_capacity(raw_word.len());
    for mut c in raw_word.chars() {
        let is_alpha = c.is_alphanumeric();
        if is_alpha || include_characters.contains(&c) {
            c.make_ascii_lowercase();
            if c.is_uppercase() {
                // Non-ascii uppercase can lower to multiple chars
                normalized_word.extend(c.to_lowercase());
            } else {
                normalized_word.push(c);
            }
        }
    }
    normalized_word
}

fn stem_word(
    normalized_word: &str,
    stemmer: Option<&Stemmer>,
    is_compound_part: bool,
) -> IndexableWord {
    let diacritic_normalized = normalize_diacritics(normalized_word);

    let stemmed = if let Some(stemmer) = stemmer {
        stemmer
            .stem(diacritic_normalized.as_deref().unwrap_or(normalized_word))
            .into_owned()
    } else {
        diacritic_normalized
            .clone()
            .unwrap_or_else(|| normalized_word.to_string())
    };

    IndexableWord {
        stemmed,
        original: diacritic_normalized.map(|_| normalized_word.to_string()),
        is_compound_part,
    }
}

fn is_segmented_cluster(cluster: &str) -> bool {
    matches!(
        cluster.chars().next().map(u32::from),
        Some(
            0x3400..=0x4DBF
                | 0x4E00..=0x9FFF
                | 0xF900..=0xFAFF
                | 0x20000..=0x2FA1F
                | 0x30000..=0x323AF
                | 0x3041..=0x30FF
                | 0x31F0..=0x31FF
                | 0xFF66..=0xFF9F
                | 0x0E00..=0x0EFF
                | 0x1000..=0x109F
                | 0x1780..=0x17FF
        )
    )
}

/// The pair of grapheme clusters spanning the boundary between
/// `previous_cluster` and the start of `word`, if both sides of the boundary are
/// in a segmented script.
fn cross_pair(word: &str, previous_cluster: Option<&str>) -> Option<String> {
    let previous = previous_cluster.filter(|c| is_segmented_cluster(c))?;
    let first = word.graphemes(true).next()?;
    if !is_segmented_cluster(first) {
        return None;
    }
    let mut bigram = String::with_capacity(previous.len() + first.len());
    bigram.push_str(previous);
    bigram.push_str(first);
    Some(bigram)
}

/// Index every pair of adjacent grapheme clusters in a segment, including the
/// pair that spans the boundary from `previous_cluster`.
fn pair_bigrams(word: &str, previous_cluster: Option<&str>) -> Vec<String> {
    let mut bigrams: Vec<String> = cross_pair(word, previous_cluster).into_iter().collect();
    // The pair spanning the boundary, if any, was the first one
    let mut previous: Option<&str> = None;
    for cluster in word.graphemes(true) {
        if !is_segmented_cluster(cluster) {
            previous = None;
            continue;
        }
        if let Some(previous) = previous {
            let mut bigram = String::with_capacity(previous.len() + cluster.len());
            bigram.push_str(previous);
            bigram.push_str(cluster);
            bigrams.push(bigram);
        }
        previous = Some(cluster);
    }
    bigrams
}

/// True for the characters ICU4X (currently) leaves in an unsegmented run: katakana,
/// the prolonged sound mark, and their halfwidth forms.
fn is_katakana(c: char) -> bool {
    matches!(c as u32,
        0x30A1..=0x30FA | 0x30FC..=0x30FF | 0x31F0..=0x31FF | 0xFF66..=0xFF9F)
}

/// ICU4X's dictionary (currently) has no katakana entries.
/// Indexing every suffix is a cheap hack for now, so the existing prefix
/// matching resolves a query for substrings.
///
/// Tracking: https://github.com/unicode-org/icu4x/issues/2239
fn katakana_suffixes(word: &str) -> Vec<&str> {
    if word.chars().count() < 3 || !word.chars().all(is_katakana) {
        return Vec::new();
    }
    let mut seen = HashSet::new();
    let mut suffixes = Vec::new();
    for (i, _) in word.char_indices().skip(1) {
        let rest = &word[i..];
        let suffix = match rest.char_indices().nth(MAX_KATAKANA_SUFFIX_CHARS) {
            Some((end, _)) => &rest[..end],
            None => rest,
        };
        // Skip duplicate suffixes
        if suffix.chars().count() >= 2 && seen.insert(suffix) {
            suffixes.push(suffix);
        }
    }
    suffixes
}

/// Normalize diacritics, if required, before indexing
/// (context: we index "café" as "cafe" for retrieval purposes,
/// but we do still store each variant in the index,
/// so matching diacritics are preferred when ranking search results)
fn normalize_diacritics(word: &str) -> Option<String> {
    if word.is_ascii() || !has_decomposition(word) {
        return None;
    }

    // The kana voiced/semi-voiced sound marks are combining marks, but they
    // distinguish words, so they are kept
    let stripped: String = word
        .nfd()
        .filter(|c| {
            !unicode_normalization::char::is_combining_mark(*c)
                || matches!(*c, '\u{3099}' | '\u{309A}')
        })
        .collect();

    // Recompose when kana voiced/semi-voiced sound marks survived,
    // otherwise we conflate バンド ("band") and ハンド ("hand").
    // Leave everything else decomposed for better search, for example in Hangul,
    // jamo are letters rather than combining marks. Composing them into syllable
    // blocks would stop a partially-typed syllable being a prefix of the indexed word
    let normalized = if stripped.chars().any(|c| matches!(c, '\u{3099}' | '\u{309A}')) {
        stripped.nfc().collect::<String>()
    } else {
        stripped
    };

    if normalized != word {
        Some(normalized)
    } else {
        None
    }
}

fn has_decomposition(word: &str) -> bool {
    word.chars().any(|c| {
        unicode_normalization::char::is_combining_mark(c) || {
            let mut decomposes = false;
            unicode_normalization::char::decompose_canonical(c, |part| {
                decomposes = decomposes || part != c;
            });
            decomposes
        }
    })
}

fn get_discrete_words<S: AsRef<str>>(s: S) -> (String, Option<Vec<String>>) {
    let mut extras = None;

    let words = s
        .as_ref()
        .replace(|c: char| c.is_ascii_punctuation(), " ")
        .to_case(Case::Lower)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if EMOJI.is_match(s.as_ref()) {
        extras = Some(
            s.as_ref()
                .graphemes(true)
                .into_iter()
                .filter_map(|x| {
                    if emojis::get(x).is_some() {
                        Some(x.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        );
    }

    (words, extras)
}

#[cfg(test)]
mod tests {
    use crate::fossick::get_stemmer;

    use super::*;

    #[test]
    fn indexable_words_simple() {
        let stemmer = get_stemmer("en");
        let results = get_indexable_words("hello", stemmer.as_ref(), &[]);

        assert_eq!(
            results,
            vec![IndexableWord {
                stemmed: "hello".to_string(),
                is_compound_part: false,
                original: None
            }]
        );
    }

    #[test]
    fn indexable_words_compound() {
        let stemmer = get_stemmer("en");

        let expected = vec![
            IndexableWord {
                stemmed: "threeantelop".to_string(),
                is_compound_part: false,
                original: None,
            },
            IndexableWord {
                stemmed: "three".to_string(),
                is_compound_part: true,
                original: None,
            },
            IndexableWord {
                stemmed: "antelop".to_string(),
                is_compound_part: true,
                original: None,
            },
        ];

        let camel_results = get_indexable_words("ThreeAntelopes", stemmer.as_ref(), &[]);
        let kebab_results = get_indexable_words("three-antelopes", stemmer.as_ref(), &[]);
        let snake_results = get_indexable_words("three_antelopes", stemmer.as_ref(), &[]);

        assert_eq!(camel_results, expected);
        assert_eq!(kebab_results, expected);
        assert_eq!(snake_results, expected);
    }

    #[test]
    fn indexable_words_compound_diacritics() {
        let stemmer = get_stemmer("fr");
        let results = get_indexable_words("café-crème", stemmer.as_ref(), &[]);

        assert_eq!(
            results,
            vec![
                IndexableWord {
                    stemmed: "cafecrem".to_string(),
                    is_compound_part: false,
                    original: Some("cafécrème".to_string()),
                },
                IndexableWord {
                    stemmed: "caf".to_string(),
                    is_compound_part: true,
                    original: Some("café".to_string()),
                },
                IndexableWord {
                    stemmed: "crem".to_string(),
                    is_compound_part: true,
                    original: Some("crème".to_string()),
                },
            ]
        );
    }

    #[test]
    fn hyphenated_words() {
        let input = "these-words-are-hyphenated";
        assert_eq!(
            get_discrete_words(input),
            ("these words are hyphenated".into(), None)
        );
    }

    #[test]
    fn underscored_words() {
        let input = "__array_structures";
        assert_eq!(get_discrete_words(input), ("array structures".into(), None));
    }

    #[test]
    fn camel_words() {
        let input = "WKWebVIEWComponent";
        assert_eq!(
            get_discrete_words(input),
            ("wk web view component".into(), None)
        );
    }

    #[test]
    fn dotted_words() {
        let input = "page.Find";
        assert_eq!(get_discrete_words(input), ("page find".into(), None));
    }

    #[test]
    fn misc_punctuation() {
        let input = "cloud/cannon,page.find";
        assert_eq!(
            get_discrete_words(input),
            ("cloud cannon page find".into(), None)
        );
    }

    #[test]
    fn french() {
        let input = "l'alphabet";
        assert_eq!(get_discrete_words(input), ("l alphabet".into(), None));
    }

    #[test]
    fn html() {
        let input = "<FormComponent data-pagefind-meta='[key:(value)]'>";
        assert_eq!(
            get_discrete_words(input),
            ("form component data pagefind meta key value".into(), None)
        );
    }

    #[test]
    fn diacritics_are_normalized_like_the_search_query() {
        // Latin marks are stripped
        assert_eq!(normalize_diacritics("café"), Some("cafe".to_string()));
        // Kana voiced sound marks are kept, and recomposed
        assert_eq!(normalize_diacritics("バンド"), None);
        assert_eq!(
            normalize_diacritics("ハ\u{3099}ント\u{3099}"),
            Some("バンド".to_string())
        );
        // Hangul is left decomposed, so that a partially typed syllable is a
        // prefix of the indexed word
        assert_eq!(
            normalize_diacritics("한"),
            Some("\u{1112}\u{1161}\u{11AB}".to_string())
        );
        // Han has nothing to normalize
        assert_eq!(normalize_diacritics("東京"), None);
        // Thai marks are stripped, as they are from the search query
        assert_eq!(
            normalize_diacritics("วิกิพีเดีย"),
            Some("วกพเดย".to_string())
        );
    }

    #[test]
    fn bigrams_span_segments() {
        let bigrams = |word: &str, previous: Option<&str>| pair_bigrams(word, previous);

        assert_eq!(bigrams("學校", None), vec!["學校".to_string()]);
        assert_eq!(
            bigrams("高速公路", None),
            vec!["高速".to_string(), "速公".to_string(), "公路".to_string()]
        );
        // The pair spanning two segments, so that a query for the ICU4C word
        // 學校 finds a page that ICU4X segmented as 在學|校
        assert_eq!(bigrams("校", Some("學")), vec!["學校".to_string()]);
        // Thai clusters stay whole, so a query for พี finds a page that
        // ICU4X segmented as วิ|กิพี|เดีย
        assert_eq!(
            bigrams("เดีย", Some("พี")),
            vec!["พีเ".to_string(), "เดี".to_string(), "ดีย".to_string()]
        );
        // Latin text on a segmented page is left alone
        assert!(bigrams("hello", Some("校")).is_empty());
        // Punctuation doesn't form pairs
        assert!(bigrams("校", Some("。")).is_empty());
        assert!(bigrams("corner", Some("。")).is_empty());
    }

    #[test]
    fn only_the_pair_spanning_two_segments_is_indexed_against_the_first() {
        let cross = |word: &str, previous: Option<&str>| {
            get_indexable_cross_bigram(word, previous, None, &[])
                .map(|bigram| bigram.stemmed)
                .unwrap_or_default()
        };

        assert_eq!(cross("校", Some("學")), "學校");
        // The stored word is stemmed, which for Thai strips the marks
        assert_eq!(cross("เดีย", Some("พี")), "พเ");
        // Nothing to span at the start of a run of text
        assert_eq!(cross("學校", None), "");
        // Latin text and punctuation don't span
        assert_eq!(cross("hello", Some("校")), "");
        assert_eq!(cross("corner", Some("。")), "");
        assert_eq!(cross("。", Some("校")), "");
        // A pair that is already indexable isn't indexed again
        let existing = vec![IndexableWord {
            stemmed: "的學".to_string(),
            is_compound_part: false,
            original: None,
        }];
        assert!(get_indexable_cross_bigram("學校", Some("的"), None, &existing).is_none());
    }

    #[test]
    fn two_cluster_words_are_not_duplicated() {
        let results = get_indexable_words("學校", None, &[]);
        assert_eq!(
            results,
            vec![IndexableWord {
                stemmed: "學校".to_string(),
                is_compound_part: false,
                original: None,
            }]
        );
    }

    #[test]
    fn katakana_suffixes_of_a_compound() {
        assert_eq!(
            katakana_suffixes("バックアップ"),
            vec!["ックアップ", "クアップ", "アップ", "ップ"]
        );
        assert!(katakana_suffixes("テスト").contains(&"スト"));
        assert!(katakana_suffixes("リスト型").is_empty());
    }

    #[test]
    fn katakana_suffixes_stay_linear_on_long_runs() {
        let kana: Vec<char> = ('ァ'..='ヶ').collect();
        let run: String = (0..10_000).map(|i| kana[(i * 7) % kana.len()]).collect();
        let suffixes = katakana_suffixes(&run);
        assert!(suffixes
            .iter()
            .all(|s| s.chars().count() <= MAX_KATAKANA_SUFFIX_CHARS));

        // A repetitive run collapses to one suffix per distinct length
        let rule = "ー".repeat(1_000);
        assert_eq!(
            katakana_suffixes(&rule).len(),
            MAX_KATAKANA_SUFFIX_CHARS - 1
        );
    }

    #[test]
    fn emoji() {
        let input = "cloud🌦️cannon";
        assert_eq!(
            get_discrete_words(input),
            ("cloud🌦️cannon".into(), Some(vec!["🌦️".into()]))
        );

        let input = "👋👨‍👩‍👧‍👦🌾";
        assert_eq!(
            get_discrete_words(input),
            (
                "👋👨‍👩‍👧‍👦🌾".into(),
                Some(vec!["👋".into(), "👨‍👩‍👧‍👦".into(), "🌾".into()])
            )
        );
    }
}
