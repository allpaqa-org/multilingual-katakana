use regex::Regex;

/// Exclude pattern: either a literal string or a regular expression.
#[derive(Debug, Clone)]
pub enum ExcludePattern {
    Literal(String),
    Regex(Regex),
}

impl From<String> for ExcludePattern {
    fn from(s: String) -> Self {
        ExcludePattern::Literal(s)
    }
}

impl From<&str> for ExcludePattern {
    fn from(s: &str) -> Self {
        ExcludePattern::Literal(s.to_string())
    }
}

impl From<Regex> for ExcludePattern {
    fn from(r: Regex) -> Self {
        ExcludePattern::Regex(r)
    }
}

/// Configuration options for Katakana conversion.
#[derive(Debug, Clone)]
pub struct KatakanaOptions {
    pub enable_cyrillic: bool,
    pub enable_korean: bool,
    pub enable_chinese: bool,
    pub enable_spanish: bool,
    pub enable_french: bool,
    pub enable_vietnamese: bool,
    pub enable_thai: bool,
    pub enable_slang: bool,
    pub enable_english: bool,
    pub normalize_prosody: bool,
    pub exclude: Vec<ExcludePattern>,
}

impl Default for KatakanaOptions {
    fn default() -> Self {
        Self {
            enable_cyrillic: true,
            enable_korean: true,
            enable_chinese: true,
            enable_spanish: true,
            enable_french: true,
            enable_vietnamese: true,
            enable_thai: true,
            enable_slang: true,
            enable_english: true,
            normalize_prosody: true,
            exclude: Vec::new(),
        }
    }
}

impl KatakanaOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_exclude<T: Into<ExcludePattern>>(mut self, pattern: T) -> Self {
        self.exclude.push(pattern.into());
        self
    }
}

/// Per-flag optional overrides for [`KatakanaOptions`] (no `exclude`).
///
/// This is the shared option-resolution used by all language glue crates
/// (node / python / ffi): each glue layer maps its host-language arguments
/// into this struct and calls [`resolve`](Self::resolve), so the default
/// for every flag lives in one place (`KatakanaOptions::default()`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KatakanaOptionsOverrides {
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

impl KatakanaOptionsOverrides {
    /// Fill every `None` with the corresponding `KatakanaOptions::default()`
    /// value. `exclude` is always empty.
    pub fn resolve(&self) -> KatakanaOptions {
        let d = KatakanaOptions::default();
        KatakanaOptions {
            enable_cyrillic: self.enable_cyrillic.unwrap_or(d.enable_cyrillic),
            enable_korean: self.enable_korean.unwrap_or(d.enable_korean),
            enable_chinese: self.enable_chinese.unwrap_or(d.enable_chinese),
            enable_spanish: self.enable_spanish.unwrap_or(d.enable_spanish),
            enable_french: self.enable_french.unwrap_or(d.enable_french),
            enable_vietnamese: self.enable_vietnamese.unwrap_or(d.enable_vietnamese),
            enable_thai: self.enable_thai.unwrap_or(d.enable_thai),
            enable_slang: self.enable_slang.unwrap_or(d.enable_slang),
            enable_english: self.enable_english.unwrap_or(d.enable_english),
            normalize_prosody: self.normalize_prosody.unwrap_or(d.normalize_prosody),
            exclude: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_overrides_resolve_to_default_options() {
        let r = KatakanaOptionsOverrides::default().resolve();
        let d = KatakanaOptions::default();
        assert_eq!(r.enable_cyrillic, d.enable_cyrillic);
        assert_eq!(r.enable_korean, d.enable_korean);
        assert_eq!(r.enable_chinese, d.enable_chinese);
        assert_eq!(r.enable_spanish, d.enable_spanish);
        assert_eq!(r.enable_french, d.enable_french);
        assert_eq!(r.enable_vietnamese, d.enable_vietnamese);
        assert_eq!(r.enable_thai, d.enable_thai);
        assert_eq!(r.enable_slang, d.enable_slang);
        assert_eq!(r.enable_english, d.enable_english);
        assert_eq!(r.normalize_prosody, d.normalize_prosody);
        assert!(r.exclude.is_empty());
    }

    #[test]
    fn single_override_is_respected() {
        let r = KatakanaOptionsOverrides {
            enable_english: Some(false),
            ..Default::default()
        }
        .resolve();
        assert!(!r.enable_english);
        assert!(r.enable_cyrillic);
        assert!(r.enable_korean);
        assert!(r.enable_chinese);
        assert!(r.enable_spanish);
        assert!(r.enable_french);
        assert!(r.enable_vietnamese);
        assert!(r.enable_thai);
        assert!(r.enable_slang);
        assert!(r.normalize_prosody);
    }
}
