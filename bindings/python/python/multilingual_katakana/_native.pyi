"""Type stubs for the PyO3 native extension (multilingual_katakana._native).

Implemented in Rust: crates/multilingual-katakana-python/src/lib.rs.
"""

from typing import Optional

def to_katakana_native(
    text: str,
    enable_cyrillic: Optional[bool] = None,
    enable_korean: Optional[bool] = None,
    enable_chinese: Optional[bool] = None,
    enable_spanish: Optional[bool] = None,
    enable_french: Optional[bool] = None,
    enable_vietnamese: Optional[bool] = None,
    enable_thai: Optional[bool] = None,
    enable_slang: Optional[bool] = None,
    enable_english: Optional[bool] = None,
    normalize_prosody: Optional[bool] = None,
) -> str: ...
def native_self_check() -> bool: ...
