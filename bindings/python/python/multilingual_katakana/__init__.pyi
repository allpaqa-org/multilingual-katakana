from dataclasses import dataclass
from typing import Optional

__version__: str

@dataclass(frozen=True)
class KatakanaOptions:
    enable_cyrillic: bool = True
    enable_korean: bool = True
    enable_chinese: bool = True
    enable_spanish: bool = True
    enable_french: bool = True
    enable_vietnamese: bool = True
    enable_thai: bool = True
    enable_slang: bool = True
    enable_english: bool = True
    normalize_prosody: bool = True

def to_katakana(text: str, options: Optional[KatakanaOptions] = None) -> str: ...

class KatakanaConverter:
    def __init__(self, options: Optional[KatakanaOptions] = None) -> None: ...
    def convert(self, text: str) -> str: ...
