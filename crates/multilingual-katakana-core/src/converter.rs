use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::sync::LazyLock;

use crate::languages::{
    chinese::convert_chinese,
    cyrillic::convert_cyrillic,
    english::{get_english_word, phonics_to_katakana},
    french::{get_french_word, replace_french_phrases},
    korean::convert_korean,
    slang::{get_slang_word, replace_slang_phrases},
    spanish::{get_spanish_word, replace_spanish_phrases, spanish_preprocess},
    thai::convert_thai,
    vietnamese::{get_vietnamese_word, replace_vietnamese_phrases, vietnamese_preprocess},
};
use crate::normalizers::normalize_prosody;
use crate::types::{ExcludePattern, KatakanaOptions};

static WORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)[a-zñáéíóúüäößàâèêëîïôûùçãõìòœæ\u{0110}\u{0111}\u{1EA0}-\u{1EF9}\u{0102}\u{0103}\u{01A0}\u{01A1}\u{01AF}\u{01B0}]+('[a-z]+)?",
    )
    .expect("Valid regex")
});

#[derive(Debug, Clone)]
struct MatchInterval {
    start: usize,
    end: usize,
    text: String,
}

fn collect_intervals(text: &str, exclude: &[ExcludePattern]) -> Vec<MatchInterval> {
    let mut intervals = Vec::new();
    for pattern in exclude {
        match pattern {
            ExcludePattern::Literal(s) => {
                if !s.is_empty() {
                    for (start, matched) in text.match_indices(s) {
                        intervals.push(MatchInterval {
                            start,
                            end: start + matched.len(),
                            text: matched.to_string(),
                        });
                    }
                }
            }
            ExcludePattern::Regex(re) => {
                for mat in re.find_iter(text) {
                    if !mat.as_str().is_empty() {
                        intervals.push(MatchInterval {
                            start: mat.start(),
                            end: mat.end(),
                            text: mat.as_str().to_string(),
                        });
                    }
                }
            }
        }
    }
    intervals
}

fn resolve_intervals(mut intervals: Vec<MatchInterval>) -> Vec<MatchInterval> {
    intervals.sort_by(|a, b| {
        a.start
            .cmp(&b.start)
            .then_with(|| (b.end - b.start).cmp(&(a.end - a.start)))
    });

    let mut result = Vec::new();
    let mut last_end = 0;
    for iv in intervals {
        if iv.start >= last_end {
            last_end = iv.end;
            result.push(iv);
        }
    }
    result
}

/// Escape excluded strings/regex matches using Unicode Private Use Area (U+E000..U+F8FF).
pub fn escape_excluded(text: &str, exclude: &[ExcludePattern]) -> (String, BTreeMap<char, String>) {
    if exclude.is_empty() {
        return (text.to_string(), BTreeMap::new());
    }

    let mut existing_pua = HashSet::new();
    for c in text.chars() {
        let code = c as u32;
        if (0xE000..=0xF8FF).contains(&code) {
            existing_pua.insert(code);
        }
    }

    let intervals = resolve_intervals(collect_intervals(text, exclude));
    let mut token_map = BTreeMap::new();
    let mut result = String::with_capacity(text.len());
    let mut last_index = 0;
    let mut next_pua_code: u32 = 0xE000;

    for iv in intervals {
        result.push_str(&text[last_index..iv.start]);

        while existing_pua.contains(&next_pua_code) {
            next_pua_code += 1;
        }

        if next_pua_code > 0xF8FF {
            panic!("Exceeded maximum number of protectable tokens in PUA range (U+E000 - U+F8FF)");
        }

        if let Some(token_char) = char::from_u32(next_pua_code) {
            existing_pua.insert(next_pua_code);
            next_pua_code += 1;

            token_map.insert(token_char, iv.text);
            result.push(token_char);
        }

        last_index = iv.end;
    }
    result.push_str(&text[last_index..]);

    (result, token_map)
}

/// Restore original text from PUA tokens.
pub fn restore_excluded(text: &str, token_map: &BTreeMap<char, String>) -> String {
    if token_map.is_empty() {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        if let Some(original) = token_map.get(&c) {
            result.push_str(original);
        } else {
            result.push(c);
        }
    }
    result
}

