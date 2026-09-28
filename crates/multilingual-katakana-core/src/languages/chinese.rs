use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{
    lookup_char_sorted, CHINESE_COMMON_WORDS, CHINESE_HANZI_MAP, CHINESE_MARKER_PATTERN,
    CHINESE_TAIWAN_PHRASES,
};

static CHINESE_MARKER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(CHINESE_MARKER_PATTERN).expect("Invalid CHINESE_MARKER_PATTERN"));

static PUNCT_SPACE_BEFORE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s\u{3000}]+([、。！？!?,.])").expect("Invalid regex"));

static PUNCT_SPACE_AFTER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([、。！？!?,.])[\s\u{3000}]+").expect("Invalid regex"));

static MULTI_SPACE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s\u{3000}]+").expect("Invalid regex"));

/// Determines if the text should be treated as Chinese.
///
/// 1. Japanese Absolute Guard: If text contains ANY Hiragana or Katakana, returns `false`.
/// 2. Must contain at least one CJK Ideograph (`\u{4E00}..=\u{9FFF}`).
/// 3. Checks Chinese marker regex pattern (`CHINESE_MARKER_PATTERN`).
/// 4. Checks Chinese common words (`CHINESE_COMMON_WORDS`).
pub fn is_chinese(text: &str) -> bool {
    // 1. Japanese Absolute Guard: If text contains ANY Hiragana or Katakana, 100% Japanese!
    if text
        .chars()
        .any(|c| matches!(c, '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}'))
    {
        return false;
    }

    // 2. Must contain at least one CJK Ideograph (漢字/汉字)
    if !text.chars().any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c)) {
        return false;
    }

    // 3. Check for distinctive Chinese marker characters / particles
    if CHINESE_MARKER_REGEX.is_match(text) {
        return true;
    }

    // 4. Check for common Chinese phrases/greetings
    for &word in CHINESE_COMMON_WORDS {
        if text.contains(word) {
            return true;
        }
    }

    // 5. Otherwise return false
    false
}

/// Replace Taiwan phrases and greetings (sorted by length descending).
pub fn replace_taiwan_phrases(text: &str) -> String {
    let mut result = text.to_string();
    for &(phrase, katakana) in CHINESE_TAIWAN_PHRASES {
        if result.contains(phrase) {
            result = result.replace(phrase, katakana);
        }
    }
    result
}

/// Convert Chinese text to Katakana for Japanese TTS.
pub fn convert_chinese(text: &str) -> String {
    // 1. Taiwan phrases & greetings are replaced first (even in mixed Japanese text)
    let mut result = replace_taiwan_phrases(text);

    // 2. If pure Chinese text, convert all remaining Hanzi blocks
    if is_chinese(text) {
        let mut converted = String::with_capacity(result.len() * 2);
        for c in result.chars() {
            if ('\u{4E00}'..='\u{9FFF}').contains(&c) {
                if let Some(katakana) = lookup_char_sorted(CHINESE_HANZI_MAP, c) {
                    converted.push_str(katakana);
                } else {
                    converted.push(c);
                }
            } else {
                match c {
                    '，' => converted.push('、'),
                    '。' => converted.push('。'),
                    '！' => converted.push('！'),
                    '？' => converted.push('？'),
                    other => converted.push(other),
                }
            }
        }
        result = converted;

        // 3. Remove unnecessary spaces around punctuation
        result = PUNCT_SPACE_BEFORE_RE
            .replace_all(&result, "$1")
            .into_owned();
        result = PUNCT_SPACE_AFTER_RE.replace_all(&result, "$1").into_owned();

        // 4. Normalize spaces: replace multiple whitespace/full-width space with single space and trim
        result = MULTI_SPACE_RE.replace_all(&result, " ").trim().to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kanji_guard() {
        // Safe Kanji Guard:
        assert!(!is_chinese("了解しました"));
        assert!(!is_chinese("初見歓迎です"));
        assert!(!is_chinese("神回"));
        assert!(!is_chinese("配信開始！"));
        assert!(!is_chinese("了解"));
        assert!(!is_chinese("初見歓迎"));
        assert!(!is_chinese("感謝"));
        assert!(!is_chinese("最高"));
        assert!(!is_chinese("優勝"));

        assert_eq!(convert_chinese("了解しました"), "了解しました");
        assert_eq!(convert_chinese("初見歓迎です"), "初見歓迎です");
        assert_eq!(convert_chinese("神回"), "神回");
        assert_eq!(convert_chinese("配信開始！"), "配信開始！");
    }

    #[test]
    fn test_chinese_conversion() {
        assert_eq!(convert_chinese("你好"), "ニーハオ");
        assert_eq!(convert_chinese("謝謝"), "シエシエ");
        assert_eq!(convert_chinese("笑死"), "シアオスー");
        assert_eq!(
            convert_chinese("我喜欢这个直播"),
            "ウォ喜ホアンジャーガージー播"
        );
    }

    #[test]
    fn test_spec_cases_chinese() {
        assert_eq!(
            convert_chinese("你好！玩得很好，加油！"),
            "ニーハオ！ワンドゥヘンハオ、ジャーヨウ！"
        );
        assert_eq!(
            convert_chinese("大家安安！實況主太強了，謝謝乾爹！"),
            "ダージアアンアン！シークアンジュータイチャンラ、シエシエガンディエ！"
        );
        assert_eq!(
            convert_chinese("這個是垃圾桶"),
            "ジャーガーシーレースートン"
        );
        assert_eq!(
            convert_chinese("實況主太厲害了吧"),
            "シークアンジュータイリーハイラバー"
        );
        assert_eq!(convert_chinese("謝謝乾爹"), "シエシエガンディエ");
        assert_eq!(convert_chinese("早安！"), "ザオアン！");
        assert_eq!(convert_chinese("晚安！"), "ワンアン！");
        assert_eq!(
            convert_chinese("笑死，太好笑了！"),
            "シアオスー、タイハオシアオラ！"
        );
        assert_eq!(
            convert_chinese("辛苦了！明天見！"),
            "シンクーラ！ミンティエンジエン！"
        );
        assert_eq!(
            convert_chinese("歡迎大家！請按讚！"),
            "ホワンインダージア！チンアンザン！"
        );
    }

    #[test]
    fn test_replace_taiwan_phrases() {
        assert_eq!(replace_taiwan_phrases("你好"), "ニーハオ");
        assert_eq!(replace_taiwan_phrases("謝謝"), "シエシエ");
        assert_eq!(replace_taiwan_phrases("笑死"), "シアオスー");
        assert_eq!(replace_taiwan_phrases("按讚"), "アンザン");
        assert_eq!(replace_taiwan_phrases("辛苦了"), "シンクーラ");
        assert_eq!(replace_taiwan_phrases("加油"), "ジャーヨウ");
    }
}
