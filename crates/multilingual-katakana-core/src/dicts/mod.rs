include!(concat!(env!("OUT_DIR"), "/generated_dicts.rs"));

/// Binary search on sorted key-value static slice
#[inline]
pub fn lookup_sorted(
    table: &'static [(&'static str, &'static str)],
    key: &str,
) -> Option<&'static str> {
    table
        .binary_search_by_key(&key, |&(k, _)| k)
        .ok()
        .map(|idx| table[idx].1)
}

/// Binary search on sorted char-value static slice
#[inline]
pub fn lookup_char_sorted(
    table: &'static [(char, &'static str)],
    key: char,
) -> Option<&'static str> {
    table
        .binary_search_by_key(&key, |&(k, _)| k)
        .ok()
        .map(|idx| table[idx].1)
}
