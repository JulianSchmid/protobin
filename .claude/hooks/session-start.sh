#!/bin/bash
# Prepares Claude Code on the web sessions so `cargo test`, `cargo fmt` and
# `cargo clippy` work immediately. Runs synchronously at session start.
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

cd "$CLAUDE_PROJECT_DIR"

# Components used by the CI fmt and clippy jobs (no-op when already installed).
rustup component add rustfmt clippy

# Download dev-dependencies (proptest) and compile the crate plus all test
# targets, so the first `cargo test` in a session only has to run them.
cargo fetch
cargo test --no-run
