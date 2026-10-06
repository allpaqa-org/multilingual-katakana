# Release Automation & Protocol (Release Process)

This document defines the fully autonomous release automation protocol
(CI/CD, draft releases, AI-driven release-note authoring, and npm
publishing) for `multilingual-katakana`.
AI agents and developers must strictly follow this protocol when
performing a release.

---

## 1. Release Philosophy

1. **Fully zero-dependency**
   - No external runtime dependencies. Pure TypeScript and Pure Rust
     guarantee lightweight, secure, deterministic behavior.
2. **100% compliance with the quality gates**
   - It is an absolute release requirement that, both in CI and locally,
     all unit tests, Biome static analysis, code complexity (CC <= 15),
     spec schema validation, and Rust Clippy/Fmt all pass at 100%.
3. **AI autonomous update protocol (per `AGENTS.md` §5)**
   - After GitHub Actions (`release.yml`) creates a draft release, an
     AI coding agent in the development environment (Antigravity /
     Gemini CLI, etc.) autonomously runs `gh release edit` to format the
     release notes and hands the Releases URL back to the user — a
     self-contained, autonomous release operation.

---

## 2. Release Automation Architecture Overview

```text
[1. Sync version numbers & run local pre-release verification]
  - Update package.json (root), bindings/node/package.json, and
    crates/multilingual-katakana-core/Cargo.toml in lockstep
  - Confirm every quality gate (Bun + Cargo) is fully green
       ↓
[2. Commit, tag, and push]
  - git commit -am "chore(release): vX.Y.Z"
  - git tag vX.Y.Z
  - git push origin main --tags
       ↓
[3. CD: release.yml triggers]
  - GitHub Actions fires on push (tags: ['v*'])
  - After the quality gate (verify-and-pack) passes, it builds dual
    ESM/CJS bundles and runs npm pack
  - A least-privilege job (draft-release) automatically creates a draft
    release (draft: true) with dist/*.tgz attached
       ↓
[4. AI autonomously drafts the release notes]
  - Once GitHub Actions finishes, an AI coding agent in the development
    environment (Antigravity / Gemini CLI, etc.) autonomously runs
    gh release edit
  - It rewrites the body into Markdown summarizing the changes, key new
    features, and the list of distributed assets, then hands the
    Releases URL back to the user
       ↓
[5. One-click publish]
  - The user clicks "Publish release" once on the GitHub Releases page
       ↓
[6. CD: publish.yml triggers & automatic npm publish]
  - Triggered by the release (types: [published]) event
  - @allpaqa/multilingual-katakana is published to the official npm
    registry with Sigstore provenance attached
```

---

## 3. Monorepo Versioning Rule

This repository is a monorepo containing the TypeScript bindings and the
Rust core.
Following Semantic Versioning (SemVer), the version number in the
following **3 locations must always be kept in sync** (e.g. `0.2.0`):

1. **Root `package.json`**: `"version": "X.Y.Z"`
2. **Node.js bindings `bindings/node/package.json`**: `"version": "X.Y.Z"`
3. **Rust core crate `crates/multilingual-katakana-core/Cargo.toml`**:
   `version = "X.Y.Z"`

### 3.1 Native platform packages (v0.4.0+)

Once native distribution ships (see `docs/V0.4.0_BINDINGS_SCOPE.md` §7.4,
"lockstep versioning"), the following **must also match the same
`X.Y.Z`** as the 3 locations above before tagging a release:

4. **Each of the 8 platform packages**:
   `bindings/node/npm/<triple>/package.json` → `"version": "X.Y.Z"`
   (triples: `darwin-arm64`, `darwin-x64`, `linux-x64-gnu`,
   `linux-arm64-gnu`, `linux-x64-musl`, `linux-arm64-musl`,
   `win32-x64-msvc`, `win32-arm64-msvc`)
5. **`bindings/node/package.json`'s `optionalDependencies`**: each of the
   8 `@allpaqa/multilingual-katakana-<triple>` entries must be pinned to
   the exact same `"X.Y.Z"` (no `^`/`~` range — see §3 of the bindings
   scope doc for the zero-dependency rationale).

Items 4 and 5 are **generated, not hand-edited** — they, along with the
platform list itself, come from the single source of truth
`.github/native-platforms.json` via `scripts/generate_native_packages.ts`
(run `bun run generate:native-packages` after bumping the version so the
version bump propagates into all 8 files). See
`bindings/node/npm/README.md` for details.

`.github/workflows/build-native-matrix.yml` enforces this automatically:
its `verify-versions` job fails the release build if the git tag doesn't
match `bindings/node/package.json`'s version, or if the committed platform
package files (item 4/5 above) don't match what
`scripts/generate_native_packages.ts` would generate, before any platform
package is published — because **npm versions are immutable** — a
mismatch published under the wrong version number can never be corrected
after the fact.

### 3.2 Python binding (v0.5.0+)

