# changelog

## [unreleased]

## [0.5.4] - 2026-06-01

- expose which fields use NetBox's polymorphic generic foreign keys via the new `generic_fk` module (`is_generic_fk`, `GENERIC_FK_FIELDS`, re-exported at the crate root). Derived from the OpenAPI schema (for v4.6.0: `dcim.cable.a_terminations` and `dcim.cable.b_terminations`), this lets consumers such as the alembic NetBox adapter encode those fields as `{ "object_type": ..., "object_id": ... }` instead of a bare id, without hardcoding an allowlist.

## [0.5.3] - 2026-05-31

- fix: `docs.rs` builds no longer run out of memory. The per-tag OpenAPI API modules (`dcim_api`, `ipam_api`, `vpn_api`, etc.) account for ~78% of `netbox-openapi`'s ~420k lines and exceeded the docs.rs memory limit; they are now gated out of docs builds. The high-level `netbox` client doesn't call them (only re-exports them), so crates.io users see no behavior change; only the per-tag pages on `docs.rs/netbox-openapi/` are dropped.

## [0.5.2] - 2026-05-31

- another attempt at fixing docs.rs out-of-memory builds: the `netbox-openapi` debug-info override is now also set in the per-crate manifest so it survives `cargo publish` (the workspace-root setting from 0.5.1 was stripped during packaging). Fully resolved in 0.5.3.

## [0.5.1] - 2026-05-23

- attempt to fix docs.rs out-of-memory builds by disabling debug info for `netbox-openapi` (`[profile.dev.package.netbox-openapi] debug = 0`). Affects only dev-profile builds; fully resolved in 0.5.3.

## [0.5.0] - 2026-05-23

### client
- expose `DcimApi::cable_bundles()`, `DcimApi::rack_groups()`, and `VirtualizationApi::virtual_machine_types()` for the new v4.6.0 endpoints.

### cli
- add `cable-bundles`, `rack-groups`, and `virtual-machine-types` to the resource tables.

### openapi
- regenerated bindings from NetBox v4.6.0: adds `CableBundle`, `RackGroup`, `VirtualMachineType`, and `JobNotifications`; removes `CreateAvailableVlanRequestRole`.

### compatibility
- tested against NetBox v4.6.0.

## [0.4.0] - 2026-05-06

### changed
- MSRV raised from 1.85 to 1.91.

### cli
- fix a panic in `compact_json` on multi-byte UTF-8 (truncation now respects char boundaries).

### openapi
- regenerated bindings from NetBox v4.5.9: `TokenProvisionRequest` gains optional `version`, `enabled`, and `token` fields.

### compatibility
- tested against NetBox v4.5.9.

## [0.3.3] - 2026-02-21

### cli
- fix table output: render explicit `--columns` headers even when the result set is empty.

### openapi
- regenerated bindings from NetBox 4.4.2: adds the `ObjectChangeAction` model.
- define the netbox-branching plugin types locally, since they are no longer part of the NetBox core schema.

### docs
- add a compatibility matrix (`docs/compat.md`) mapping client releases to tested NetBox versions.

### compatibility
- tested against NetBox 4.4.2.

## [0.3.2] - 2026-02-08

### client
- add configurable HTTP extension points: inject a prebuilt `reqwest` client, customize the client builder, or attach request/response hooks (`HttpHooks`).
- add an optional `tracing` feature for request-lifecycle instrumentation (URL build, send/response timing, retries, error classification).

### release
- releases now ship a CycloneDX SBOM.

## [0.3.1] - 2026-01-24

### cli
- add config file support at `~/.config/netbox-cli/config.toml` with named profiles, selectable via `--profile` (default `default`).
- add a `config` subcommand (`path`, `list`, `show`, `validate`).
- support `token_env` and `token_command` for secure token retrieval.
- add `--columns` for explicit table columns and `--max-columns` to cap auto-selected columns (default 6).

### docs
- document config profiles, and expand `--select` with examples for nested arrays and objects.

## [0.3.0] - 2026-01-24

### client
- add IPAM availability endpoints (available IPs/prefixes in a prefix, IPs in a range, VLANs in a group, ASNs in a range, plus the matching create operations).
- add core task management (`enqueue_task`, `stop_task`, `requeue_task`, `delete_task`, `sync_data_source`).
- add extras sync/render operations (config contexts, config templates, export templates, custom-field choices).
- add circuits and virtual-circuit termination path endpoints.
- add DCIM trace endpoints (interface, console port, console server port, power port, power outlet, power feed).
- add virtualization `render_vm_config`.

### cli
- add commands mirroring the new client endpoints: IPAM availability, core task management, extras sync/render, circuits termination paths, DCIM trace, and virtualization render-config.

## [0.2.1] - 2026-01-22

### client
- allow dynamic resource paths via `Resource::dynamic`, plus a `Client::resource` helper for ad-hoc endpoints.

## [0.2.0] - 2026-01-21

### client
- add a read-only GraphQL helper with query variables.
- add `scope_type` and `scope_id` to prefix create/update requests.

### cli
- add `json`, `yaml`, and `table` output formats, with automatic table shaping for paginated results.
- add `--select` for dot-path extraction and `--dry-run` for write operations (prints the full request).
- improve error messages with status, path, and request id when present.

### docs
- split user and developer docs, and refresh the readme for pre-release stability.

## [0.1.6] - 2026-01-20

### client
- add bulk create/update/patch/delete helpers on `Resource`, with `BulkUpdate`/`BulkDelete` wrappers.

## [0.1.5] - 2026-01-16

### client
- add field-patch request structs for DCIM and IPAM (custom fields, tags, device local context) to support typed patch calls.

## [0.1.4] - 2026-01-15

- fix OpenAPI generation incorrectly treating integer enums as string enums.

## [0.1.3] - 2026-01-14

- fix repository and homepage URLs in crates.io metadata.

## [0.1.2] - 2026-01-14

- add docs.rs link metadata for `netbox`.

## [0.1.1] - 2026-01-14

### docs
- add per-crate readme metadata so crates.io shows docs for each crate; refocus the CLI docs on end-user usage and examples; add local docs-build guidance.

### metadata
- add docs.rs metadata for `netbox-openapi`.

## [0.1.0] - 2026-01-14

### crates
- `netbox-openapi`: generated bindings for all NetBox endpoints and models.
- `netbox`: high-level client with typed resources, pagination, and a query builder.
- `netbox-cli`: CLI for listing, reading, and mutating resources.

### client features
- token auth, configurable timeouts, retries, and TLS verification.
- raw request support, OpenAPI config access, and direct HTTP client access.
- structured API errors with helpers.

### coverage
- full coverage across dcim, ipam, circuits, tenancy, extras, core, users, virtualization, vpn, wireless, and plugins, plus status and schema endpoints.

### docs
- readme and rustdoc coverage for the client and CLI, plus a contributing guide.

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
