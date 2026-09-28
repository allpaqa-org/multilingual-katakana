use crate::dicts::{lookup_sorted, ENGLISH_WORDS};

/// Suffix replacements applied globally.
static SUFFIX_PHRASES: &[(&str, &str)] = &[
    ("tion", "ション"),
    ("sion", "ジョン"),
    ("ture", "チャー"),
    ("ight", "アイト"),
    ("ough", "オフ"),
];

/// Digraphs and phonics combinations.
static DIGRAPHS: &[(&str, &str)] = &[
    ("ch", "チ"),
    ("sh", "シュ"),
    ("ph", "フ"),
    ("th", "ス"),
    ("wh", "ホ"),
    ("ck", "ック"),
    ("qu", "ク"),
    ("kn", "ナ"),
    ("wr", "ラ"),
    ("eer", "イアー"),
    ("ee", "イー"),
    ("ea", "イー"),
    ("oo", "ウー"),
    ("ai", "エイ"),
    ("ay", "エイ"),
    ("oa", "オー"),
    ("ou", "アウ"),
    ("ow", "オウ"),
];

/// Consonant clusters.
static CLUSTERS: &[(&str, &str)] = &[
    ("str", "ストラ"),
    ("st", "スト"),
    ("sp", "スピ"),
    ("sk", "スク"),
    ("sm", "スメ"),
    ("sn", "スネ"),
    ("sw", "スワ"),
    ("gra", "グラ"),
    ("gri", "グリ"),
    ("gru", "グル"),
    ("gre", "グレ"),
    ("gro", "グロ"),
    ("gr", "グラ"),
    ("tra", "トラ"),
    ("tri", "トリ"),
    ("tru", "トゥルー"),
    ("tre", "トレ"),
    ("tro", "トロ"),
    ("tr", "トラ"),
    ("bra", "ブラ"),
    ("bri", "ブリ"),
    ("bru", "ブル"),
    ("bre", "ブレ"),
    ("bro", "ブロ"),
    ("br", "ブラ"),
    ("cra", "クラ"),
    ("cri", "クリ"),
    ("cru", "クル"),
    ("cre", "クレ"),
    ("cro", "クロ"),
    ("cr", "クラ"),
    ("dra", "ドラ"),
    ("dri", "ドリ"),
    ("dru", "ドラ"),
    ("dre", "ドレ"),
    ("dro", "ドロ"),
    ("dr", "ドラ"),
    ("pla", "プラ"),
    ("pli", "プリ"),
    ("plu", "プル"),
    ("ple", "プレ"),
    ("plo", "プロ"),
    ("pl", "プル"),
    ("fla", "フラ"),
    ("fli", "フリ"),
    ("flu", "フル"),
    ("fle", "フレ"),
    ("flo", "フロ"),
    ("fl", "フル"),
];

