# DDGC CI Image

This image provides the shared toolchain used by DDGC CI jobs:

- Node.js and npm from the official Playwright image
- Rust stable through rustup
- Playwright 1.59.1 Chromium and Linux browser dependencies
- Common native build tools for Rust crates

It intentionally does not copy repository source code into the image. CI jobs
should checkout the workspace and run commands against mounted source.

## Image

The workflow publishes to:

```text
ghcr.io/iamshenkui/ddgc_newarch/ci:latest
```

Branch and SHA tags are also emitted by `docker/metadata-action`.

## Local Build

```bash
docker build -f .github/ci/Dockerfile -t ddgc-newarch-ci .
```

## Example Usage

From the orchestration workspace root, where sibling repositories are present:

```bash
docker run --rm -it \
  -v "$PWD:/workspace" \
  -w /workspace/repositories/DDGC_newArch \
  ddgc-newarch-ci \
  bash
```

Then run the usual checks:

```bash
cargo test
cd frontend
npm ci
npm run smoke-build
```
