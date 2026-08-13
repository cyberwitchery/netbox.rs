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

## cutting a release

publishing runs from CI on a `v*` tag. `./scripts/release_local.sh` is the
pre-flight check you run before tagging.

1. rotate `docs/compat.md`: rename the `main` row to the version being
   released, then start a fresh `main` row carrying the same pin.
   `release_local.sh` refuses to continue if the released version has no row.
2. move the `[unreleased]` entries in `CHANGELOG.md` under a heading for the
   new version.
3. bump the workspace version in `Cargo.toml`.
4. run `./scripts/release_local.sh`.
5. push a `v<version>` tag. the release workflow publishes `netbox-openapi`,
   then `netbox`, then `netbox-cli`.
