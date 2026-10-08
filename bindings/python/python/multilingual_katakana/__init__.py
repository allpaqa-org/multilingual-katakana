"""allpaqa-multilingual-katakana: zero-dependency, ultra-fast multilingual
foreign language to Japanese Katakana phonetic converter.

This is a thin, hand-written Python wrapper around the native Rust core
(``multilingual_katakana._native``, built with PyO3). It exposes the public
API using Python naming conventions (snake_case) while keeping the same
shape as the Node.js/TypeScript binding's ``toKatakana`` / ``KatakanaConverter``
(see crates/multilingual-katakana-python/src/lib.rs and AGENTS.md section 4).
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from . import _native

__all__ = ["KatakanaOptions", "KatakanaConverter", "to_katakana"]

__version__ = "0.5.0"


@dataclass(frozen=True)
class KatakanaOptions:
    """Per-language toggles and prosody normalization, mirroring the Rust
    core's ``KatakanaOptions`` defaults (all languages enabled by default).

    Note: ``exclude`` (user-defined literal/regex protection) from the
    Node.js binding is not yet available here — see the native binding's
    Python follow-up issue.
    """

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


def to_katakana(text: str, options: Optional[KatakanaOptions] = None) -> str:
    """Convert ``text`` (English/Chinese/Korean/Russian/Spanish/French/
    Vietnamese/Thai/streaming slang) to Japanese Katakana for TTS playback.

    Unrecognized languages, emoji, and symbols are always passed through
    unchanged rather than dropped (Safe Failure principle, AGENTS.md
    section 1.3).
    """
    opts = options or KatakanaOptions()
    return _native.to_katakana_native(
        text,
        opts.enable_cyrillic,
        opts.enable_korean,
        opts.enable_chinese,
        opts.enable_spanish,
        opts.enable_french,
        opts.enable_vietnamese,
        opts.enable_thai,
        opts.enable_slang,
        opts.enable_english,
        opts.normalize_prosody,
    )


class KatakanaConverter:
    """Reusable converter bound to a fixed set of options, for callers that
    convert many strings with the same configuration (mirrors the
    Node.js/TypeScript ``KatakanaConverter`` class)."""

    def __init__(self, options: Optional[KatakanaOptions] = None) -> None:
        self._options = options or KatakanaOptions()

    def convert(self, text: str) -> str:
        return to_katakana(text, self._options)


def _self_check() -> bool:
    """Internal: confirms the native extension loaded correctly and
    produces expected output. Used by tests, not part of the public API."""
    return bool(_native.native_self_check())
