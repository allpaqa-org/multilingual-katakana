use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{lookup_sorted, SPANISH_PHRASES, SPANISH_WORDS};

/// Preprocesses Spanish text:
/// - Strips inverted punctuation ('¡' and '¿')
/// - Maps Spanish digraphs (`ña`..`ñu`, `ñ`, `ll`, `rr`) to Katakana
/// - Normalizes European accented letters and umlauts to ASCII
pub fn spanish_preprocess(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];
        match c {
            '¡' | '¿' => {
                i += 1;
            }
            'ñ' | 'Ñ' => {
                if i + 1 < len {
                    match chars[i + 1] {
                        'a' | 'A' => {
                            result.push_str("ニャ");
                            i += 2;
                        }
                        'e' | 'E' => {
                            result.push_str("ニェ");
                            i += 2;
                        }
                        'i' | 'I' => {
                            result.push('ニ');
                            i += 2;
                        }
                        'o' | 'O' => {
                            result.push_str("ニョ");
                            i += 2;
                        }
                        'u' | 'U' => {
                            result.push_str("ニュ");
                            i += 2;
                        }
                        _ => {
                            result.push_str("ニャ");
                            i += 1;
                        }
                    }
                } else {
                    result.push_str("ニャ");
                    i += 1;
                }
            }
            'l' | 'L' if i + 1 < len && (chars[i + 1] == 'l' || chars[i + 1] == 'L') => {
                result.push_str("リャ");
                i += 2;
            }
            'r' | 'R' if i + 1 < len && (chars[i + 1] == 'r' || chars[i + 1] == 'R') => {
                result.push('ル');
                i += 2;
            }
            'ü' | 'Ü' => {
                result.push('u');
                i += 1;
            }
            'ä' | 'Ä' => {
                result.push('e');
                i += 1;
            }
            'ö' | 'Ö' => {
                result.push('o');
                i += 1;
            }
            'ß' => {
                result.push_str("ss");
                i += 1;
            }
            'à' | 'â' | 'À' | 'Â' | 'á' | 'Á' => {
                result.push('a');
                i += 1;
            }
            'è' | 'ê' | 'ë' | 'È' | 'Ê' | 'Ë' | 'é' | 'É' => {
                result.push('e');
                i += 1;
            }
            'î' | 'ï' | 'Î' | 'Ï' | 'í' | 'Í' => {
                result.push('i');
                i += 1;
            }
            'ô' | 'Ô' | 'ó' | 'Ó' => {
                result.push('o');
                i += 1;
            }
            'û' | 'ù' | 'Û' | 'Ù' | 'ú' | 'Ú' => {
                result.push('u');
                i += 1;
            }
            'ç' | 'Ç' => {
                result.push('s');
                i += 1;
            }
            _ => {
                result.push(c);
                i += 1;
            }
        }
    }

    result
}

fn spanish_phrase_to_pattern(phrase: &str) -> String {
    let mut pattern = String::from(r"(?i)\b");
    for c in phrase.chars() {
        match c {
            ' ' => pattern.push_str(r"\s+"),
            'a' => pattern.push_str("[aá]"),
            'e' => pattern.push_str("[eé]"),
            'i' => pattern.push_str("[ií]"),
            'o' => pattern.push_str("[oó]"),
            'u' => pattern.push_str("[uúü]"),
            'n' => pattern.push_str("[nñ]"),
            other => {
                pattern.push_str(&regex::escape(&other.to_string()));
            }
        }
    }
    pattern.push_str(r"\b");
    pattern
}

static COMPILED_SPANISH_PHRASES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    SPANISH_PHRASES
        .iter()
        .map(|&(phrase, katakana)| {
            let pattern = spanish_phrase_to_pattern(phrase);
            (
                Regex::new(&pattern).expect("valid regex for spanish phrase"),
                katakana,
            )
        })
        .collect()
});

/// Replaces multi-word Spanish phrases with Katakana using word-boundary aware regex.
pub fn replace_spanish_phrases(text: &str) -> String {
    let mut result = text.to_string();
    for (re, katakana) in COMPILED_SPANISH_PHRASES.iter() {
        if re.is_match(&result) {
            result = re.replace_all(&result, *katakana).into_owned();
        }
    }
    result
}

/// Looks up a single Spanish word in `SPANISH_WORDS`.
pub fn get_spanish_word(word: &str) -> Option<&'static str> {
    lookup_sorted(SPANISH_WORDS, &word.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spanish_preprocess() {
        assert_eq!(spanish_preprocess("¡Hola!"), "Hola!");
        assert_eq!(spanish_preprocess("¿Cómo estás?"), "Como estas?");
        assert_eq!(spanish_preprocess("señor"), "seニョr");
        assert_eq!(spanish_preprocess("señora"), "seニョra");
        assert_eq!(spanish_preprocess("llama"), "リャama");
        assert_eq!(spanish_preprocess("perro"), "peルo");
        assert_eq!(spanish_preprocess("niño"), "niニョ");
        assert_eq!(spanish_preprocess("niña"), "niニャ");
    }

    #[test]
    fn test_replace_spanish_phrases() {
        assert_eq!(replace_spanish_phrases("buenos dias"), "ブエノス ディアス");
        assert_eq!(replace_spanish_phrases("Buenos dias"), "ブエノス ディアス");
        assert_eq!(replace_spanish_phrases("Buenos días"), "ブエノス ディアス");
        assert_eq!(
            replace_spanish_phrases("muchas gracias"),
            "ムチャス グラシアス"
        );
        assert_eq!(
            replace_spanish_phrases("buenas noches"),
            "ブエナス ノーチェス"
        );
        assert_eq!(replace_spanish_phrases("por favor"), "ポル ファボール");
        assert_eq!(replace_spanish_phrases("como estas"), "コモ エスタス");
    }

    #[test]
    fn test_spanish_phrases_with_prosody() {
        use crate::normalizers::prosody::normalize_prosody;
        assert_eq!(
            normalize_prosody(&replace_spanish_phrases("Buenos días")),
            "ブエノスディアス"
        );
        assert_eq!(
            normalize_prosody(&replace_spanish_phrases("muchas gracias")),
            "ムチャスグラシアス"
        );
    }

    #[test]
    fn test_get_spanish_word() {
        assert_eq!(get_spanish_word("gracias"), Some("グラシアス"));
        assert_eq!(get_spanish_word("Gracias"), Some("グラシアス"));
        assert_eq!(get_spanish_word("amigo"), Some("アミーゴ"));
        assert_eq!(get_spanish_word("hola"), Some("オラ"));
        assert_eq!(get_spanish_word("adios"), Some("アディオス"));
        assert_eq!(get_spanish_word("unknown_word"), None);
    }
}
