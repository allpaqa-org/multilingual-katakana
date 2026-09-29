use multilingual_katakana_core::{to_katakana, KatakanaOptions};
use napi_derive::napi;

/// JS-facing options mirroring `KatakanaOptions`, minus `exclude`.
///
/// `exclude` intentionally is NOT exposed here: JavaScript `RegExp` semantics
/// differ from Rust `regex` semantics, so the JS side (`escapeExcluded` /
/// `restoreExcluded`) always protects excluded spans with PUA placeholders
/// *before* calling into native code and restores them *after*. The native
/// backend only ever sees already-escaped text.
#[napi(object)]
#[derive(Default)]
pub struct JsKatakanaOptions {
    pub enable_cyrillic: Option<bool>,
    pub enable_korean: Option<bool>,
    pub enable_chinese: Option<bool>,
    pub enable_spanish: Option<bool>,
    pub enable_french: Option<bool>,
    pub enable_vietnamese: Option<bool>,
    pub enable_thai: Option<bool>,
    pub enable_slang: Option<bool>,
    pub enable_english: Option<bool>,
    pub normalize_prosody: Option<bool>,
}

impl From<JsKatakanaOptions> for KatakanaOptions {
    fn from(opts: JsKatakanaOptions) -> Self {
        let defaults = KatakanaOptions::default();
        KatakanaOptions {
            enable_cyrillic: opts.enable_cyrillic.unwrap_or(defaults.enable_cyrillic),
            enable_korean: opts.enable_korean.unwrap_or(defaults.enable_korean),
            enable_chinese: opts.enable_chinese.unwrap_or(defaults.enable_chinese),
            enable_spanish: opts.enable_spanish.unwrap_or(defaults.enable_spanish),
            enable_french: opts.enable_french.unwrap_or(defaults.enable_french),
            enable_vietnamese: opts.enable_vietnamese.unwrap_or(defaults.enable_vietnamese),
            enable_thai: opts.enable_thai.unwrap_or(defaults.enable_thai),
            enable_slang: opts.enable_slang.unwrap_or(defaults.enable_slang),
            enable_english: opts.enable_english.unwrap_or(defaults.enable_english),
            normalize_prosody: opts.normalize_prosody.unwrap_or(defaults.normalize_prosody),
            exclude: Vec::new(),
        }
    }
}

/// Convert `text` to Katakana using the native Rust core.
///
/// `text` must already have excluded spans escaped to PUA placeholders by
/// the JS caller (see `escapeExcluded` in `bindings/node/src/converter.ts`).
#[napi]
pub fn to_katakana_native(text: String, options: Option<JsKatakanaOptions>) -> String {
    let opts: KatakanaOptions = options.unwrap_or_default().into();
    to_katakana(&text, Some(&opts))
}

/// Lightweight runtime self-check used by the JS loader to confirm the
/// native addon loaded correctly and produces expected output before
/// trusting it for real conversions. Returns `true` on success.
#[napi]
pub fn native_self_check() -> bool {
    to_katakana_native("hello".to_string(), None) == "ハロー"
}
