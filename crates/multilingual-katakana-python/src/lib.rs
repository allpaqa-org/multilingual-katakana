use multilingual_katakana_core::{to_katakana, KatakanaOptions};
use pyo3::prelude::*;

/// Convert `text` to Katakana using the native Rust core.
///
/// All flag arguments are optional; omitting one (passing `None` from the
/// Python wrapper) falls back to the same default as `KatakanaOptions::default()`
/// in the Rust core. This mirrors the shape of the Node.js native binding's
/// `JsKatakanaOptions` (see `crates/multilingual-katakana-node/src/lib.rs`),
/// keeping default resolution logic in one place (Rust), not duplicated
/// per-language.
///
/// `exclude` (user-defined literal/regex protection) is intentionally NOT
/// exposed yet — it is a pure-Python-side feature in the Node/TS binding
/// (PUA placeholder escape/restore in `bindings/node/src/converter.ts`) with
/// no native equivalent here yet. Tracked as follow-up work, not required by
/// the `spec/cases/*.json` contract.
#[pyfunction]
#[pyo3(signature = (
    text,
    enable_cyrillic=None,
    enable_korean=None,
    enable_chinese=None,
    enable_spanish=None,
    enable_french=None,
    enable_vietnamese=None,
    enable_thai=None,
    enable_slang=None,
    enable_english=None,
    normalize_prosody=None,
))]
#[allow(clippy::too_many_arguments)]
fn to_katakana_native(
    text: &str,
    enable_cyrillic: Option<bool>,
    enable_korean: Option<bool>,
    enable_chinese: Option<bool>,
    enable_spanish: Option<bool>,
    enable_french: Option<bool>,
    enable_vietnamese: Option<bool>,
    enable_thai: Option<bool>,
    enable_slang: Option<bool>,
    enable_english: Option<bool>,
    normalize_prosody: Option<bool>,
) -> String {
    let defaults = KatakanaOptions::default();
    let opts = KatakanaOptions {
        enable_cyrillic: enable_cyrillic.unwrap_or(defaults.enable_cyrillic),
        enable_korean: enable_korean.unwrap_or(defaults.enable_korean),
        enable_chinese: enable_chinese.unwrap_or(defaults.enable_chinese),
        enable_spanish: enable_spanish.unwrap_or(defaults.enable_spanish),
        enable_french: enable_french.unwrap_or(defaults.enable_french),
        enable_vietnamese: enable_vietnamese.unwrap_or(defaults.enable_vietnamese),
        enable_thai: enable_thai.unwrap_or(defaults.enable_thai),
        enable_slang: enable_slang.unwrap_or(defaults.enable_slang),
        enable_english: enable_english.unwrap_or(defaults.enable_english),
        normalize_prosody: normalize_prosody.unwrap_or(defaults.normalize_prosody),
        exclude: Vec::new(),
    };
    to_katakana(text, Some(&opts))
}

/// Lightweight runtime self-check used by the Python package's `__init__.py`
/// to confirm the native extension loaded correctly and produces expected
/// output (mirrors `native_self_check` in the Node.js binding).
#[pyfunction]
fn native_self_check() -> bool {
    to_katakana_native(
        "hello", None, None, None, None, None, None, None, None, None, None,
    ) == "ハロー"
}

/// PyO3 module registered as `multilingual_katakana._native` (see
/// `bindings/python/pyproject.toml`'s `[tool.maturin] module-name`). The
/// pure-Python `__init__.py` wraps these into the public `to_katakana` /
/// `KatakanaConverter` / `KatakanaOptions` API.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_katakana_native, m)?)?;
    m.add_function(wrap_pyfunction!(native_self_check, m)?)?;
    Ok(())
}
