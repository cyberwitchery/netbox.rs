# contributing

short, practical notes for working on this repo.

## local dev commands

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features
cargo fmt --all
```

first builds on a new machine or fresh checkout are slow because the dependency cache is cold.

## local smoke tests

manual smoke tests live under `crates/netbox/tests/smoke_local.rs` and hit a live netbox.

they are ignored by default. run with:

```bash
NETBOX_TOKEN=... NETBOX_URL=http://localhost:8000 cargo test -p netbox --test smoke_local -- --ignored
```

## coverage

we use `cargo llvm-cov`.

install:

```bash
cargo install cargo-llvm-cov
```

run:

```bash
cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/netbox-openapi'
```

generate lcov (for ci or tooling):

```bash
cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/netbox-openapi' --lcov --output-path lcov.info
```

ci enforces a minimum line coverage of 70% while excluding generated code.

## documentation

- add rustdoc for public apis
- include examples for new features
- update `README.md` and `CHANGELOG.md` for user-visible changes

## api design principles

1. ergonomics
2. type safety
3. consistency
4. stability

## release checklist (maintainers)

1. update `CHANGELOG.md`
2. bump versions in `Cargo.toml`
3. regenerate openapi bindings
4. run tests and coverage
5. publish crates

## issue reporting

use https://github.com/network-auto/netbox.rs/issues

## local release

run:

```bash
./scripts/release_local.sh
```

set `SKIP_COVERAGE=1` to skip coverage.

notes:
- publish `netbox-openapi` before `netbox`
- the script uses `cargo package -p netbox --no-verify` for local packaging

## release automation

we publish from tags matching `v*` (for example, `v0.0.1`).

the release workflow expects a repository secret named `CARGO_REGISTRY_TOKEN`.
