# docs index

entrypoints:
- client guide: `docs/client.md`
- cli guide: `docs/cli.md`
- codegen guide: `scripts/README.md`
- local netbox: `docs/local-netbox.md`
- examples: `crates/netbox/examples/README.md`

quick start:

```bash
cargo build
cargo test
```

docs build:

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo doc --workspace --all-features --no-deps
```

open locally:

- `target/doc/netbox/index.html`
- `target/doc/netbox_cli/index.html`
- `target/doc/netbox_openapi/index.html`

for local netbox setup, follow:
https://github.com/netbox-community/netbox-docker