Once the Python binding ships (see `docs/V0.4.0_BINDINGS_SCOPE.md` §7.3),
add the following to the same lockstep `"X.Y.Z"`:

6. **PyO3 glue crate `crates/multilingual-katakana-python/Cargo.toml`**:
   `version = "X.Y.Z"`
7. **Python package `bindings/python/pyproject.toml`**:
   `version = "X.Y.Z"` (under `[project]`) — also mirrored in
   `bindings/python/python/multilingual_katakana/__init__.py`'s
   `__version__`.

`.github/workflows/build-python-matrix.yml` enforces this automatically:
its `verify-versions` job fails the build if the git tag doesn't match
items 6/7 above (before any wheel is published) — because **PyPI
versions are immutable**, a mismatch published under the wrong version
number can never be corrected after the fact. Unlike the native platform
packages (§3.1), there are no generated per-platform files to keep in
sync here — abi3 wheels are tagged per-platform automatically by
maturin/auditwheel, not by hand-edited JSON.

### 3.3 .NET binding (v0.6.0+)

Once the .NET binding ships (see `docs/V0.4.0_BINDINGS_SCOPE.md` §7.3),
add the following to the same lockstep `"X.Y.Z"`:

8. **C ABI crate `crates/multilingual-katakana-ffi/Cargo.toml`**:
   `version = "X.Y.Z"`
9. **NuGet package `Allpaqa.MultilingualKatakana`**:
   `bindings/dotnet/Directory.Build.props` → `<Version>X.Y.Z</Version>`
   (the single version source for every project under `bindings/dotnet`;
   do not add a `<Version>` to an individual `.csproj`). Prereleases use
   the npm/Cargo spelling (`X.Y.Z-rc.N`).

The 9 RIDs shipped in the package (`win-x64`, `win-x86`, `win-arm64`,
`linux-x64`, `linux-arm64`, `linux-musl-x64`, `linux-musl-arm64`,
`osx-x64`, `osx-arm64`) come from the single source of truth
`.github/dotnet-platforms.json`.

`scripts/check_versions.ts` verifies **every** lockstep location in
§3–§3.3 at once (npm, platform packages and their `optionalDependencies`
pins, all `crates/*/Cargo.toml`, `pyproject.toml` / `__version__` in
PEP 440 spelling, and `Directory.Build.props`). `ci.yml` runs it on pushes
to `main` and `feature/**` and on pull requests to `main`; the
`verify-versions` jobs of the matrix workflows (including
`.github/workflows/build-dotnet-matrix.yml`) run it with `--tag` on tag
pushes and releases, before any nupkg is published — because **NuGet versions
are immutable** (a published version can only be unlisted, never
replaced).

**NuGet publishing prerequisites (one-time setup):**

- nuget.org organization **`allpaqa`** owns the package, with the
  **`Allpaqa.`** ID prefix reserved for it.
- A **Trusted Publishing policy** on nuget.org (owner `allpaqa`):
  repository `allpaqa-org/multilingual-katakana`, workflow file
  `build-dotnet-matrix.yml`, environment `nuget`.
- A **GitHub Environment `nuget`** in this repo (optionally with required
  reviewers), which the `publish` job runs in.
- A repository (or `nuget` environment) secret **`NUGET_USER`**: the
  nuget.org account (profile) name that owns the policy — *not* an API
  key. `NuGet/login` exchanges the job's OIDC token for a short-lived API
  key at publish time, so no long-lived key is stored anywhere.
- A repository **variable** `NUGET_PUBLISH_ENABLED` set to `true`. The
  `publish` job additionally requires it. Set it only once all the items
  above are ready (planned for v0.6.0); until then releases build, pack and
  test the nupkg but never publish to NuGet.

---

## 4. Pre-Release Verification Commands (Quality Gates)

Before creating a release tag, run all of the following verification
commands locally and confirm that **every one is 100% green (0 errors,
0 warnings)**:

