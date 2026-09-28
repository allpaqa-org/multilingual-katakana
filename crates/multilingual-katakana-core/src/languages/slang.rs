use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{lookup_sorted, SLANG_PHRASES, SLANG_WORDS};

static COMPILED_SLANG_PHRASES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    SLANG_PHRASES
        .iter()
        .map(|&(phrase, katakana)| {
            let escaped = regex::escape(phrase).replace(' ', r"\s+");
            let pattern = format!(r"(?i)\b{}\b", escaped);
            (
                Regex::new(&pattern).expect("valid regex for slang phrase"),
                katakana,
            )
        })
        .collect()
});

/// Replaces multi-word slang phrases with Katakana using word-boundary aware regex.
pub fn replace_slang_phrases(text: &str) -> String {
    let mut result = text.to_string();
    for (re, katakana) in COMPILED_SLANG_PHRASES.iter() {
        if re.is_match(&result) {
            result = re.replace_all(&result, *katakana).into_owned();
        }
    }
    result
}

/// Looks up a single slang word in `SLANG_WORDS`.
pub fn get_slang_word(word: &str) -> Option<&'static str> {
    lookup_sorted(SLANG_WORDS, &word.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_slang_phrases() {
        assert_eq!(replace_slang_phrases("gg wp"), "ジージー ウェルプレイド");
        assert_eq!(replace_slang_phrases("GG WP"), "ジージー ウェルプレイド");
        assert_eq!(replace_slang_phrases("gg   wp"), "ジージー ウェルプレイド");
        assert_eq!(
            replace_slang_phrases("that was gg wp!"),
            "that was ジージー ウェルプレイド!"
        );
    }

    #[test]
    fn test_slang_phrases_with_prosody() {
        use crate::normalizers::prosody::normalize_prosody;
        assert_eq!(
            normalize_prosody(&replace_slang_phrases("gg wp")),
            "ジージーウェルプレイド"
        );
    }

    #[test]
    fn test_get_slang_word() {
        assert_eq!(get_slang_word("gg"), Some("ジージー"));
        assert_eq!(get_slang_word("GG"), Some("ジージー"));
        assert_eq!(get_slang_word("ez"), Some("イージー"));
        assert_eq!(get_slang_word("pog"), Some("ポグ"));
        assert_eq!(get_slang_word("poggers"), Some("ポガーズ"));
        assert_eq!(get_slang_word("pogchamp"), Some("ポグチャンプ"));
        assert_eq!(get_slang_word("kekw"), Some("ケクダブリュー"));
        assert_eq!(get_slang_word("afk"), Some("エーエフケー"));
        assert_eq!(get_slang_word("brb"), Some("ビーアールビー"));
        assert_eq!(get_slang_word("lol"), Some("ロル"));
        assert_eq!(get_slang_word("lmao"), Some("エルエムエーオー"));
        assert_eq!(get_slang_word("wtf"), Some("ダブリューティーエフ"));
        assert_eq!(get_slang_word("nt"), Some("ナイストライ"));
        assert_eq!(get_slang_word("kusa"), Some("くさ"));
        assert_eq!(get_slang_word("w"), Some("わら"));
        assert_eq!(get_slang_word("ww"), Some("わらわら"));
        assert_eq!(get_slang_word("www"), Some("わらわら"));
        assert_eq!(get_slang_word("bro"), Some("ブロ"));
        assert_eq!(get_slang_word("unknown_slang"), None);
    }
}
