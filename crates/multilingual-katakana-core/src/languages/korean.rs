use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{KOREAN_CHOSUNG_JUNGSUNG, KOREAN_JONGSUNG, KOREAN_PHRASES};

static PUNCT_SPACE_BEFORE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s\u{3000}]+([、。！？!?,.])").expect("Invalid regex"));

static MULTI_SPACE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s\u{3000}]+").expect("Invalid regex"));

/// Returns true if text contains any Hangul syllable (`\u{AC00}..=\u{D7AF}`)
/// or Hangul Jamo (`\u{1100}..=\u{11FF}`, `\u{3130}..=\u{318F}`).
pub fn is_korean(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(
            c,
            '\u{AC00}'..='\u{D7AF}' | '\u{1100}'..='\u{11FF}' | '\u{3130}'..='\u{318F}'
        )
    })
}

/// Decomposes a Hangul syllable character into Chosung (l), Jungsung (v), and Jongsung (t) indices.
///
/// Hangul syllable formula:
/// `code = 0xAC00 + (l * 21 + v) * 28 + t`
/// - `l` in `0..19` (19 initial consonants)
/// - `v` in `0..21` (21 medial vowels)
/// - `t` in `0..28` (28 final consonants / batchim; 0 means none)
pub fn decompose_hangul_syllable(c: char) -> Option<(usize, usize, usize)> {
    let code = c as u32;
    if (0xAC00..=0xD7A3).contains(&code) {
        let idx = code - 0xAC00;
        let l = (idx / 588) as usize;
        let v = ((idx % 588) / 28) as usize;
        let t = (idx % 28) as usize;
        Some((l, v, t))
    } else {
        None
    }
}

/// Resolves Korean liaison (連音化) transfer when a batchim is followed by silent initial ㅇ (l == 11).
/// Returns `(remaining_batchim_t, transferred_chosung_l)`.
fn resolve_liaison(t: usize) -> (usize, Option<usize>) {
    match t {
        1 => (0, Some(0)),   // ㄱ -> ㄱ
        2 => (0, Some(1)),   // ㄲ -> ㄲ
        3 => (1, Some(9)),   // ㄳ -> ㄱ stays, ㅅ moves
        4 => (0, Some(2)),   // ㄴ -> ㄴ
        5 => (4, Some(12)),  // ㄵ -> ㄴ stays, ㅈ moves
        6 => (4, None),      // ㄶ -> ㄴ stays, ㅎ drops
        7 => (0, Some(3)),   // ㄷ -> ㄷ
        8 => (0, Some(5)),   // ㄹ -> ㄹ
        9 => (8, Some(0)),   // ㄺ -> ㄹ stays, ㄱ moves
        10 => (8, Some(6)),  // ㄻ -> ㄹ stays, ㅁ moves
        11 => (8, Some(7)),  // ㄼ -> ㄹ stays, ㅂ moves
        12 => (8, Some(9)),  // ㄽ -> ㄹ stays, ㅅ moves
        13 => (8, Some(16)), // ㄾ -> ㄹ stays, ㅌ moves
        14 => (8, Some(17)), // ㄿ -> ㄹ stays, ㅍ moves
        15 => (8, None),     // ㅀ -> ㄹ stays, ㅎ drops
        16 => (0, Some(6)),  // ㅁ -> ㅁ
        17 => (0, Some(7)),  // ㅂ -> ㅂ
        18 => (17, Some(9)), // ㅄ -> ㅂ stays, ㅅ moves
        19 => (0, Some(9)),  // ㅅ -> ㅅ
        20 => (0, Some(10)), // ㅆ -> ㅆ moves (row 10 chosung includes sokuon ッ)
        21 => (21, None),    // ㅇ -> stays as ng
        22 => (0, Some(12)), // ㅈ -> ㅈ
        23 => (0, Some(14)), // ㅊ -> ㅊ
        24 => (0, Some(15)), // ㅋ -> ㅋ
        25 => (0, Some(16)), // ㅌ -> ㅌ
        26 => (0, Some(17)), // ㅍ -> ㅍ
        27 => (0, None),     // ㅎ -> drops before vowel
        _ => (t, None),
    }
}

/// Composes consecutive Hangul Jamo (NFD decomposed characters) into precomposed Hangul syllables (NFC).
pub fn compose_hangul_jamo(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];
        let code = c as u32;

        // Check if c is Chosung (Initial Jamo: 0x1100..=0x1112)
        if (0x1100..=0x1112).contains(&code) && i + 1 < len {
            let next_code = chars[i + 1] as u32;
            // Check if next is Jungsung (Medial Jamo: 0x1161..=0x1175)
            if (0x1161..=0x1175).contains(&next_code) {
                let l = code - 0x1100;
                let v = next_code - 0x1161;
                let mut t = 0;
                let mut advance = 2;

                // Check if followed by Jongsung (Trailing Jamo: 0x11A8..=0x11C2)
                if i + 2 < len {
                    let trailing_code = chars[i + 2] as u32;
                    if (0x11A8..=0x11C2).contains(&trailing_code) {
                        t = trailing_code - 0x11A7;
                        advance = 3;
                    }
                }

                let syllable_code = 0xAC00 + (l * 21 + v) * 28 + t;
                if let Some(syllable_char) = char::from_u32(syllable_code) {
                    result.push(syllable_char);
                    i += advance;
                    continue;
                }
            }
        }

        result.push(c);
        i += 1;
    }

    result
}

