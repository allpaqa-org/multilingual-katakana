use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{
    contains_sorted, lookup_sorted, FRENCH_CUES, FRENCH_CUE_READINGS, FRENCH_PHRASES, FRENCH_WORDS,
};

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

static FRENCH_ELISION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:^|[^\p{L}\p{N}_])(?:j|l|c|d|m|n|s|t|qu)['’‘][aeiouyàâéèêëîïôùûüh]|aujourd['’‘]hui",
    )
    .expect("valid regex for French elision cues")
});

static LATIN_WORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)[a-zñáéíóúüäößàâèêëîïôûùçãõìòœæ\u{0110}\u{0111}\u{1EA0}-\u{1EF9}\u{0102}\u{0103}\u{01A0}\u{01A1}\u{01AF}\u{01B0}]+('[a-z]+)?",
    )
    .expect("valid regex for Latin words")
});

fn count_french_letter_cues(text: &str) -> usize {
    text.chars()
        .filter(|c| {
            matches!(
                c,
                'ç' | 'œ'
                    | 'è'
                    | 'ë'
                    | 'ï'
                    | 'î'
                    | 'û'
                    | 'ù'
                    | 'Ç'
                    | 'Œ'
                    | 'È'
                    | 'Ë'
                    | 'Ï'
                    | 'Î'
                    | 'Û'
                    | 'Ù'
            )
        })
        .count()
}

fn count_french_elision_cues(text: &str) -> usize {
    FRENCH_ELISION_REGEX.find_iter(text).count()
}

fn count_french_word_cues(text: &str) -> usize {
    LATIN_WORD_REGEX
        .find_iter(text)
        .filter(|m| contains_sorted(FRENCH_CUES, &m.as_str().to_lowercase()))
        .count()
}

/// Detects whether an input comment contains enough French cues to activate French mode.
pub fn detect_french_mode(text: &str) -> bool {
    let hits = count_french_letter_cues(text)
        + count_french_elision_cues(text)
        + count_french_word_cues(text);
    hits >= 2
}

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

/// Looks up a French cue-word reading (e.g. le=ル, je=ジュ, tout=トゥ, que=ク, de=ドゥ).
pub fn get_french_cue_reading(word: &str) -> Option<&'static str> {
    lookup_sorted(FRENCH_CUE_READINGS, &word.to_lowercase())
}

fn apply_french_elisions(word: &mut String) {
    let prefixes: &[(&str, &str)] = &[
        ("j'a", "ジャ"),
        ("j’a", "ジャ"),
        ("j'e", "ジェ"),
        ("j’e", "ジェ"),
        ("j'i", "ジ"),
        ("j’i", "ジ"),
        ("j'o", "ジョ"),
        ("j’o", "ジョ"),
        ("j'u", "ジュ"),
        ("j’u", "ジュ"),
        ("j'", "ジュ"),
        ("j’", "ジュ"),
        ("l'a", "ラ"),
        ("l’a", "ラ"),
        ("l'e", "レ"),
        ("l’e", "レ"),
        ("l'i", "リ"),
        ("l’i", "リ"),
        ("l'o", "ロ"),
        ("l’o", "ロ"),
        ("l'u", "リュ"),
        ("l’u", "リュ"),
        ("l'", "ル"),
        ("l’", "ル"),
        ("c'", "セ"),
        ("c’", "セ"),
        ("d'", "ド"),
        ("d’", "ド"),
        ("m'", "ム"),
        ("m’", "ム"),
        ("n'", "ン"),
        ("n’", "ン"),
        ("s'", "ス"),
        ("s’", "ス"),
        ("t'", "ト"),
        ("t’", "ト"),
        ("qu'", "ク"),
        ("qu’", "ク"),
    ];
    for &(pfx, rep) in prefixes {
        if word.starts_with(pfx) {
            let rest = word[pfx.len()..].to_string();
            *word = format!("{rep}{rest}");
            break;
        }
    }
}

