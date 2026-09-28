use crate::dicts::{lookup_sorted, CYRILLIC_LETTERS, CYRILLIC_PHRASES};

/// Checks whether the text contains any Cyrillic characters (Unicode U+0400..=U+04FF).
#[inline]
pub fn is_cyrillic(text: &str) -> bool {
    text.chars().any(|c| matches!(c, '\u{0400}'..='\u{04FF}'))
}

/// Converts Cyrillic text to Katakana for Japanese TTS engines.
///
/// 1. If no Cyrillic characters are present, returns original text.
/// 2. Converts text to lowercase.
/// 3. Performs greedy multi-word and word phrase replacement from `CYRILLIC_PHRASES`.
/// 4. Converts individual Cyrillic letters using `CYRILLIC_LETTERS`.
pub fn convert_cyrillic(text: &str) -> String {
    if !is_cyrillic(text) {
        return text.to_string();
    }

    let mut lower = text.to_lowercase();
    for &(phrase, katakana) in CYRILLIC_PHRASES {
        if lower.contains(phrase) {
            lower = lower.replace(phrase, katakana);
        }
    }

    let mut result = String::with_capacity(lower.len());
    let mut buf = [0u8; 4];
    for c in lower.chars() {
        let s = c.encode_utf8(&mut buf);
        if let Some(mapped) = lookup_sorted(CYRILLIC_LETTERS, s) {
            result.push_str(mapped);
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cyrillic() {
        assert!(is_cyrillic("Привет"));
        assert!(is_cyrillic("да"));
        assert!(is_cyrillic("объект"));
        assert!(!is_cyrillic("Hello"));
        assert!(!is_cyrillic("こんにちは"));
    }

    #[test]
    fn test_convert_cyrillic() {
        // Common greetings & thanks
        assert_eq!(convert_cyrillic("Привет"), "プリヴィエト");
        assert_eq!(convert_cyrillic("Спасибо"), "スパスィーバ");
        assert_eq!(convert_cyrillic("хорошо"), "ハラショー");

        // Basic words
        assert_eq!(convert_cyrillic("Да"), "ダ");
        assert_eq!(convert_cyrillic("да"), "ダ");
        assert_eq!(convert_cyrillic("Нет"), "ニェット");
        assert_eq!(convert_cyrillic("нет"), "ニェット");

        // Formal greetings & farewells
        assert_eq!(convert_cyrillic("здравствуйте"), "ズドラーストヴィチェ");
        assert_eq!(convert_cyrillic("пожалуйста"), "パジャールスタ");
        assert_eq!(convert_cyrillic("пока"), "パカー");
        assert_eq!(convert_cyrillic("досвидания"), "ダスヴィダーニャ");

        // Non-cyrillic pass-through
        assert_eq!(convert_cyrillic("Hello"), "Hello");
    }
}
