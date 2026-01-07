use convert_case::{Case, Casing};
use emojis;
use lazy_static::lazy_static;
use pagefind_stem::Stemmer;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

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

    if !normalized_word.is_empty() {
        let diacritic_normalized = normalize_diacritics(&normalized_word);

        let stemmed = if let Some(stemmer) = stemmer {
            stemmer
                .stem(diacritic_normalized.as_deref().unwrap_or(&normalized_word))
                .into_owned()
        } else {
            diacritic_normalized
                .clone()
                .unwrap_or_else(|| normalized_word.to_string())
        };

        results.push(IndexableWord {
            stemmed,
            original: diacritic_normalized.map(|_| normalized_word.to_string()),
            is_compound_part: false,
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

/// Normalize diacritics, if required, before indexing
/// (context: we index "café" as "cafe" for retrieval purposes,
/// but we do still store each variant in the index,
/// so matching diacritics are preferred when ranking search results)
fn normalize_diacritics(word: &str) -> Option<String> {
    if word.is_ascii() {
        return None;
    }

    let normalized: String = word
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect();

    if normalized != word {
        Some(normalized)
    } else {
        None
    }
}

fn get_discrete_words<S: AsRef<str>>(s: S) -> (String, Option<Vec<String>>) {
    let mut extras = None;

    let words = s
        .as_ref()
        .replace(|c: char| c.is_ascii_punctuation(), " ")
        .to_case(Case::Lower);

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
