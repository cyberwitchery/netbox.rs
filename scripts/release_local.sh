#!/usr/bin/env bash
set -euo pipefail

root_dir=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$root_dir"

version=$(rg -m 1 '^version = ' Cargo.toml | sed -E 's/version = "([^"]+)"/\1/')

echo "release version: ${version}"

if git status --porcelain | rg . >/dev/null 2>&1; then
  echo "warning: git working tree is dirty"
fi

echo "running fmt"
cargo fmt --all

echo "running clippy"
cargo clippy --all-targets --all-features

echo "running tests"
cargo test

if [[ "${SKIP_COVERAGE:-}" != "1" ]]; then
  echo "running coverage"
  cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/netbox-openapi|smoke_local.rs'
else
  echo "skipping coverage (SKIP_COVERAGE=1)"
fi

echo "building release"
cargo build --release

echo "packaging netbox"
cargo package -p netbox

echo "packaging netbox-openapi"
cargo package -p netbox-openapi

echo "done"
echo "next: review package contents and publish with cargo publish -p netbox"
