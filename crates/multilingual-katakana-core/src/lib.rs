pub mod converter;
pub mod dicts;
pub mod languages;
pub mod normalizers;
pub mod types;

pub use converter::{
    escape_excluded, native_self_check, resolve_word, restore_excluded, to_katakana,
    KatakanaConverter,
};
pub use languages::french::{get_french_word, replace_french_phrases};
pub use languages::thai::{convert_thai, convert_thai_syllables, is_thai, replace_thai_phrases};
pub use languages::vietnamese::{
    get_vietnamese_word, replace_vietnamese_phrases, vietnamese_preprocess,
};
pub use normalizers::normalize_prosody;
pub use types::{ExcludePattern, KatakanaOptions, KatakanaOptionsOverrides};