fn apply_french_endings(word: &mut String) {
    if word.ends_with("er") || word.ends_with("ez") {
        let len = word.len();
        if len > 2 {
            let c = word[..len - 2].chars().last();
            if let Some(ch) = c {
                if matches!(ch, 'b'..='d' | 'f'..='h' | 'j'..='n' | 'p'..='t' | 'v'..='z') {
                    word.truncate(len - 2);
                    word.push('エ');
                }
            }
        }
    }

    let ir_endings: &[(&str, &str)] = &[
        ("bir", "ビール"),
        ("dir", "ディール"),
        ("fir", "フィール"),
        ("gir", "ジール"),
        ("mir", "ミール"),
        ("nir", "ニール"),
        ("pir", "ピール"),
        ("rir", "リール"),
        ("sir", "シール"),
        ("tir", "ティール"),
        ("vir", "ヴィール"),
        ("lir", "リール"),
    ];
    for &(pfx, rep) in ir_endings {
        if word.ends_with(pfx) {
            word.truncate(word.len() - pfx.len());
            word.push_str(rep);
            break;
        }
    }

    let is_vowel = |c: char| {
        matches!(
            c,
            'a' | 'e'
                | 'i'
                | 'o'
                | 'u'
                | 'y'
                | 'à'
                | 'â'
                | 'é'
                | 'è'
                | 'ê'
                | 'ë'
                | 'î'
                | 'ï'
                | 'ô'
                | 'ù'
                | 'û'
                | 'ü'
        )
    };

    if word.ends_with("es") {
        let mut chars = word[..word.len() - 2].chars();
        if let Some(prev) = chars.next_back() {
            if is_vowel(prev) {
                word.truncate(word.len() - 2);
            }
        }
    } else if let Some(last) = word.chars().last() {
        if matches!(last, 's' | 't' | 'd' | 'x' | 'z' | 'p') {
            let mut chars = word[..word.len() - last.len_utf8()].chars();
            if let Some(prev) = chars.next_back() {
                if is_vowel(prev) {
                    word.truncate(word.len() - last.len_utf8());
                }
            }
        }
    }

    if word.ends_with('e') {
        let prefix = &word[..word.len() - 1];
        let mut has_cons = false;
        let mut has_vowel = false;
        for c in prefix.chars().rev() {
            if is_vowel(c) {
                has_vowel = true;
                break;
            } else if matches!(c, 'b'..='d' | 'f'..='h' | 'j'..='n' | 'p'..='t' | 'v'..='z') {
                has_cons = true;
            }
        }
        if has_cons && has_vowel {
            word.truncate(word.len() - 1);
        }
    }
}

static NASAL_PATTERNS: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    let ce = r"([bcdfghjklpqrstvwxz]|$)";
    let pairs: &[(String, &'static str)] = &[
        (format!("bon{ce}"), "ボン$1"),
        (format!("mon{ce}"), "モン$1"),
        (format!("ton{ce}"), "トン$1"),
        (format!("son{ce}"), "ソン$1"),
        (format!("don{ce}"), "ドン$1"),
        (format!("con{ce}"), "コン$1"),
        (format!("pon{ce}"), "ポン$1"),
        (format!("ron{ce}"), "ロン$1"),
        (format!("lon{ce}"), "ロン$1"),
        (format!("non{ce}"), "ノン$1"),
        (format!("von{ce}"), "ヴォン$1"),
        (format!("fon{ce}"), "フォン$1"),
        (format!("chon{ce}"), "ション$1"),
        (format!("jon{ce}"), "ジョン$1"),
        (format!("(?:on|om){ce}"), "オン$1"),
        (format!("b(?:an|en){ce}"), "バン$1"),
        (format!("m(?:an|en){ce}"), "マン$1"),
        (format!("t(?:an|en){ce}"), "タン$1"),
        (format!("s(?:an|en){ce}"), "サン$1"),
        (format!("d(?:an|en){ce}"), "ダン$1"),
        (format!("p(?:an|en){ce}"), "パン$1"),
        (format!("r(?:an|en){ce}"), "ラン$1"),
        (format!("l(?:an|en){ce}"), "ラン$1"),
        (format!("n(?:an|en){ce}"), "ナン$1"),
        (format!("v(?:an|en){ce}"), "ヴァン$1"),
        (format!("f(?:an|en){ce}"), "ファン$1"),
        (format!("ch(?:an|en){ce}"), "シャン$1"),
        (format!("j(?:an|en){ce}"), "ジャン$1"),
        (format!("(?:an|en|am|em){ce}"), "アン$1"),
        (format!("b(?:in|ain|ein|un){ce}"), "バン$1"),
        (format!("m(?:in|ain|ein|un){ce}"), "マン$1"),
        (format!("t(?:in|ain|ein|un){ce}"), "タン$1"),
        (format!("s(?:in|ain|ein|un){ce}"), "サン$1"),
        (format!("d(?:in|ain|ein|un){ce}"), "ダン$1"),
        (format!("p(?:in|ain|ein|un){ce}"), "パン$1"),
        (format!("r(?:in|ain|ein|un){ce}"), "ラン$1"),
        (format!("l(?:in|ain|ein|un){ce}"), "ラン$1"),
        (format!("v(?:in|ain|ein|un){ce}"), "ヴァン$1"),
        (format!("f(?:in|ain|ein|un){ce}"), "ファン$1"),
        (format!("(?:in|im|ain|ein|un|um){ce}"), "アン$1"),
    ];
    pairs
        .iter()
        .map(|(p, rep)| (Regex::new(p).expect("valid nasal regex"), *rep))
        .collect()
});

