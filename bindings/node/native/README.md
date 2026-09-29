# Native addon build output

This directory holds the locally-built NAPI-RS native addon used by the
optional Node.js native backend (see `docs/V0.4.0_BINDINGS_SCOPE.md`).

It is empty in git; the `.node` artifact is a build output, not source, and
is never committed. If this directory is empty or missing entirely, the
package gracefully falls back to the pure-TypeScript pipeline.

To build the native addon for local development:

```bash
cargo build --release -p multilingual-katakana-node
# macOS (Apple Silicon example):
cp target/release/libmultilingual_katakana_node.dylib \
  bindings/node/native/multilingual-katakana.darwin-arm64.node
# Linux (glibc, x64):
cp target/release/libmultilingual_katakana_node.so \
  bindings/node/native/multilingual-katakana.linux-x64-gnu.node
```

Then run tests with the native backend forced to confirm it loads:

```bash
cd bindings/node && MULTILINGUAL_KATAKANA_BACKEND=native bun test
```
