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