/// Standard CV (Consonant-Vowel) syllables.
static CV_TABLE: &[(&str, &str)] = &[
    ("kya", "キャ"),
    ("kyu", "キュ"),
    ("kyo", "キョ"),
    ("sha", "シャ"),
    ("shu", "シュ"),
    ("sho", "ショ"),
    ("shi", "シ"),
    ("cha", "チャ"),
    ("chu", "チュ"),
    ("cho", "チョ"),
    ("chi", "チ"),
    ("tsu", "ツ"),
    ("fa", "ファ"),
    ("fi", "フィ"),
    ("fe", "フェ"),
    ("fo", "フォ"),
    ("fu", "フ"),
    ("ja", "ジャ"),
    ("ju", "ジュ"),
    ("jo", "ジョ"),
    ("ji", "ジ"),
    ("va", "ヴァ"),
    ("vi", "ヴィ"),
    ("vu", "ヴ"),
    ("ve", "ヴェ"),
    ("vo", "ヴォ"),
    ("ka", "カ"),
    ("ki", "キ"),
    ("ku", "ク"),
    ("ke", "ケ"),
    ("ko", "コ"),
    ("sa", "サ"),
    ("si", "シ"),
    ("su", "ス"),
    ("se", "セ"),
    ("so", "ソ"),
    ("ta", "タ"),
    ("ti", "ティ"),
    ("tu", "トゥ"),
    ("te", "テ"),
    ("to", "ト"),
    ("na", "ナ"),
    ("ni", "ニ"),
    ("nu", "ヌ"),
    ("ne", "ネ"),
    ("no", "ノ"),
    ("ha", "ハ"),
    ("hi", "ヒ"),
    ("he", "ヘ"),
    ("ho", "ホ"),
    ("ma", "マ"),
    ("mi", "ミ"),
    ("mu", "ム"),
    ("me", "メ"),
    ("mo", "モ"),
    ("ya", "ヤ"),
    ("yu", "ユ"),
    ("yo", "ヨ"),
    ("ra", "ラ"),
    ("ri", "リ"),
    ("ru", "ル"),
    ("re", "レ"),
    ("ro", "ロ"),
    ("la", "ラ"),
    ("li", "リ"),
    ("lu", "ル"),
    ("le", "レ"),
    ("lo", "ロ"),
    ("wa", "ワ"),
    ("wo", "ウォ"),
    ("ga", "ガ"),
    ("gi", "ギ"),
    ("gu", "グ"),
    ("ge", "ゲ"),
    ("go", "ゴ"),
    ("za", "ザ"),
    ("zu", "ズ"),
    ("ze", "ゼ"),
    ("zo", "ゾ"),
    ("da", "ダ"),
    ("di", "ディ"),
    ("du", "ドゥ"),
    ("de", "デ"),
    ("do", "ド"),
    ("ba", "バ"),
    ("bi", "ビ"),
    ("bu", "ブ"),
    ("be", "ベ"),
    ("bo", "ボ"),
    ("pa", "パ"),
    ("pi", "ピ"),
    ("pu", "プ"),
    ("pe", "ペ"),
    ("po", "ポ"),
    ("ca", "カ"),
    ("cu", "ク"),
    ("co", "コ"),
    ("ci", "シ"),
    ("ce", "セ"),
    ("a", "ア"),
    ("i", "イ"),
    ("u", "ウ"),
    ("e", "エ"),
    ("o", "オ"),
];

/// Remaining standalone consonants.
static CONSONANT_TABLE: &[(&str, &str)] = &[
    ("b", "ブ"),
    ("c", "ク"),
    ("d", "ド"),
    ("f", "フ"),
    ("g", "グ"),
    ("h", "ハ"),
    ("j", "ジ"),
    ("k", "ク"),
    ("l", "ル"),
    ("m", "ム"),
    ("n", "ン"),
    ("p", "プ"),
    ("q", "ク"),
    ("r", "ル"),
    ("s", "ス"),
    ("t", "ト"),
    ("v", "ヴ"),
    ("w", "ウ"),
    ("x", "クス"),
    ("y", "イ"),
    ("z", "ズ"),
];

/// Look up a word in the pre-compiled English dictionary (CMUdict).
/// Matching is case-insensitive.
pub fn get_english_word(word: &str) -> Option<&'static str> {
    if word.bytes().any(|b| b.is_ascii_uppercase()) {
        let lower = word.to_ascii_lowercase();
        lookup_sorted(ENGLISH_WORDS, &lower)
    } else {
        lookup_sorted(ENGLISH_WORDS, word)
    }
}

/// Helper to sequentially replace patterns from a table if found in the word.
#[inline]
fn replace_table(word: &mut String, table: &[(&str, &str)]) {
    for &(pat, rep) in table {
        if word.contains(pat) {
            *word = word.replace(pat, rep);
        }
    }
}

/// Suffix matching at word end (`$` anchor).
#[inline]
fn apply_end_suffixes(word: &mut String) {
    if word.ends_with("ing") {
        word.truncate(word.len() - 3);
        word.push_str("イング");
    } else if word.ends_with("ed") {
        word.truncate(word.len() - 2);
        word.push('ド');
    } else if word.ends_with("er") {
        word.truncate(word.len() - 2);
        word.push_str("アー");
    } else if word.ends_with("or") {
        word.truncate(word.len() - 2);
        word.push_str("オー");
    } else if word.ends_with("ar") {
        word.truncate(word.len() - 2);
        word.push_str("アー");
    } else if word.ends_with("al") {
        word.truncate(word.len() - 2);
        word.push_str("アル");
    }
}

