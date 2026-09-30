"""Basic API-shape tests, complementing the spec-conformance suite in
test_spec.py."""

from multilingual_katakana import (
    KatakanaConverter,
    KatakanaOptions,
    _self_check,
    to_katakana,
)


def test_native_self_check() -> None:
    assert _self_check() is True


def test_to_katakana_basic() -> None:
    assert to_katakana("hello") == "ハロー"


def test_options_disable_language_passes_through_unchanged() -> None:
    # Safe Failure: disabling a language must never drop/crash, only skip
    # conversion for that language's text (AGENTS.md section 1.3).
    assert to_katakana("你好", KatakanaOptions(enable_chinese=False)) == "你好"


def test_kanji_guard_not_misread_as_pinyin() -> None:
    assert to_katakana("了解") == "了解"


def test_katakana_converter_reuses_options() -> None:
    converter = KatakanaConverter(KatakanaOptions(enable_korean=False))
    assert converter.convert("안녕") == "안녕"
    assert converter.convert("hello") == "ハロー"


def test_options_are_frozen() -> None:
    options = KatakanaOptions()
    try:
        options.enable_english = False  # type: ignore[misc]
        assert False, "KatakanaOptions must be immutable"
    except AttributeError:
        pass
