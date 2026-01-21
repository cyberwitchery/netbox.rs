# developer docs

this section covers contributor workflows and local validation steps.

## build and test

```bash
cargo build
cargo test
```

## lint and format

```bash
cargo clippy --all-targets --all-features
cargo fmt --all
```

## docs build

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo doc --workspace --all-features --no-deps
```

open locally:

- `target/doc/netbox/index.html`
- `target/doc/netbox_cli/index.html`
- `target/doc/netbox_openapi/index.html`

## scripts

- assurance: `./scripts/run_assurance.sh`
- static analysis: `./scripts/run_static.sh`

## local netbox

see `docs/local-netbox.md`.
