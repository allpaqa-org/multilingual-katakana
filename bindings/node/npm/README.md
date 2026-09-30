# Platform package scaffolding

This directory holds the per-platform npm package templates for the Tier 1 native
Node.js binaries described in `docs/V0.4.0_BINDINGS_SCOPE.md` §5.2.

**Do not hand-edit these `package.json` files.** They are generated from the
single source of truth `.github/native-platforms.json` by
`scripts/generate_native_packages.ts`, which also regenerates the
`optionalDependencies` block in `bindings/node/package.json`. To add/change a
platform, edit `.github/native-platforms.json` and run:

```bash
bun run generate:native-packages
```

CI (`build-native-matrix.yml`'s `verify-versions` job) runs the same script
with `--check` and fails the build if any committed file doesn't match what's
generated, so drift between the platform list, the npm packaging metadata,
and the workflow build matrix (also loaded from the same JSON file) is
caught automatically.

Only `package.json` scaffolding is committed here. CI copies the freshly built
`.node` binary into the matching `bindings/node/npm/<triple>/` directory just
before `npm pack`, then smoke-tests the tarball (via local `npm install` for
glibc/darwin/win32 targets, or inside an Alpine/musl container for the two
musl targets) before publishing.

These packages are never published directly from a developer machine.
`.github/workflows/build-native-matrix.yml` publishes all 8 platform packages
to npm automatically, but only when a GitHub Release is actually published
(`release: published`) — the same trigger/gate `publish.yml` uses for the
main package, so platform packages are never published before release.yml's
quality gates and the human "Publish release" click have happened. A version
tag push (`v*`) and `workflow_dispatch` both still run build/pack/smoke-test
for early verification, but never publish. A `verify-versions` job runs
first (on tag push and on release) and fails the run if the tag doesn't
match `bindings/node/package.json`'s version, or if any generated platform
package file is out of date relative to `.github/native-platforms.json` (see
above, and `docs/RELEASE_PROCESS.md` §3.1). The publish step also skips
cleanly (instead of failing) if a version was
already published by an earlier attempt, so re-running the workflow after a
partial failure is safe.

`.github/workflows/publish.yml` (the main package) in turn verifies that all
8 platform packages already exist on npm at the pinned version before it
publishes the main package — since both workflows trigger on the same
`release: published` event with no guaranteed ordering, this stops users
from silently getting the pure-TypeScript fallback if the platform packages
haven't finished publishing yet.

The **first** publish of each of the 8 packages used the classic `NPM_TOKEN`
secret, because npm Trusted Publishing (OIDC) cannot be configured for a
package that doesn't exist on npm yet. Since all 8 (plus the main package)
now exist on npm, every one has Trusted Publishing configured on npmjs.com,
and both `publish.yml` and `build-native-matrix.yml` publish via OIDC —
no long-lived token is used or stored for publishing anymore.
