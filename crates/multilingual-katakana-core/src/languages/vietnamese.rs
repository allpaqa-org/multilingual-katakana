use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{lookup_sorted, VIETNAMESE_PHRASES, VIETNAMESE_WORDS};

/// Preprocesses Vietnamese text:
/// - Maps palatal/affricate digraphs (`nh`, `gi`, `tr`) followed by a vowel to Katakana,
///   mirroring how Spanish `ñ` is handled (irregular pronunciation, not approximable via
///   plain English phonics).
/// - Normalizes Vietnamese tone marks and vowel diacritics (`ă`, `â`, `ê`, `ô`, `ơ`, `ư`
///   plus all six tone variants) to their base ASCII vowel so the English phonics
///   fallback can process the remainder.
/// - Maps `đ`/`Đ` to plain `d`.
pub fn vietnamese_preprocess(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];
        match c {
            'n' | 'N' if i + 1 < len && matches!(chars[i + 1], 'h' | 'H') => {
                let vowel = (i + 2 < len)
                    .then(|| normalize_vietnamese_vowel(chars[i + 2]).to_ascii_lowercase());
                match vowel {
                    Some('a') => {
                        result.push_str("ニャ");
                        i += 3;
                    }
                    Some('e') => {
                        result.push_str("ニェ");
                        i += 3;
                    }
                    Some('i') => {
                        result.push('ニ');
                        i += 3;
                    }
                    Some('o') => {
                        result.push_str("ニョ");
                        i += 3;
                    }
                    Some('u') => {
                        result.push_str("ニュ");
                        i += 3;
                    }
                    _ => {
                        result.push('ニ');
                        i += 2;
                    }
                }
            }
            'g' | 'G' if i + 1 < len && matches!(chars[i + 1], 'i' | 'I') => {
                let vowel = (i + 2 < len)
                    .then(|| normalize_vietnamese_vowel(chars[i + 2]).to_ascii_lowercase());
                match vowel {
                    Some('a') => {
                        result.push_str("ジャ");
                        i += 3;
                    }
                    Some('e') => {
                        result.push_str("ジェ");
                        i += 3;
                    }
                    Some('o') => {
                        result.push_str("ジョ");
                        i += 3;
                    }
                    Some('u') => {
                        result.push_str("ジュ");
                        i += 3;
                    }
                    _ => {
                        result.push('ジ');
                        i += 2;
                    }
                }
            }
            't' | 'T' if i + 1 < len && matches!(chars[i + 1], 'r' | 'R') => {
                let vowel = (i + 2 < len)
                    .then(|| normalize_vietnamese_vowel(chars[i + 2]).to_ascii_lowercase());
                match vowel {
                    Some('a') => {
                        result.push_str("チャ");
                        i += 3;
                    }
                    Some('e') => {
                        result.push_str("チェ");
                        i += 3;
                    }
                    Some('i') => {
                        result.push('チ');
                        i += 3;
                    }
                    Some('o') => {
                        result.push_str("チョ");
                        i += 3;
                    }
                    Some('u') => {
                        result.push_str("チュ");
                        i += 3;
                    }
                    _ => {
                        result.push('チ');
                        i += 2;
                    }
                }
            }
            'đ' | 'Đ' => {
                result.push('d');
                i += 1;
            }
            _ => {
                result.push(normalize_vietnamese_vowel(c));
                i += 1;
            }
        }
    }

    result
}

