use multilingual_katakana_core::{to_katakana, KatakanaOptionsOverrides};
use pyo3::prelude::*;

/// Convert `text` to Katakana using the native Rust core.
///
/// All flag arguments are optional; omitting one (passing `None` from the
/// Python wrapper) falls back to the same default as `KatakanaOptions::default()`
/// in the Rust core. Resolution is delegated to the core's shared
/// `KatakanaOptionsOverrides::resolve`, which the Node.js native binding
/// (`crates/multilingual-katakana-node/src/lib.rs`) also uses, so default
/// resolution lives in one place (the core), not duplicated per-language.
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
    let opts = KatakanaOptionsOverrides {
        enable_cyrillic,
        enable_korean,
        enable_chinese,
        enable_spanish,
        enable_french,
        enable_vietnamese,
        enable_thai,
        enable_slang,
        enable_english,
        normalize_prosody,
    }
    .resolve();
    to_katakana(text, Some(&opts))
}

/// Lightweight runtime self-check used by the Python package's `__init__.py`
/// to confirm the native extension loaded correctly and produces expected
/// output (mirrors `native_self_check` in the Node.js binding).
#[pyfunction]
fn native_self_check() -> bool {
    multilingual_katakana_core::native_self_check()
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
