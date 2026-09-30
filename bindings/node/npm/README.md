# Platform package scaffolding

This directory holds the per-platform npm package templates for the Tier 1 native
Node.js binaries described in `docs/V0.4.0_BINDINGS_SCOPE.md` §5.2.

Only `package.json` scaffolding is committed here. CI copies the freshly built
`.node` binary into the matching `bindings/node/npm/<triple>/` directory just
before `npm pack`, then smoke-tests the tarball via local `npm install`.

These packages are not published directly from a developer machine. The initial
npm reservation / trusted-publishing rollout is tracked in issue #15, and any
future publication should happen from CI after that human-reviewed setup exists.
