# Platform package scaffolding

This directory holds the per-platform npm package templates for the Tier 1 native
Node.js binaries described in `docs/V0.4.0_BINDINGS_SCOPE.md` §5.2.

Only `package.json` scaffolding is committed here. CI copies the freshly built
`.node` binary into the matching `bindings/node/npm/<triple>/` directory just
before `npm pack`, then smoke-tests the tarball (via local `npm install` for
glibc/darwin/win32 targets, or inside an Alpine/musl container for the two
musl targets) before publishing.

These packages are never published directly from a developer machine.
`.github/workflows/build-native-matrix.yml` publishes all 8 platform packages
to npm automatically, but only when triggered by a version tag push (`v*`) —
`workflow_dispatch` runs stay build/pack/smoke-test only, for manual
verification ahead of a release. A `verify-versions` job runs first and fails
the release if the tag doesn't match all 8 platform `package.json` versions
and the 8 `optionalDependencies` pins in `bindings/node/package.json` (see
`docs/RELEASE_PROCESS.md` §3.1).

The **first** publish of each of the 8 packages uses the classic `NPM_TOKEN`
secret, because npm Trusted Publishing (OIDC) cannot be configured for a
package that doesn't exist on npm yet (tracked in issue #15). Once all 8
exist on npm, Trusted Publishing should be configured for each on
npmjs.com, and the `NODE_AUTH_TOKEN` env var can be removed from the publish
step so npm CLI uses OIDC automatically.
