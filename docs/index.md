# docs index

entrypoints:
- client guide: `crates/netbox/docs/client.md`
- http customization and hooks: `crates/netbox/docs/client.md#http-customization-hooks`
- cli guide: `crates/netbox-cli/docs/cli.md`
- examples: `crates/netbox/examples/README.md`
- local netbox: `docs/local-netbox.md`

developer docs:
- dev guide: `docs/dev.md`
- codegen: `scripts/README.md`
- contributing: `CONTRIBUTING.md`

quick start:

```bash
NETBOX_TOKEN=... cargo run -p netbox --example status
NETBOX_TOKEN=... cargo run -p netbox --example graphql_query
```

for local netbox setup, follow:
https://github.com/netbox-community/netbox-docker
