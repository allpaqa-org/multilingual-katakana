/// Check if a character is Katakana or the Japanese prolonged sound mark (ー).
#[inline]
pub fn is_katakana_or_chouon(c: char) -> bool {
    matches!(c, '\u{30A0}'..='\u{30FF}')
}

/// Check if a character is Japanese punctuation.
#[inline]
pub fn is_jp_punctuation(c: char) -> bool {
    matches!(c, '、' | '。' | '！' | '？')
}

/// Normalize prosody for Japanese TTS engines (VOICEVOX, COEIROINK, etc.):
/// - Cleans inverted punctuation
/// - Converts Western punctuation to Japanese punctuation (. at word boundary -> 。, , -> 、, ! -> ！, ? -> ？)
/// - Collapses foreign spaces between consecutive Katakana words (ハロー ガイズ -> ハローガイズ)
/// - Strips spaces surrounding Japanese punctuation
/// - Normalizes consecutive spaces
pub fn normalize_prosody(text: &str) -> String {
    let mut step1 = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    // 1. Clean inverted punctuation & normalize Western punctuation
    for i in 0..len {
        let c = chars[i];
        match c {
            '¡' | '¿' => {}
            ',' => step1.push('、'),
            '!' => step1.push('！'),
            '?' => step1.push('？'),
            '.' => {
                // Only convert period to '。' if it's at end of string or followed by whitespace
                let next_is_space_or_end = if i + 1 < len {
                    chars[i + 1].is_whitespace() || chars[i + 1] == '\u{3000}'
                } else {
                    true
                };
                if next_is_space_or_end {
                    step1.push('。');
                } else {
                    step1.push('.');
                }
            }
            _ => step1.push(c),
        }
    }

    // 2. Remove whitespace between consecutive Katakana words
    // and remove whitespace surrounding Japanese punctuation
    let step1_chars: Vec<char> = step1.chars().collect();
    let step1_len = step1_chars.len();
    let mut step2 = String::with_capacity(step1.len());

    let mut i = 0;
    while i < step1_len {
        let c = step1_chars[i];

        if c.is_whitespace() || c == '\u{3000}' {
            // Check preceding non-whitespace character
            let prev_katakana = step2
                .chars()
                .last()
                .map(is_katakana_or_chouon)
                .unwrap_or(false);
            let prev_jp_punct = step2.chars().last().map(is_jp_punctuation).unwrap_or(false);

            // Look ahead for next non-whitespace character
            let mut j = i;
            while j < step1_len && (step1_chars[j].is_whitespace() || step1_chars[j] == '\u{3000}')
            {
                j += 1;
            }

            let next_char = if j < step1_len {
                Some(step1_chars[j])
            } else {
                None
            };
            let next_katakana = next_char.map(is_katakana_or_chouon).unwrap_or(false);
            let next_jp_punct = next_char.map(is_jp_punctuation).unwrap_or(false);

            // Case A: Between Katakana words -> delete space entirely!
            if prev_katakana && next_katakana {
                i = j;
                continue;
            }

            // Case B: Around Japanese punctuation -> delete space entirely!
            if prev_jp_punct || next_jp_punct {
                i = j;
                continue;
            }

            // Otherwise, keep a single ASCII space
            if !step2.is_empty() && j < step1_len {
                step2.push(' ');
            }
            i = j;
        } else {
            step2.push(c);
            i += 1;
        }
    }

    step2.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prosody_whitespace_collapse() {
        assert_eq!(normalize_prosody("ハロー ガイズ"), "ハローガイズ");
        assert_eq!(normalize_prosody("ユー プレイ"), "ユープレイ");
    }

    #[test]
    fn test_prosody_punctuation_normalization() {
        assert_eq!(normalize_prosody("Hello, world."), "Hello、world。");
        assert_eq!(normalize_prosody("Yes! Really?"), "Yes！Really？");
        assert_eq!(normalize_prosody("3.14"), "3.14");
    }

    #[test]
    fn test_prosody_punctuation_spacing() {
        assert_eq!(normalize_prosody("ハロー 、 ガイズ 。"), "ハロー、ガイズ。");
    }
}
