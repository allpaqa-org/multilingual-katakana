use crate::dicts::THAI_PHRASES;

/// Returns true if text contains any Thai script character (`\u{0E00}..=\u{0E7F}`).
pub fn is_thai(text: &str) -> bool {
    text.chars().any(|c| ('\u{0E00}'..='\u{0E7F}').contains(&c))
}

/// Replaces known Thai phrases/words (greetings, thanks, streaming slang) with Katakana.
/// `THAI_PHRASES` is sorted by length descending so longer phrases win over their prefixes.
pub fn replace_thai_phrases(text: &str) -> String {
    let mut result = text.to_string();
    for &(phrase, katakana) in THAI_PHRASES {
        if result.contains(phrase) {
            result = result.replace(phrase, katakana);
        }
    }
    result
}

/// Katakana row for a consonant sound class, indexed by vowel column:
/// `[a, i, ue, u, e, ae, o, aw]`
type Row = [&'static str; 8];

const ROW_K: Row = ["カ", "キ", "ク", "ク", "ケ", "ケ", "コ", "コ"];
const ROW_NG: Row = ["ガ", "ギ", "グ", "グ", "ゲ", "ゲ", "ゴ", "ゴ"];
const ROW_J: Row = ["ジャ", "ジ", "ジュ", "ジュ", "ジェ", "ジェ", "ジョ", "ジョ"];
const ROW_CH: Row = ["チャ", "チ", "チュ", "チュ", "チェ", "チェ", "チョ", "チョ"];
const ROW_S: Row = ["サ", "シ", "ス", "ス", "セ", "セ", "ソ", "ソ"];
const ROW_Y: Row = ["ヤ", "イ", "ユ", "ユ", "イェ", "イェ", "ヨ", "ヨ"];
const ROW_D: Row = ["ダ", "ディ", "ドゥ", "ドゥ", "デ", "デ", "ド", "ド"];
const ROW_T: Row = ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"];
const ROW_N: Row = ["ナ", "ニ", "ヌ", "ヌ", "ネ", "ネ", "ノ", "ノ"];
const ROW_B: Row = ["バ", "ビ", "ブ", "ブ", "ベ", "ベ", "ボ", "ボ"];
const ROW_P: Row = ["パ", "ピ", "プ", "プ", "ペ", "ペ", "ポ", "ポ"];
const ROW_F: Row = ["ファ", "フィ", "フ", "フ", "フェ", "フェ", "フォ", "フォ"];
const ROW_M: Row = ["マ", "ミ", "ム", "ム", "メ", "メ", "モ", "モ"];
const ROW_R: Row = ["ラ", "リ", "ル", "ル", "レ", "レ", "ロ", "ロ"];
const ROW_W: Row = ["ワ", "ウィ", "ウ", "ウ", "ウェ", "ウェ", "ウォ", "ウォ"];
const ROW_H: Row = ["ハ", "ヒ", "フ", "フ", "ヘ", "ヘ", "ホ", "ホ"];
const ROW_NULL: Row = ["ア", "イ", "ウ", "ウ", "エ", "エ", "オ", "オ"];

/// Maps a Thai initial consonant character to its approximate katakana onset row.
/// Aspiration/consonant-class distinctions (e.g. ข vs ค, ผ vs พ) are collapsed since
/// Katakana cannot represent them; this is an intentional, documented simplification.
fn consonant_row(c: char) -> Option<Row> {
    match c {
        'ก' | 'ข' | 'ฃ' | 'ค' | 'ฅ' | 'ฆ' => Some(ROW_K),
        'ง' => Some(ROW_NG),
        'จ' => Some(ROW_J),
        'ฉ' | 'ช' | 'ฌ' => Some(ROW_CH),
        'ซ' | 'ศ' | 'ษ' | 'ส' => Some(ROW_S),
        'ญ' | 'ย' => Some(ROW_Y),
        'ฎ' | 'ด' => Some(ROW_D),
        'ฏ' | 'ต' | 'ฐ' | 'ฑ' | 'ฒ' | 'ถ' | 'ท' | 'ธ' => Some(ROW_T),
        'ณ' | 'น' => Some(ROW_N),
        'บ' => Some(ROW_B),
        'ป' | 'ผ' | 'พ' | 'ภ' => Some(ROW_P),
        'ฝ' | 'ฟ' => Some(ROW_F),
        'ม' => Some(ROW_M),
        'ร' | 'ล' | 'ฬ' => Some(ROW_R),
        'ว' => Some(ROW_W),
        'ห' | 'ฮ' => Some(ROW_H),
        'อ' => Some(ROW_NULL),
        _ => None,
    }
}

fn is_leading_vowel(c: char) -> bool {
    matches!(c, 'เ' | 'แ' | 'โ' | 'ใ' | 'ไ')
}

fn is_thai_char(c: char) -> bool {
    ('\u{0E00}'..='\u{0E7F}').contains(&c)
}

fn is_tone_mark(c: char) -> bool {
    matches!(c, '\u{0E48}'..='\u{0E4B}')
}

/// Renders one katakana mora from a consonant row + vowel column, optionally lengthened.
fn render(row: Row, col: usize, elongate: bool) -> String {
    let base = row[col];
    if elongate {
        format!("{base}ー")
    } else {
        base.to_string()
    }
}

/// Best-effort syllable-level Katakana rendering for Thai text left over after phrase
/// substitution. Thai script has no spaces between words and a following consonant can
/// be either the final of the current syllable or the onset of the next one; since we
/// have no dictionary-based word segmentation, any syllable followed by such an
/// ambiguous plain consonant is left completely untouched (Safe Failure) rather than
/// guessed. Only unambiguous, self-contained syllables (open syllables bounded by
/// whitespace/punctuation/another leading vowel/end of string) are converted.
pub fn convert_thai_syllables(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut out = String::with_capacity(text.len() * 2);
    let mut i = 0;

    while i < len {
        let start = i;
        let mut c = chars[i];

        let mut leading: Option<char> = None;
        if is_leading_vowel(c) {
            if i + 1 >= len || consonant_row(chars[i + 1]).is_none() {
                out.push(c);
                i += 1;
                continue;
            }
            leading = Some(c);
            i += 1;
            c = chars[i];
        }

        let Some(row) = consonant_row(c) else {
            out.push(chars[start]);
            i = start + 1;
            continue;
        };
        i += 1;

        while i < len && is_tone_mark(chars[i]) {
            i += 1;
        }

        // vowel_col: 0=a 1=i 2=ue 3=u 4=e 5=ae 6=o 7=aw; diphthong is rendered directly.
        let mut vowel_col: Option<usize> = None;
        let mut elongate = false;
        let mut diphthong: Option<&str> = None;

        match leading {
            Some('ใ') | Some('ไ') => {
                diphthong = Some("イ");
            }
            Some(lv @ ('เ' | 'แ' | 'โ')) => {
                if i < len && chars[i] == 'ะ' {
                    i += 1;
                    vowel_col = Some(match lv {
                        'เ' => 4,
                        'แ' => 5,
                        _ => 6,
                    });
                } else if lv == 'เ' && i < len && chars[i] == 'า' {
                    i += 1;
                    diphthong = Some("オ");
                } else {
                    vowel_col = Some(match lv {
                        'เ' => 4,
                        'แ' => 5,
                        _ => 6,
                    });
                    elongate = true;
                }
            }
            _ => {
                if i < len {
                    match chars[i] {
                        'า' => {
                            vowel_col = Some(0);
                            elongate = true;
                            i += 1;
                        }
                        'ั' => {
                            vowel_col = Some(0);
                            i += 1;
                        }
                        'ิ' => {
                            vowel_col = Some(1);
                            i += 1;
                        }
                        'ี' => {
                            vowel_col = Some(1);
                            elongate = true;
                            i += 1;
                        }
                        'ึ' => {
                            vowel_col = Some(2);
                            i += 1;
                        }
                        'ื' => {
                            vowel_col = Some(2);
                            elongate = true;
                            i += 1;
                        }
                        'ุ' => {
                            vowel_col = Some(3);
                            i += 1;
                        }
                        'ู' => {
                            vowel_col = Some(3);
                            elongate = true;
                            i += 1;
                        }
                        'อ' => {
                            vowel_col = Some(7);
                            elongate = true;
                            i += 1;
                        }
                        'ะ' => {
                            vowel_col = Some(0);
                            i += 1;
                        }
                        _ => {}
                    }
                }
            }
        }

        while i < len && is_tone_mark(chars[i]) {
            i += 1;
        }

        // Ambiguous trailing plain consonant: could be this syllable's final or the next
        // syllable's onset. We can't disambiguate without word segmentation, so bail out
        // and emit the entire contiguous Thai-script run untouched (Safe Failure) instead
        // of reprocessing the ambiguous consonant as a fresh syllable start.
        if i < len && !is_leading_vowel(chars[i]) && consonant_row(chars[i]).is_some() {
            let mut j = i;
            while j < len && is_thai_char(chars[j]) {
                j += 1;
            }
            out.extend(&chars[start..j]);
            i = j;
            continue;
        }

        if let Some(suffix) = diphthong {
            out.push_str(row[0]);
            out.push_str(suffix);
        } else {
            let col = vowel_col.unwrap_or(0); // no vowel sign at all -> implicit short "a"
            out.push_str(&render(row, col, elongate));
        }
    }

    out
}

/// Converts Thai text to Katakana for Japanese TTS: known phrases first, then a
/// conservative syllable-level fallback for the remainder.
pub fn convert_thai(text: &str) -> String {
    let phrased = replace_thai_phrases(text);
    if !is_thai(&phrased) {
        return phrased;
    }
    convert_thai_syllables(&phrased)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_thai() {
        assert!(is_thai("สวัสดี"));
        assert!(!is_thai("Hello"));
        assert!(!is_thai("こんにちは"));
    }

    #[test]
    fn test_replace_thai_phrases() {
        assert_eq!(replace_thai_phrases("สวัสดีครับ"), "サワッディークラップ");
        assert_eq!(replace_thai_phrases("ขอบคุณ"), "コープクン");
    }

    #[test]
    fn test_convert_thai_open_syllables() {
        assert_eq!(convert_thai("จ้า"), "チャー");
        assert_eq!(convert_thai("ค่ะ"), "カー");
        assert_eq!(convert_thai("นะ"), "ナ");
        assert_eq!(convert_thai("ไป"), "パイ");
        assert_eq!(convert_thai("ใจ"), "ジャイ");
    }

    #[test]
    fn test_convert_thai_phrase_then_syllable() {
        assert_eq!(convert_thai("ขอบคุณค่ะ"), "コープクンカー");
    }

    #[test]
    fn test_convert_thai_ambiguous_passthrough() {
        // "คน" (khon, person) has no phrase entry and its final consonant "น" is
        // ambiguous without word segmentation, so it must remain untouched.
        assert_eq!(convert_thai("คน"), "คน");
    }

    #[test]
    fn test_convert_thai_non_thai_passthrough() {
        assert_eq!(convert_thai("Hello world"), "Hello world");
    }
}