fn apply_french_nasals(word: &mut String) {
    for (re, rep) in NASAL_PATTERNS.iter() {
        if re.is_match(word) {
            *word = re.replace_all(word, *rep).into_owned();
        }
    }
}

static VOWEL_TABLE: &[(&str, &str)] = &[
    ("vai", "ヴェ"),
    ("fai", "フェ"),
    ("bai", "ベ"),
    ("dai", "デ"),
    ("mai", "メ"),
    ("nai", "ネ"),
    ("pai", "ペ"),
    ("rai", "レ"),
    ("sai", "セ"),
    ("tai", "テ"),
    ("cai", "セ"),
    ("cei", "セ"),
    ("gai", "ジェ"),
    ("gei", "ジェ"),
    ("ai", "エ"),
    ("ei", "エ"),
    ("eau", "オ"),
    ("au", "オ"),
    ("ou", "ウ"),
    ("croi", "クロワ"),
    ("troi", "トロワ"),
    ("droi", "ドロワ"),
    ("broi", "ブロワ"),
    ("proi", "プロワ"),
    ("moi", "モワ"),
    ("toi", "トワ"),
    ("soi", "ソワ"),
    ("voi", "ヴォワ"),
    ("roi", "ロワ"),
    ("loi", "ロワ"),
    ("joi", "ジョワ"),
    ("choi", "ショワ"),
    ("boi", "ボワ"),
    ("poi", "ポワ"),
    ("doi", "ドワ"),
    ("oi", "ワ"),
    ("jeu", "ジュ"),
    ("cheu", "シュ"),
    ("veu", "ヴ"),
    ("beu", "ブ"),
    ("peu", "プ"),
    ("meu", "ム"),
    ("neu", "ヌ"),
    ("eu", "ウ"),
    ("œu", "ウ"),
    ("œ", "ウ"),
    ("drô", "ドロー"),
    ("trô", "トロー"),
    ("é", "エ"),
    ("è", "エ"),
    ("ê", "エ"),
    ("ë", "エ"),
    ("à", "ア"),
    ("â", "ア"),
    ("î", "イ"),
    ("ï", "イ"),
    ("ô", "オ"),
    ("ö", "オ"),
    ("û", "ュ"),
    ("ü", "ュ"),
    ("ù", "ュ"),
    ("ç", "ス"),
];

static ENDING_OR_TABLE: &[(&str, &str)] = &[
    ("dor", "ドール"),
    ("tor", "トール"),
    ("por", "ポール"),
    ("cor", "コール"),
    ("mor", "モール"),
];

