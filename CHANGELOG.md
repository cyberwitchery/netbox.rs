# changelog

all notable changes are documented here.

format based on keep a changelog and semantic versioning.

## [unreleased]

### added
- openapi access helpers: `Client::openapi_config` and `netbox::openapi` module
- cli list commands for devices, prefixes, and ip addresses
- get retry logic for transient network errors and 429/5xx responses
- schema normalization and pinned generator version for reproducible codegen

### changed
- generated `netbox-openapi` crate uses `rustls-tls` with `reqwest` default features disabled

### fixed
- code generation failure caused by invalid enum values in the netbox schema

## [0.0.1] - tbd

### added

#### core features
- initial release of the netbox rust client
- token-based auth
- configurable http client with timeout and retry settings
- ssl verification control
- error handling with `thiserror`

#### api support
- dcim
  - devices: list, get, create, update, patch, delete
  - interfaces: list, get
- ipam
  - prefixes: list, get, create, update, patch, delete
  - ip addresses: list, get, create, update, patch, delete

#### developer experience
- pagination with `Paginator`
- query builder for filters
- typed request/response models

#### tooling
- openapi schema fetch script (`scripts/fetch_schema.sh`)
- code generation script (`scripts/generate.sh`)
- ci workflow
- formatting, linting, and tests

### documentation
- readme with quickstart and examples
- rustdoc api docs
- contributing guidelines
- script usage documentation

### internal
- workspace structure with three crates:
  - `netbox-openapi`: generated bindings
  - `netbox`: high-level client
  - `netbox-cli`: cli tool

[unreleased]: https://github.com/network-auto/netbox.rs/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/network-auto/netbox.rs/releases/tag/v0.0.1
