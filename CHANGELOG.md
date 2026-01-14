# changelog

this release captures the current state of the project. no prior published state exists for comparison.

## [0.1.0] - 2026-01-14

### crates
- `netbox-openapi`: generated bindings for all netbox endpoints and models
- `netbox`: high-level client with typed resources, pagination, and query builder
- `netbox-cli`: fully featured cli for listing, reading, and mutating resources

### client features
- token-based auth, configurable timeouts, retries, and ssl verification
- raw request support, openapi config access, and direct http client access
- error handling with structured api errors and helpers

### coverage
- full module coverage across dcim, ipam, circuits, tenancy, extras, core, users, virtualization, vpn, wireless, plugins
- status and schema endpoints

### tooling
- reproducible openapi generation and schema fetch scripts
- local smoke tests and assurance scripts
- ci workflows for docs, tests, coverage, and static analysis

### docs
- readme and rustdoc coverage for client and cli
- contributing guide and script documentation

[0.1.0]: https://github.com/network-auto/netbox.rs/releases/tag/v0.1.0