/// Resolves a single word via slang, Spanish/Vietnamese dictionaries, English word
/// dictionary, or phonics fallback.
pub fn resolve_word(word: &str, opts: &KatakanaOptions) -> String {
    let lower = word.to_lowercase();

    // 1. Check special slang first (gg, ez, w, etc.)
    if opts.enable_slang {
        if let Some(slang) = get_slang_word(&lower) {
            return slang.to_string();
        }
    }

    if opts.enable_french {
        if let Some(french) = get_french_word(&lower) {
            return french.to_string();
        }
    }

    // 2. Check common Vietnamese words (chào, cảm ơn, bạn, etc.)
    if opts.enable_vietnamese {
        if let Some(vietnamese) = get_vietnamese_word(&lower) {
            return vietnamese.to_string();
        }
    }

    // 3. Check common Spanish words (hola, amigo, gracias, etc.)
    if opts.enable_spanish {
        if let Some(spanish) = get_spanish_word(&lower) {
            return spanish.to_string();
        }
    }

    // 4. Check English words dictionary (from CMU dict pre-conversion)
    if opts.enable_english {
        if let Some(english) = get_english_word(&lower) {
            return english.to_string();
        }
    }

    // 5. Fallback: mirrors the TS `resolveWord` priority exactly. When
    // English romanization is enabled, chain Vietnamese then Spanish
    // preprocessing before phonics. When it is disabled, apply *only one*
    // preprocessor (Vietnamese takes priority over Spanish) instead of
    // chaining both, and never touch the word if neither applies -- e.g.
    // an English word like "hello" must stay untouched when
    // `enable_english=false`, not partially rewritten by Spanish digraph
    // preprocessing.
    if opts.enable_english {
        let mut preprocessed = word.to_string();
        if opts.enable_vietnamese {
            preprocessed = vietnamese_preprocess(&preprocessed);
        }
        if opts.enable_spanish {
            preprocessed = spanish_preprocess(&preprocessed);
        }
        phonics_to_katakana(&preprocessed)
    } else if opts.enable_vietnamese {
        vietnamese_preprocess(word)
    } else if opts.enable_spanish {
        spanish_preprocess(word)
    } else {
        word.to_string()
    }
}

/// Main Katakana converter pipeline orchestrator.
#[derive(Debug, Clone, Default)]
pub struct KatakanaConverter {
    default_options: KatakanaOptions,
}

impl KatakanaConverter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(options: KatakanaOptions) -> Self {
        Self {
            default_options: options,
        }
    }

    pub fn convert(&self, text: &str) -> String {
        self.convert_with_options(text, &self.default_options)
    }

    pub fn convert_with_options(&self, text: &str, opts: &KatakanaOptions) -> String {
        // 0. Escape excluded patterns (protect user-defined words, URLs, mentions, etc.)
        let (escaped, token_map) = escape_excluded(text, &opts.exclude);

        // 1. Normalize smart curly apostrophes to standard ASCII apostrophe
        let mut result = escaped.replace(['\u{2018}', '\u{2019}'], "'");

        // 2. Cyrillic (Russian)
        if opts.enable_cyrillic {
            result = convert_cyrillic(&result);
        }

        // 3. Hangul (Korean)
        if opts.enable_korean {
            result = convert_korean(&result);
        }

        // 4. Chinese (Kanji Guard + Taiwan phrases + Mandarin Hanzi)
        if opts.enable_chinese {
            result = convert_chinese(&result);
        }

        // 4b. Thai script (phrase dictionary + conservative syllable decomposition)
        if opts.enable_thai {
            result = convert_thai(&result);
        }

        // 5. French dictionary phrases
        if opts.enable_french {
            result = replace_french_phrases(&result);
        }

        // 6. Spanish multi-word phrases
        if opts.enable_spanish {
            result = replace_spanish_phrases(&result);
        }

        // 5b. Vietnamese multi-word phrases
        if opts.enable_vietnamese {
            result = replace_vietnamese_phrases(&result);
        }

        // 6. Slang multi-word phrases
        if opts.enable_slang {
            result = replace_slang_phrases(&result);
        }

        // 7. Word-level conversion (Slang -> Vietnamese -> Spanish -> English words -> Phonics fallback)
        if opts.enable_english
            || opts.enable_slang
            || opts.enable_spanish
            || opts.enable_french
            || opts.enable_vietnamese
        {
            result = WORD_REGEX
                .replace_all(&result, |caps: &regex::Captures| {
                    resolve_word(&caps[0], opts)
                })
                .into_owned();
        }

        // 8. Prosody normalization
        if opts.normalize_prosody {
            result = normalize_prosody(&result);
        }

        // 9. Restore excluded tokens
        restore_excluded(&result, &token_map)
    }
}

/// Helper function to convert text with default or custom options.
pub fn to_katakana(text: &str, options: Option<&KatakanaOptions>) -> String {
    match options {
        Some(opts) => {
            let converter = KatakanaConverter::new();
            converter.convert_with_options(text, opts)
        }
        None => {
            static DEFAULT_CONVERTER: LazyLock<KatakanaConverter> =
                LazyLock::new(KatakanaConverter::new);
            DEFAULT_CONVERTER.convert(text)
        }
    }
}

/// Load-time sanity check used by the language glue crates (node / python /
/// ffi) to confirm the native library is wired up and produces expected
/// output before it is trusted for real conversions.
pub fn native_self_check() -> bool {
    to_katakana("hello", None) == "ハロー"
}
