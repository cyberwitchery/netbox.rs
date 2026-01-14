#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "cargo-llvm-cov is required (cargo install cargo-llvm-cov)" >&2
  exit 1
fi

echo "building docs"
cargo doc --workspace --all-features --no-deps

echo "running coverage"
cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/netbox-openapi' --fail-under-lines 75

echo "running client smoke tests"
./scripts/run_smoke.sh

echo "running cli smoke tests"
./scripts/run_cli_smoke.sh

echo "running static analysis"
./scripts/run_static.sh
