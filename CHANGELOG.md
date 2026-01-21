# changelog

this release captures the current state of the project. no prior published state exists for comparison.

## [unreleased]

## [0.2.1] - 2026-01-22

### client
- allow dynamic resource paths via `Resource::dynamic`
- add `Client::resource` helper for ad-hoc endpoints

## [0.2.0] - 2026-01-21

### client
- add read-only graphql helper with query variables
- add scope_type and scope_id to prefix create/update requests

### docs
- split user docs and dev docs, improve docs index entrypoints
- refresh readme text to reflect pre-release stability

### cli
- add output formats (json, yaml, table) with automatic table shaping for paginated results
- add simple --select for dot paths
- add --dry-run for write operations with full request output
- improve error messages with status, path, and request id when present

### tests
- expand smoke coverage for graphql, openapi status, pagination, and cli output modes

## [0.1.6] - 2026-01-20

### client
- add bulk create/update/patch/delete helpers on `Resource` plus `BulkUpdate`/`BulkDelete` wrappers
- remove duplicate root docs and point to crate docs as the source of truth

## [0.1.5] - 2026-01-16

### client
- add field patch request structs for dcim and ipam (custom fields, tags, and device local context) to support typed patch calls

## [0.1.4] - 2026-01-15

- fix a bug in the openapi generation that automatically assumed enums were strings and fixed them for integers

## [0.1.3] - 2026-01-14

### metadata
- fix repository and homepage urls for crates.io metadata

## [0.1.2] - 2026-01-14

### documentation
- add docs.rs link metadata for `netbox`

## [0.1.1] - 2026-01-14

### documentation
- add per-crate readme metadata so crates.io shows docs for each crate
- refresh cli docs to focus on end user usage and examples
- add local docs build guidance and entrypoints

### metadata
- add docs.rs metadata for `netbox-openapi`

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

[unreleased]: https://github.com/cyberwitchery/netbox.rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/cyberwitchery/netbox.rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.6...v0.2.0
[0.1.5]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.2...v0.1.4
[0.1.3]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cyberwitchery/netbox.rs/releases/tag/v0.1.0
