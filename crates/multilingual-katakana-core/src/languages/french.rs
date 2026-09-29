use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{lookup_sorted, FRENCH_PHRASES, FRENCH_WORDS};

fn french_phrase_to_pattern(phrase: &str) -> String {
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

static COMPILED_FRENCH_PHRASES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    FRENCH_PHRASES
        .iter()
        .map(|&(phrase, katakana)| {
            let pattern = french_phrase_to_pattern(phrase);
            (
                Regex::new(&pattern).expect("valid regex for French phrase"),
                katakana,
            )
        })
        .collect()
});

/// Replaces curated French phrases; unknown text keeps following the normal pipeline.
pub fn replace_french_phrases(text: &str) -> String {
    let mut result = text.to_string();
    for (regex, katakana) in COMPILED_FRENCH_PHRASES.iter() {
        result = regex.replace_all(&result, *katakana).into_owned();
    }
    result
}

/// Looks up a curated French word by exact token.
pub fn get_french_word(word: &str) -> Option<&'static str> {
    lookup_sorted(FRENCH_WORDS, &word.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_french_phrases() {
        assert_eq!(
            replace_french_phrases("Bonjour mon ami! Merci beaucoup."),
            "ボンジュールモナミ! メルシーボクー."
        );
        assert_eq!(replace_french_phrases("S'il vous plaît"), "シルヴプレ");
    }

    #[test]
    fn test_get_french_word() {
        assert_eq!(get_french_word("bonjour"), Some("ボンジュール"));
        assert_eq!(get_french_word("AUJOURD'HUI"), Some("オジュルデュイ"));
        assert_eq!(get_french_word("hello"), None);
    }
}
