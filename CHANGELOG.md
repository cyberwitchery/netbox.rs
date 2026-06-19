# changelog

this release captures the current state of the project. no prior published state exists for comparison.

## [unreleased]

## [0.6.0] - 2026-06-19

### fixed
- cap retry backoff at 30 seconds via `RETRY_MAX_BACKOFF` (previously grew unbounded at high attempt counts)

### openapi
- regenerate bindings from NetBox v4.6.2 schema; adds `RenderConfigInputRequest` and `RenderedConfig` models; `user`/`user__n` params on `core_jobs_list` changed from `i32` to `Vec<String>` (now array-valued), with new `user_id`/`user_id__n` integer filters added alongside
- **breaking** `generic_fk` now recognizes the generic relationships NetBox writes as a `<field>_type`/`<field>_id` pair — assigning an IP address to an interface, a prefix's or VLAN group's scope, contact and journal assignments, and more — not just the cable-termination form it knew before. each field now reports how it is encoded (`generic_fk_encoding()`), so these relationships can be created and read back correctly instead of being silently unrecognized. (entries in `GENERIC_FK_FIELDS` gain an encoding element; `is_generic_fk` is unchanged.)

### ci
- bump pinned NetBox container from v4.6.0 to v4.6.2
- refresh `scripts/openapi-schema.json` against v4.6.2

### docs
- update `docs/compat.md` compatibility matrix for v4.6.2

## [0.5.4] - 2026-06-01