/// Maps a single Vietnamese tone-marked / modified vowel character to its base ASCII vowel.
/// Characters without a mapping are returned unchanged.
fn normalize_vietnamese_vowel(c: char) -> char {
    match c {
        'à' | 'á' | 'ả' | 'ã' | 'ạ' | 'ă' | 'ằ' | 'ắ' | 'ẳ' | 'ẵ' | 'ặ' | 'â' | 'ầ' | 'ấ' | 'ẩ'
        | 'ẫ' | 'ậ' => 'a',
        'À' | 'Á' | 'Ả' | 'Ã' | 'Ạ' | 'Ă' | 'Ằ' | 'Ắ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Â' | 'Ầ' | 'Ấ' | 'Ẩ'
        | 'Ẫ' | 'Ậ' => 'A',
        'è' | 'é' | 'ẻ' | 'ẽ' | 'ẹ' | 'ê' | 'ề' | 'ế' | 'ể' | 'ễ' | 'ệ' => 'e',
        'È' | 'É' | 'Ẻ' | 'Ẽ' | 'Ẹ' | 'Ê' | 'Ề' | 'Ế' | 'Ể' | 'Ễ' | 'Ệ' => 'E',
        'ì' | 'í' | 'ỉ' | 'ĩ' | 'ị' => 'i',
        'Ì' | 'Í' | 'Ỉ' | 'Ĩ' | 'Ị' => 'I',
        'ò' | 'ó' | 'ỏ' | 'õ' | 'ọ' | 'ô' | 'ồ' | 'ố' | 'ổ' | 'ỗ' | 'ộ' | 'ơ' | 'ờ' | 'ớ' | 'ở'
        | 'ỡ' | 'ợ' => 'o',
        'Ò' | 'Ó' | 'Ỏ' | 'Õ' | 'Ọ' | 'Ô' | 'Ồ' | 'Ố' | 'Ổ' | 'Ỗ' | 'Ộ' | 'Ơ' | 'Ờ' | 'Ớ' | 'Ở'
        | 'Ỡ' | 'Ợ' => 'O',
        'ù' | 'ú' | 'ủ' | 'ũ' | 'ụ' | 'ư' | 'ừ' | 'ứ' | 'ử' | 'ữ' | 'ự' => 'u',
        'Ù' | 'Ú' | 'Ủ' | 'Ũ' | 'Ụ' | 'Ư' | 'Ừ' | 'Ứ' | 'Ử' | 'Ữ' | 'Ự' => 'U',
        'ỳ' | 'ý' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
        'Ỳ' | 'Ý' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'Y',
        other => other,
    }
}

fn vietnamese_phrase_to_pattern(phrase: &str) -> String {
    let mut pattern = String::from(r"(?i)\b");
    for c in phrase.chars() {
        match c {
            ' ' => pattern.push_str(r"\s+"),
            other => pattern.push_str(&regex::escape(&other.to_string())),
        }
    }
    pattern.push_str(r"\b");
    pattern
}

static COMPILED_VIETNAMESE_PHRASES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    VIETNAMESE_PHRASES
        .iter()
        .map(|&(phrase, katakana)| {
            let pattern = vietnamese_phrase_to_pattern(phrase);
            (
                Regex::new(&pattern).expect("valid regex for vietnamese phrase"),
                katakana,
            )
        })
        .collect()
});

/// Replaces multi-word Vietnamese phrases with Katakana using word-boundary aware regex.
pub fn replace_vietnamese_phrases(text: &str) -> String {
    let mut result = text.to_string();
    for (re, katakana) in COMPILED_VIETNAMESE_PHRASES.iter() {
        if re.is_match(&result) {
            result = re.replace_all(&result, *katakana).into_owned();
        }
    }
    result
}

/// Looks up a single Vietnamese word in `VIETNAMESE_WORDS`.
pub fn get_vietnamese_word(word: &str) -> Option<&'static str> {
    lookup_sorted(VIETNAMESE_WORDS, &word.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vietnamese_preprocess_digraphs() {
        assert_eq!(vietnamese_preprocess("nha"), "ニャ");
        assert_eq!(vietnamese_preprocess("nhà"), "ニャ");
        assert_eq!(vietnamese_preprocess("nho"), "ニョ");
        assert_eq!(vietnamese_preprocess("gia"), "ジャ");
        assert_eq!(vietnamese_preprocess("giá"), "ジャ");
        assert_eq!(vietnamese_preprocess("tra"), "チャ");
        assert_eq!(vietnamese_preprocess("trẻ"), "チェ");
        assert_eq!(vietnamese_preprocess("đẹp"), "dep");
        assert_eq!(vietnamese_preprocess("Đẹp"), "dep");
    }

    #[test]
    fn test_vietnamese_preprocess_tones() {
        assert_eq!(vietnamese_preprocess("cảm ơn"), "cam on");
        assert_eq!(vietnamese_preprocess("xin chào"), "xin chao");
        assert_eq!(vietnamese_preprocess("được"), "duoc");
    }

    #[test]
    fn test_replace_vietnamese_phrases() {
        assert_eq!(replace_vietnamese_phrases("xin chào"), "シンチャオ");
        assert_eq!(replace_vietnamese_phrases("cảm ơn"), "カムオン");
        assert_eq!(replace_vietnamese_phrases("Xin Chào"), "シンチャオ");
    }

    #[test]
    fn test_get_vietnamese_word() {
        assert_eq!(get_vietnamese_word("chào"), Some("チャオ"));
        assert_eq!(get_vietnamese_word("bạn"), Some("バン"));
        assert_eq!(get_vietnamese_word("unknown_word"), None);
    }
}
