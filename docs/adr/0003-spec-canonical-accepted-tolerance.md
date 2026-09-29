# ADR-0003: Allow `canonical`/`accepted` tolerance in `spec/cases/*.json`

## Context

`spec/cases/*.json` is the language-neutral, public contract that every
binding (TypeScript today, Rust core, future bindings) must pass 100% of
the time. For some inputs — particularly Cyrillic transliteration, where
multiple katakana renderings of a Russian word are equally natural to a
native Japanese listener (e.g. "досвидания" → both "ダスヴィダーニャ" and
"ドオスヴイダンイヤ" are acceptable TTS pronunciations) — requiring a single
exact output string would force the implementation to pick one arbitrary
"correct" answer and fail otherwise-reasonable conversions, or would force
test cases to be dropped/weakened to avoid false failures.

## Decision

Each test case's `expected` object may declare one primary `canonical`
string plus an optional `accepted` array of alternative strings that are
also considered correct:

```json
"expected": {
  "canonical": "ダスヴィダーニャ",
  "accepted": ["ドオスヴイダンイヤ"]
}
```

`scripts/validate_spec.ts` enforces that `canonical` is always present and
that `accepted`, if present, is an array. Test runners (e.g.
`bindings/node/tests/spec.test.ts`) assert `actual === canonical ||
accepted.includes(actual)`, and report the mismatch against `canonical` if
neither matches, so failure messages stay legible.

## Consequences

- `canonical` remains the single source of truth for the "primary"
  expected output (e.g. what appears in documentation or is used for
  performance/diffing purposes), while `accepted` documents deliberately
  tolerated alternative pronunciations rather than silently loosening
  assertions.
- Every binding (TS, Rust, future ones) must implement this same
  `canonical` OR `accepted` matching logic in its own test runner —
  it is part of the spec contract, not an implementation detail of one
  binding.
- This tolerance must be used sparingly and only for genuine "both are
  natural" pronunciation ambiguity. It must not be used to mask arbitrary
  bugs or regressions; any new `accepted` entry should be reviewed for
  whether it reflects real linguistic ambiguity.