### openapi
- generate a `generic_fk` module listing the `(app_label.model, field)` pairs whose write payload is a `GenericObjectRequest` (netbox's polymorphic generic foreign keys), derived from the openapi schema. against the v4.6.0 schema this is `dcim.cable.a_terminations` and `dcim.cable.b_terminations`. lets consumers (e.g. the alembic netbox adapter) detect which ref/list_ref fields must be encoded as `{ "object_type": ..., "object_id": ... }` instead of a bare id, without hardcoding a per-type allowlist

### client
- re-export `is_generic_fk` and `GENERIC_FK_FIELDS` at the `netbox` crate root (and under `openapi::generic_fk`)

### build
- `scripts/generate.sh`: emit `crates/netbox-openapi/src/generic_fk.rs` from the normalized schema on every regeneration. the table is sorted, so the `regen.sh` idempotency check still passes

## [0.5.3] - 2026-05-31

### build
- gate per-tag API modules (`dcim_api`, `ipam_api`, `vpn_api`, etc.) behind `#[cfg(not(docsrs))]` so docs.rs skips compiling them — they account for ~78% of `netbox-openapi`'s ~420k lines, and rustc OOMs on them at the docs.rs container limit (peak 6.58 GiB needed, 6 GiB available). the high-level `netbox` client never calls these per-tag functions (only re-exports them), so users on crates.io see no behaviour change; only `docs.rs/netbox-openapi/` loses the per-tag pages.
- the previous `[profile.dev.package.netbox-openapi] debug = 0` override (added in 0.5.1, propagated correctly in 0.5.2) was insufficient on its own — debuginfo wasn't the dominant cost; type-checking the generated derives was.
- post-processing in `scripts/generate.sh` injects the gates automatically on subsequent regenerations.
- added a `build.rs` to `netbox-openapi` that translates `DOCS_RS=1` (set by docs.rs) into `cfg(docsrs)` (the standard idiom for crate-local docs.rs cfg).

## [0.5.2] - 2026-05-31

### build
- duplicate the `[profile.dev.package.netbox-openapi] debug = 0` override into `crates/netbox/Cargo.toml` so it survives `cargo publish` and reaches docs.rs; the workspace-root setting added in 0.5.1 was stripped when packaging the per-crate manifest, leaving docs.rs unfixed

## [0.5.1] - 2026-05-23

### build
- override `[profile.dev.package.netbox-openapi] debug = 0` so docs.rs no longer OOMs compiling the generated bindings with `-C debuginfo=2`; affects only dev-profile builds of `netbox-openapi` as a dependency

## [0.5.0] - 2026-05-23

### openapi
- regenerate bindings from NetBox v4.6.0 schema; adds `CableBundle`, `RackGroup`, `VirtualMachineType`, and `JobNotifications` models; removes `CreateAvailableVlanRequestRole`

### client
- expose `cable_bundles()` and `rack_groups()` on `DcimApi` for new v4.6.0 DCIM endpoints
- expose `virtual_machine_types()` on `VirtualizationApi` for new v4.6.0 endpoint

### cli
- add `cable-bundles`, `rack-groups` (dcim) and `virtual-machine-types` (virtualization) to resource tables

### ci
- bump pinned NetBox container from v4.5.9 to v4.6.0
- refresh `scripts/openapi-schema.json` against v4.6.0

### docs
- update `docs/compat.md` compatibility matrix for v4.6.0

## [0.4.0] - 2026-05-06

### workspace
- bump MSRV from 1.85 to 1.91 (`floor_char_boundary` requires ≥1.82)

### cli
- fix `compact_json` panic on multi-byte UTF-8: use `floor_char_boundary` instead of raw byte offset for truncation

### openapi
- regenerate bindings from NetBox v4.5.9 schema; `TokenProvisionRequest` now exposes optional `version`, `enabled`, and `token` fields
- `scripts/generate.sh`: re-include `#![allow(clippy::all)]` in the generated `lib.rs` header so regeneration doesn't break workspace clippy

### ci
- bump pinned NetBox container from v4.5.8 to v4.5.9
- refresh `scripts/openapi-schema.json` against v4.5.9 (a new required `prefix_length` on `POST /api/ipam/prefixes/{id}/available-prefixes/` would otherwise fail the oasdiff breaking check)

## [0.3.3] - 2026-02-21

### cli
- fix table output: render explicit `--columns` headers even when the result set is empty

### openapi
- regenerate bindings from NetBox 4.4.2; adds `ObjectChangeAction` model
- fix `plugins/mod.rs`: define branch-related types locally since netbox-branching plugin models are no longer in the NetBox core schema

### ci
- add OpenAPI schema breaking-change detection via oasdiff in integration workflow
- add weekly upstream drift detection (`upstream-check.yml`) with automatic issue creation on new upstream releases
- add `docs/compat.md` compatibility matrix mapping client releases to tested NetBox versions

## [0.3.2] - 2026-02-08

### release
- add release SBOM generation and upload (CycloneDX)

### scripts
- add `regen.sh` combining schema fetch + generation with idempotency check
- document environment variables for generation scripts

### client
- add optional `tracing` feature for request lifecycle instrumentation (URL build, send/response timing, retries, and error classification)
- add configurable HTTP extension points: injected prebuilt reqwest client, client-builder callback, and request/response hooks (`HttpHooks`)

### tests
- add golden output test harness for CLI (`cargo test -p netbox-cli --test golden`)
- add GitHub Actions integration workflow with pinned NetBox service container running smoke + golden tests

## [0.3.1] - 2026-01-24

### cli
- add config file support at `~/.config/netbox-cli/config.toml` with named profiles
- add `--profile` flag to select a config profile (default: "default")
- add `config` subcommand with `path`, `list`, `show`, and `validate` actions
- support `token_env` and `token_command` for secure token retrieval
- add `--columns` flag for explicit table column selection
- add `--max-columns` flag to control auto-selected column count (default: 6)

### docs
- add config profiles documentation with examples
- expand `--select` documentation with examples for nested arrays and objects

## [0.3.0] - 2026-01-24

### client
- add IPAM availability endpoints: `available_ips_in_prefix`, `create_available_ips_in_prefix`, `available_prefixes_in_prefix`, `create_available_prefixes_in_prefix`, `available_ips_in_range`, `create_available_ips_in_range`, `available_vlans_in_group`, `create_available_vlans_in_group`, `available_asns_in_range`, `create_available_asns_in_range`
- add core task management: `enqueue_task`, `stop_task`, `requeue_task`, `delete_task`, `sync_data_source`
- add extras sync/render operations: `sync_config_context`, `sync_config_context_profile`, `sync_config_template`, `render_config_template`, `sync_export_template`, `custom_field_choices`
- add circuits path endpoints: `circuit_termination_paths`, `virtual_circuit_termination_paths`
- add dcim trace endpoints: `trace_interface`, `trace_console_port`, `trace_console_server_port`, `trace_power_port`, `trace_power_outlet`, `trace_power_feed`
- add virtualization render config: `render_vm_config`

### cli
- add IPAM availability commands: `ipam-prefix-available-ips`, `ipam-prefix-available-prefixes`, `ipam-range-available-ips`, `ipam-vlan-group-available-vlans`, `ipam-asn-range-available-asns`
- add core task management commands: `core-task-action` (enqueue/stop/requeue/delete), `core-data-source-sync`
- add extras sync/render commands: `extras-config-context-sync`, `extras-config-context-profile-sync`, `extras-config-template-sync`, `extras-config-template-render`, `extras-export-template-sync`, `extras-custom-field-choices`
- add circuits path commands: `circuits-termination-paths`, `circuits-virtual-termination-paths`
- add dcim trace command: `dcim-trace` (interface/console-port/console-server-port/power-port/power-outlet/power-feed)
- add virtualization render command: `virtualization-render-config`

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

[unreleased]: https://github.com/cyberwitchery/netbox.rs/compare/v0.3.3...HEAD
[0.3.3]: https://github.com/cyberwitchery/netbox.rs/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/cyberwitchery/netbox.rs/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/cyberwitchery/netbox.rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/cyberwitchery/netbox.rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/cyberwitchery/netbox.rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.6...v0.2.0
[0.1.5]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.2...v0.1.4
[0.1.3]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/cyberwitchery/netbox.rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cyberwitchery/netbox.rs/releases/tag/v0.1.0
