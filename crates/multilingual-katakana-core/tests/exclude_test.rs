use multilingual_katakana_core::{to_katakana, KatakanaConverter, KatakanaOptions};
use regex::Regex;

#[test]
fn test_string_exclusion() {
    let opts = KatakanaOptions::new()
        .with_exclude("bot")
        .with_exclude("nice");
    let actual = to_katakana("hello bot nice to meet you", Some(&opts));
    assert_eq!(actual, "ハロー bot nice トゥーミートユー");
}

#[test]
fn test_regex_url_exclusion() {
    let url_re = Regex::new(r"https?://\S+").unwrap();
    let opts = KatakanaOptions::new().with_exclude(url_re);
    let actual = to_katakana("check https://example.com bro", Some(&opts));
    assert_eq!(actual, "チェック https://example.com ブロ");
}

#[test]
fn test_regex_mention_exclusion() {
    let mention_re = Regex::new(r"@\w+").unwrap();
    let opts = KatakanaOptions::new().with_exclude(mention_re);
    let actual = to_katakana("hello @streamer_123 gg", Some(&opts));
    assert_eq!(actual, "ハロー @streamer_123 ジージー");
}

#[test]
fn test_mixed_language_with_exclusions() {
    let mention_re = Regex::new(r"@\w+").unwrap();
    let opts = KatakanaOptions::new().with_exclude(mention_re);
    let actual = to_katakana("안녕하세요 @streamer_123 gracias bro", Some(&opts));
    assert_eq!(actual, "アンニョンハセヨ @streamer_123 グラシアスブロ");
}

#[test]
fn test_chinese_and_url_exclusion() {
    let url_re = Regex::new(r"https?://\S+").unwrap();
    let opts = KatakanaOptions::new().with_exclude(url_re);
    let actual = to_katakana("你好 https://twitch.tv/example streamer", Some(&opts));
    assert_eq!(actual, "ニーハオ https://twitch.tv/example ストリーマー");
}

#[test]
fn test_russian_and_protected_token() {
    let opts = KatakanaOptions::new().with_exclude("BOT_V1");
    let actual = to_katakana("привет BOT_V1 hello", Some(&opts));
    assert_eq!(actual, "プリヴィエト BOT_V1 ハロー");
}

#[test]
fn test_multiple_occurrences() {
    let opts = KatakanaOptions::new().with_exclude("bot");
    let actual = to_katakana("bot hello bot and bot", Some(&opts));
    assert_eq!(actual, "bot ハロー bot アンド bot");
}

#[test]
fn test_katakana_converter_instance() {
    let opts = KatakanaOptions::new().with_exclude("bot");
    let converter = KatakanaConverter::with_options(opts);
    let actual = converter.convert("hello bot nice");
    assert_eq!(actual, "ハロー bot ナイス");
}

#[test]
fn test_empty_exclude() {
    let opts = KatakanaOptions::new();
    assert_eq!(to_katakana("hello bot", Some(&opts)), "ハローボット");
    assert_eq!(to_katakana("hello bot", None), "ハローボット");
}

#[test]
fn test_katakana_token_exclusion() {
    let opts = KatakanaOptions::new().with_exclude("ワラ");
    let actual = to_katakana("hello ワラ world", Some(&opts));
    assert_eq!(actual, "ハロー ワラ ワールド");
}