fn apply_french_vowels(word: &mut String) {
    for &(pat, rep) in VOWEL_TABLE {
        if word.contains(pat) {
            *word = word.replace(pat, rep);
        }
    }
    for &(pat, rep) in ENDING_OR_TABLE {
        if word.ends_with(pat) {
            let cut = word.len() - pat.len();
            word.truncate(cut);
            word.push_str(rep);
            break;
        }
    }
}

static CONSONANT_COMBO_TABLE: &[(&str, &str)] = &[
    ("ch", "シュ"),
    ("gn", "ニュ"),
    ("qu", "ク"),
    ("ph", "フ"),
    ("th", "ト"),
    ("ll", "ル"),
    ("h", ""),
    ("cr", "クロ"),
    ("dr", "ドロ"),
    ("tr", "トロ"),
    ("br", "ブロ"),
    ("pr", "プロ"),
    ("gr", "グロ"),
    ("fr", "フロ"),
    ("vr", "ヴロ"),
    ("cl", "クラ"),
    ("bl", "ブラ"),
    ("fl", "フラ"),
    ("gl", "グラ"),
    ("pl", "プラ"),
];

fn apply_french_consonants(word: &mut String) {
    for &(pat, rep) in CONSONANT_COMBO_TABLE {
        if word.contains(pat) {
            *word = word.replace(pat, rep);
        }
    }
    // Soft c and g before e/i/y/katakana
    let soft_c_pairs = [
        ("ce", "セe"),
        ("ci", "セi"),
        ("cy", "セy"),
        ("cエ", "セエ"),
        ("cイ", "セイ"),
    ];
    for (pat, rep) in soft_c_pairs {
        if word.contains(pat) {
            *word = word.replace(pat, rep);
        }
    }
    let soft_g_pairs = [
        ("ge", "ジェe"),
        ("gi", "ジェi"),
        ("gy", "ジェy"),
        ("gエ", "ジェエ"),
        ("gイ", "ジェイ"),
    ];
    for (pat, rep) in soft_g_pairs {
        if word.contains(pat) {
            *word = word.replace(pat, rep);
        }
    }
}

static CV_TABLE: &[(&str, &str)] = &[
    ("ja", "ジャ"),
    ("ju", "ジュ"),
    ("jo", "ジョ"),
    ("ji", "ジ"),
    ("je", "ジェ"),
    ("j", "ジュ"),
    ("va", "ヴァ"),
    ("vi", "ヴィ"),
    ("vu", "ヴュ"),
    ("ve", "ヴェ"),
    ("vo", "ヴォ"),
    ("v", "ヴ"),
    ("ka", "カ"),
    ("ki", "キ"),
    ("ku", "ク"),
    ("ke", "ケ"),
    ("ko", "コ"),
    ("ca", "カ"),
    ("cu", "キュ"),
    ("co", "コ"),
    ("sa", "サ"),
    ("si", "シ"),
    ("su", "ス"),
    ("se", "セ"),
    ("so", "ソ"),
    ("ta", "タ"),
    ("ti", "ティ"),
    ("tu", "テュ"),
    ("te", "テ"),
    ("to", "ト"),
    ("na", "ナ"),
    ("ni", "ニ"),
    ("nu", "ニュ"),
    ("ne", "ネ"),
    ("no", "ノ"),
    ("ma", "マ"),
    ("mi", "ミ"),
    ("mu", "ミュ"),
    ("me", "メ"),
    ("mo", "モ"),
    ("ra", "ラ"),
    ("ri", "リ"),
    ("ru", "リュ"),
    ("re", "レ"),
    ("ro", "ロ"),
    ("la", "ラ"),
    ("li", "リ"),
    ("lu", "リュ"),
    ("le", "レ"),
    ("lo", "ロ"),
    ("ga", "ガ"),
    ("gu", "ギュ"),
    ("go", "ゴ"),
    ("ba", "バ"),
    ("bi", "ビ"),
    ("bu", "ビュ"),
    ("be", "ベ"),
    ("bo", "ボ"),
    ("pa", "パ"),
    ("pi", "ピ"),
    ("pu", "ピュ"),
    ("pe", "ペ"),
    ("po", "ポ"),
    ("da", "ダ"),
    ("di", "ディ"),
    ("du", "デュ"),
    ("de", "デ"),
    ("do", "ド"),
    ("fa", "ファ"),
    ("fi", "フィ"),
    ("fu", "フュ"),
    ("fe", "フェ"),
    ("fo", "フォ"),
    ("za", "ザ"),
    ("zi", "ジ"),
    ("zu", "ズ"),
    ("ze", "ゼ"),
    ("zo", "ゾ"),
    ("wa", "ワ"),
    ("a", "ア"),
    ("i", "イ"),
    ("u", "ュ"),
    ("e", "エ"),
    ("o", "オ"),
    ("y", "イ"),
];