/// Silent E rule: vowel + consonant + e -> long vowel + consonant at word end.
#[inline]
fn apply_silent_e(word: &mut String) {
    let mut chars = word.chars().rev();
    if chars.next() == Some('e') {
        if let Some(c) = chars.next() {
            if matches!(c, 'b'..='d' | 'f'..='h' | 'j'..='n' | 'p'..='t' | 'v'..='z') {
                if let Some(v) = chars.next() {
                    let prefix = match v {
                        'a' => Some("エイ"),
                        'i' => Some("アイ"),
                        'o' => Some("オー"),
                        'u' => Some("ユー"),
                        _ => None,
                    };
                    if let Some(p) = prefix {
                        let cut = word.len() - 3;
                        word.truncate(cut);
                        word.push_str(p);
                        word.push(c);
                    }
                }
            }
        }
    }
}

/// Fallback phonics-based converter for words not in the pre-converted dictionary
/// (slang, character elongations like 'aaaaaaa', usernames, typos).
pub fn phonics_to_katakana(raw_word: &str) -> String {
    let mut word = raw_word.to_ascii_lowercase();

    // 1. Common suffixes
    replace_table(&mut word, SUFFIX_PHRASES);
    apply_end_suffixes(&mut word);

    // 2. Digraphs & phonics combos
    replace_table(&mut word, DIGRAPHS);

    // 3. Consonant clusters
    replace_table(&mut word, CLUSTERS);

    // 4. Silent E rule: vowel + consonant + e -> long vowel
    apply_silent_e(&mut word);

    // 5. Standard CV syllables
    replace_table(&mut word, CV_TABLE);

    // 6. Remaining consonants
    replace_table(&mut word, CONSONANT_TABLE);

    // 7. Clean up remaining [a-z] Latin characters
    word.retain(|c| !c.is_ascii_lowercase());

    if word.is_empty() {
        raw_word.to_string()
    } else {
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_english_word() {
        assert_eq!(get_english_word("hello"), Some("ハロー"));
        assert_eq!(get_english_word("Hello"), Some("ハロー"));
        assert_eq!(get_english_word("HELLO"), Some("ハロー"));
        assert_eq!(get_english_word("world"), Some("ワールド"));
        assert_eq!(get_english_word("World"), Some("ワールド"));
        assert_eq!(get_english_word("streamer"), Some("ストリーマー"));
        assert_eq!(get_english_word("Streamer"), Some("ストリーマー"));
        assert_eq!(get_english_word("play"), Some("プレイ"));
        assert_eq!(get_english_word("check"), Some("チェック"));
        assert_eq!(get_english_word("doubt"), Some("ダウト"));
        assert_eq!(get_english_word("island"), Some("アイランド"));
        assert_eq!(get_english_word("sub"), Some("サブ"));
        assert_eq!(get_english_word("host"), Some("ホスト"));
        assert_eq!(get_english_word("clutch"), Some("クラッチ"));
        assert_eq!(get_english_word("nerf"), Some("ナーフ"));
        assert_eq!(get_english_word("nonexistentword12345"), None);
    }

    #[test]
    fn test_phonics_to_katakana() {
        // Repeated vowel elongation (matching spec/cases/english.json: "aaaaaaa" -> "アアアアアアア")
        assert_eq!(phonics_to_katakana("aaaaa"), "アアアアア");
        assert_eq!(phonics_to_katakana("aaaaaaa"), "アアアアアアア");

        // Silent E rule: vowel + consonant + e -> long vowel
        assert_eq!(phonics_to_katakana("cute"), "クユート");
        assert_eq!(phonics_to_katakana("make"), "ムエイク");
        assert_eq!(phonics_to_katakana("bike"), "ブアイク");
        assert_eq!(phonics_to_katakana("hope"), "ハオープ");

        // Phonics conversions strictly matching bindings/node/src/languages/english.ts
        // Note: Common words like "play" ("プレイ") and "check" ("チェック") are resolved
        // via get_english_word dictionary lookup, while phonics_to_katakana serves as the fallback.
        assert_eq!(phonics_to_katakana("fighting"), "フアイトイング");
        assert_eq!(phonics_to_katakana("play"), "プルエイ");
        assert_eq!(phonics_to_katakana("check"), "チエック");

        // Digraphs & consonant clusters
        assert_eq!(phonics_to_katakana("black"), "ブラック");
        assert_eq!(phonics_to_katakana("flash"), "フラシュ");
        assert_eq!(phonics_to_katakana("string"), "ストライング");
        assert_eq!(phonics_to_katakana("street"), "ストライート");
    }
}
