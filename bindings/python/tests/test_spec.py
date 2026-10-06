"""Spec conformance test: every case in spec/cases/*.json must pass via the
Python binding, mirroring bindings/node/tests/spec.test.ts and
crates/multilingual-katakana-core/tests/spec_test.rs. This is the
language-neutral contract described in AGENTS.md section 2.2.
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from multilingual_katakana import to_katakana

SPEC_CASES_DIR = Path(__file__).resolve().parents[3] / "spec" / "cases"


def _load_cases():
    cases = []
    for path in sorted(SPEC_CASES_DIR.glob("*.json")):
        for tc in json.loads(path.read_text(encoding="utf-8")):
            cases.append((path.name, tc))
    return cases


ALL_CASES = _load_cases()


def _expected(tc: dict) -> tuple[str, list[str]]:
    expected = tc["expected"]
    if isinstance(expected, str):
        return expected, []
    return expected.get("canonical", ""), expected.get("accepted", [])


@pytest.mark.parametrize(
    "file_name,tc",
    ALL_CASES,
    ids=[f"{file_name}::{tc['id']}" for file_name, tc in ALL_CASES],
)
def test_spec_case(file_name: str, tc: dict) -> None:
    actual = to_katakana(tc["input"])
    canonical, accepted = _expected(tc)
    assert actual == canonical or actual in accepted, (
        f"[{tc['id']}] in {file_name} (lang: {tc['language']!r}):\n"
        f"  Input:    {tc['input']!r}\n"
        f"  Expected: {canonical!r} (or accepted: {accepted!r})\n"
        f"  Actual:   {actual!r}\n"
        f"  Desc:     {tc['description']}"
    )


def test_spec_case_count() -> None:
    # Keep this in sync with crates/multilingual-katakana-core/tests/spec_test.rs.
    assert len(ALL_CASES) == 177, "Expected exactly 177 spec test cases"