static STANDALONE_CONSONANTS: &[(&str, &str)] = &[
    ("b", "ブ"),
    ("c", "ク"),
    ("d", "ド"),
    ("f", "フ"),
    ("g", "グ"),
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
    ("x", "クス"),
    ("z", "ズ"),
];

fn apply_french_syllables(word: &mut String) {
    for &(pat, rep) in CV_TABLE {
        if word.contains(pat) {
            *word = word.replace(pat, rep);
        }
    }
    for &(pat, rep) in STANDALONE_CONSONANTS {
        if word.contains(pat) {
            *word = word.replace(pat, rep);
        }
    }
}

/// Fallback French phonetic rules for dictionary misses in French mode.
pub fn french_phonics_to_katakana(raw_word: &str) -> String {
    let mut word = raw_word.to_ascii_lowercase();
    apply_french_elisions(&mut word);
    apply_french_endings(&mut word);
    apply_french_nasals(&mut word);
    apply_french_vowels(&mut word);
    apply_french_consonants(&mut word);
    apply_french_syllables(&mut word);
    if word.contains("ルエ") {
        word = word.replace("ルエ", "レ");
    }
    while word.contains("ーー") {
        word = word.replace("ーー", "ー");
    }

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
    fn test_replace_french_phrases() {
        assert_eq!(
            replace_french_phrases("Bonjour mon ami! Merci beaucoup."),
            "ボンジュールモナミ! メルシーボクー."
        );
        assert_eq!(replace_french_phrases("S'il vous plaît"), "シルヴプレ");
        assert_eq!(
            replace_french_phrases("Bonsoir à tous!"),
            "ボンソワールアトゥ!"
        );
    }

    #[test]
    fn test_get_french_word() {
        assert_eq!(get_french_word("bonjour"), Some("ボンジュール"));
        assert_eq!(get_french_word("AUJOURD'HUI"), Some("オジュルデュイ"));
        assert_eq!(get_french_word("coucou"), Some("ククー"));
        assert_eq!(get_french_word("hello"), None);
    }

    #[test]
    fn test_french_mode_detection() {
        assert!(detect_french_mode("Salut tout le monde"));
        assert!(detect_french_mode("J'adore ce jeu"));
        assert!(detect_french_mode("mdr trop drôle"));
        assert!(detect_french_mode(
            "Je crois que je vais aller dormir aujourd'hui."
        ));
        assert!(!detect_french_mode("I love this stream"));
        assert!(!detect_french_mode("Let's go le stream"));
        assert!(!detect_french_mode("Good job on the boss"));
        assert!(!detect_french_mode("Hola, que tal? Me gusta mucho"));
        assert!(!detect_french_mode("Xin chào, bạn"));
    }

    #[test]
    fn test_french_phonics() {
        assert_eq!(french_phonics_to_katakana("j'adore"), "ジャドール");
        assert_eq!(french_phonics_to_katakana("monde"), "モンド");
        assert_eq!(french_phonics_to_katakana("drôle"), "ドロール");
        assert_eq!(french_phonics_to_katakana("jeu"), "ジュ");
        assert_eq!(french_phonics_to_katakana("crois"), "クロワ");
        assert_eq!(french_phonics_to_katakana("vais"), "ヴェ");
        assert_eq!(french_phonics_to_katakana("aller"), "アレ");
        assert_eq!(french_phonics_to_katakana("dormir"), "ドルミール");
    }
}