/// Converts Korean text to Katakana for Japanese TTS.
pub fn convert_korean(text: &str) -> String {
    if !is_korean(text) {
        return text.to_string();
    }

    // Step 0: Compose any NFD Hangul Jamo sequences into precomposed syllables
    let composed = compose_hangul_jamo(text);

    // Step 1: Replace common phrases in KOREAN_PHRASES (already sorted by length descending)
    let mut processed = composed;
    for &(phrase, katakana) in KOREAN_PHRASES {
        if processed.contains(phrase) {
            processed = processed.replace(phrase, katakana);
        }
    }

    // Step 2: Mathematical phonetic decomposition on remaining characters with liaison
    #[derive(Clone, Copy)]
    struct SyllableItem {
        orig: char,
        syllable: Option<(usize, usize, usize)>,
    }

    let mut items: Vec<SyllableItem> = processed
        .chars()
        .map(|c| SyllableItem {
            orig: c,
            syllable: decompose_hangul_syllable(c),
        })
        .collect();

    let len = items.len();
    for i in 0..len {
        if let Some((l, v, t)) = items[i].syllable {
            if t > 0 && i + 1 < len {
                if let Some((next_l, next_v, next_t)) = items[i + 1].syllable {
                    if next_l == 11 {
                        let (rem_t, new_next_l) = resolve_liaison(t);
                        items[i].syllable = Some((l, v, rem_t));
                        if let Some(transferred_l) = new_next_l {
                            items[i + 1].syllable = Some((transferred_l, next_v, next_t));
                        }
                    }
                }
            }
        }
    }

    let mut result = String::with_capacity(processed.len() * 3);
    for item in items {
        if let Some((l, v, t)) = item.syllable {
            let base = KOREAN_CHOSUNG_JUNGSUNG
                .get(l)
                .and_then(|row| row.get(v))
                .copied()
                .unwrap_or("");
            let batchim = KOREAN_JONGSUNG.get(t).copied().unwrap_or("");
            result.push_str(base);
            result.push_str(batchim);
        } else {
            result.push(item.orig);
        }
    }

    // Step 3: Remove unnecessary spaces before punctuation
    let result = PUNCT_SPACE_BEFORE_RE.replace_all(&result, "$1");

    // Step 4: Normalize spaces: replace multiple whitespace/full-width space with single space and trim
    MULTI_SPACE_RE.replace_all(&result, " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_korean() {
        assert!(is_korean("안녕하세요"));
        assert!(is_korean("감사합니다"));
        assert!(is_korean("대박"));
        assert!(is_korean("ㄱ"));
        assert!(!is_korean("Hello"));
        assert!(!is_korean("こんにちは"));
        assert!(!is_korean("了解"));
    }

    #[test]
    fn test_decompose_hangul_syllable() {
        // 방: ㅂ (7), ㅏ (0), ㅇ (21)
        assert_eq!(decompose_hangul_syllable('방'), Some((7, 0, 21)));
        // 송: ㅅ (9), ㅗ (8), ㅇ (21)
        assert_eq!(decompose_hangul_syllable('송'), Some((9, 8, 21)));
        // Non-Hangul
        assert_eq!(decompose_hangul_syllable('A'), None);
        assert_eq!(decompose_hangul_syllable('あ'), None);
    }

    #[test]
    fn test_korean_unit_cases() {
        assert_eq!(convert_korean("안녕하세요"), "アンニョンハセヨ");
        assert_eq!(convert_korean("감사합니다"), "カムサハムニダ");
        assert_eq!(convert_korean("사랑해"), "サランヘ");
        assert_eq!(convert_korean("대박"), "テバク");
        assert_eq!(convert_korean("맛있어요"), "マシッソヨ");
        assert_eq!(convert_korean("화이팅"), "ファイティン");
    }

    #[test]
    fn test_nfd_hangul_composition() {
        // NFD representation of "안녕" (ㅇ ㅏ ㄴ, ㄴ ㅕ ㅇ)
        let nfd = "\u{110B}\u{1161}\u{11AB}\u{1102}\u{1167}\u{11BC}";
        assert_eq!(convert_korean(nfd), "アンニョン");
    }

    #[test]
    fn test_spec_cases_korean() {
        assert_eq!(convert_korean("안녕"), "アンニョン");
        assert_eq!(convert_korean("고마워"), "コマウォ");
        assert_eq!(convert_korean("죄송합니다"), "チェソンハムニダ");
        assert_eq!(convert_korean("미안해"), "ミアネ");
        assert_eq!(convert_korean("사랑해요"), "サランヘヨ");
        assert_eq!(convert_korean("수고했어요"), "スゴヘッソヨ");
        assert_eq!(convert_korean("파이팅"), "パイティン");
        assert_eq!(convert_korean("방송"), "パンソン");
        assert_eq!(
            convert_korean("안녕하세요! 방송 너무 재미있어요 화이팅!"),
            "アンニョンハセヨ! パンソン ノム チェミイッソヨ ファイティン!"
        );
        assert_eq!(
            convert_korean("진짜 대박 잘자요"),
            "チンチャ テバク チャルジャヨ"
        );
    }

    #[test]
    fn test_liaison_cases() {
        assert_eq!(convert_korean("맛있어"), "マシッソ");
        assert_eq!(convert_korean("좋아"), "チョア");
        assert_eq!(convert_korean("음악"), "ウマク");
    }
}