```bash
# --- TypeScript bindings & repo-wide checks ---
# 1. All unit tests pass
bun run test

# 2. Biome static analysis / format check (0 errors, 0 warnings)
bun run check:ci

# 3. Cyclomatic complexity audit (CC <= 15)
bun run check:complexity

# 4. Language spec / dictionary JSON schema validation (Spec Schema PASS)
bun run validate:spec

# 5. Dual ESM/CJS package build (verify dist/ output)
bun run build

# 6. Lockstep version check across every manifest (§3–§3.3)
bun run scripts/check_versions.ts

# --- Rust core checks ---
# NOTE: crates/multilingual-katakana-python (PyO3) is a workspace member
# without the `extension-module` feature here, so commands 7-9 link
# against libpython and need a *discoverable* Python 3 with a linkable
# shared library (e.g. python.org installers, Homebrew, or CI's
# actions/setup-python all work; some system/Xcode-stub pythons on macOS
# do not — point PYO3_PYTHON at a working interpreter if you hit a
# "library 'pythonX.Y' not found" linker error).
# 7. All Cargo tests pass (100% PASS, 199 shared spec cases)
cargo test --all-targets

# 8. Clippy static analysis (0 warnings required)
cargo clippy --all-targets -- -D warnings

# 9. Rust code format check (0 diffs required)
cargo fmt --check

# --- Python binding checks (v0.5.0+) ---
# `maturin develop` requires an active virtualenv/conda env (or a `.venv`
# next to pyproject.toml) — plain `pip install maturin` into a system
# Python is not enough.
# 10. Build native extension and run the spec + API test suite
cd bindings/python && maturin develop --release && python -m pytest

# --- .NET binding checks (v0.6.0+; .NET 10 SDK + .NET 8 runtime) ---
# 11. Build the C ABI library and stage it into bindings/dotnet/native/<host rid>/
bun run scripts/stage_dotnet_native.ts

# 12. Spec + API tests (100% PASS) and format check (0 diffs)
cd bindings/dotnet && dotnet test -c Release && dotnet format --verify-no-changes
```

---

## 5. Creating and Pushing the Tag

Once all checks pass, create the release commit, cut a version tag, and
push it to the remote:

```bash
git commit -am "chore(release): vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

---

## 6. Automatic Draft Release Creation & AI Autonomous Update

1. **`.github/workflows/release.yml` runs automatically**:
   - The workflow triggers when a `v*` tag is pushed.
   - The `verify-and-pack` job (read-only permissions) verifies the
     Rust / Bun quality gates (`bun run check:ci`, etc.), then builds
     dual ESM/CJS bundles and runs `npm pack` to produce artifacts.
   - The `draft-release` job (write permissions) automatically creates a
     **draft release (`draft: true`)** with the `dist/*.tgz` asset
     attached.

2. **AI autonomously updates the release notes (`AGENTS.md` §5)**:
   - After GitHub Actions (`release.yml`) creates the draft, an AI coding
     agent in the development environment (Antigravity / Gemini CLI,
     etc.) autonomously runs `gh release edit` to rewrite the release
     notes into rich Markdown covering the latest changes, key new
     features, and asset information, then hands the Releases URL back
     to the user:
   ```bash
   gh release edit vX.Y.Z \
     --title "vX.Y.Z: <Release summary title>" \
     --notes "<Detailed changes, key new features, list of distributed assets>"
   ```

---

## 7. One-Click Publish & Automatic npm Publish

1. **One-click publish**:
   - Once the release notes are updated, the AI hands the GitHub
     Releases URL back to the user.
   - The user reviews the content and clicks **"Publish release"** once
     on the GitHub UI.

2. **Automatic npm publish (`.github/workflows/publish.yml`)**:
   - Once the release is published, `.github/workflows/publish.yml`
     triggers automatically.
   - `@allpaqa/multilingual-katakana` is safely published to the
     official npm registry, with Sigstore provenance attached via an
     OIDC token.

3. **Automatic PyPI publish (`.github/workflows/build-python-matrix.yml`,
   v0.5.0+)**:
   - The same `release: published` event also triggers this workflow's
     `build` job for all 8 Tier 1 platforms, followed by its `publish`
     job.
   - `allpaqa-multilingual-katakana` abi3 wheels are published to PyPI via
     Trusted Publishing (OIDC) — no long-lived API token is stored in
     this repo.
   - The `publish` job runs only when the repository variable
     `PYPI_PUBLISH_ENABLED` is `true`. Set it once the PyPI organization
     `allpaqa`, the Trusted Publisher and the GitHub Environment `pypi`
     are ready (#31); until then releases build and smoke-test wheels but
     never publish them.
   - This runs as an independent job from the npm publish above; if
     either fails, re-run only the failed workflow (both are idempotent —
     already-published versions are skipped, not re-uploaded) and note
     the failure in the release notes per §7.5 of
     `docs/V0.4.0_BINDINGS_SCOPE.md`.

4. **Automatic NuGet publish (`.github/workflows/build-dotnet-matrix.yml`,
   v0.6.0+)**:
   - The same `release: published` event builds the C ABI library for
     all 9 RIDs, packs a single `Allpaqa.MultilingualKatakana` nupkg,
     tests that nupkg on every RID (plus .NET Framework 4.8 x64/x86 and
     Native AOT), and only then runs its `publish` job.
   - The nupkg is pushed to nuget.org via Trusted Publishing (OIDC,
     `NuGet/login`) — no long-lived API key is stored in this repo (setup
     in §3.3). The `publish` job only runs when the repository variable
     `NUGET_PUBLISH_ENABLED` is `true`; until it is set, this workflow
     builds and tests but skips publishing.
   - Like the PyPI publish, this is independent of the npm publish; it is
     idempotent (`--skip-duplicate`), so re-run only the failed workflow
     and note the failure in the release notes.
