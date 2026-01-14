#!/usr/bin/env bash
set -euo pipefail

CARGO_HOME="${CARGO_HOME:-$(pwd)/.cargo}"
export CARGO_HOME
mkdir -p "$CARGO_HOME"

if ! command -v cargo-audit >/dev/null 2>&1; then
  echo "cargo-audit is required (cargo install cargo-audit)" >&2
  exit 1
fi

if ! rustup component list --toolchain nightly --installed | grep -q '^miri'; then
  echo "miri is required (rustup component add miri --toolchain nightly)" >&2
  exit 1
fi

echo "running clippy"
cargo clippy --workspace --all-targets --all-features --exclude netbox-openapi --no-deps -- -D warnings

echo "running audit"
cargo audit

echo "running miri"
PROPTEST_CASES=1 MIRIFLAGS="-Zmiri-strict-provenance -Zmiri-disable-isolation" \
  cargo +nightly miri test -p netbox -- --test-threads=1
