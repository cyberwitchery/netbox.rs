# agents

guidance for automated agents working in this repo.

## project overview
- `netbox` is a rust client for the netbox 4.x rest api.
- workspace:
  - `crates/netbox`: public client crate.
  - `crates/netbox-openapi`: generated bindings. avoid manual edits.
  - `crates/netbox-cli`: cli for manual testing.
  - `scripts/`: schema fetch + codegen helpers.

## workspace layout
- root `Cargo.toml`: workspace definition.
- `crates/netbox/src/`:
  - `client.rs`: http client + request handling.
  - `config.rs`: base url, token, timeouts, retries.
  - `error.rs`: error types.
  - `pagination.rs`: `Page<T>` + `Paginator<T>`.
  - `query.rs`: query builder.
  - `dcim/`, `ipam/`: api modules.
- `crates/netbox-openapi/src/`: generated api bindings.
- `crates/netbox-cli/src/main.rs`: cli entrypoint.

## local dev commands
- build: `cargo build`
- tests: `cargo test`
- lint: `cargo clippy --all-targets --all-features`
- format: `cargo fmt --all`
- assurance: `./scripts/run_assurance.sh`
- static analysis: `./scripts/run_static.sh`

## code generation workflow

generated bindings live under `crates/netbox-openapi`. do not hand-edit.

for a containerized netbox setup, follow:
https://github.com/netbox-community/netbox-docker

before running codegen, confirm with the user that netbox is running.

once netbox is running locally:
1. fetch schema:
   ```bash
   ./scripts/fetch_schema.sh http://localhost:8000
   ```
2. generate code:
   ```bash
   ./scripts/generate.sh
   ```
3. validate:
   ```bash
   cargo test
   ```

## agent checklist
- confirm netbox is running and reachable.
- do not start netbox containers; the user manages that.
- ask for the base url if not `http://localhost:8000`.
- confirm `NETBOX_TOKEN` is set before smoke tests.

## api wrapper patterns

when adding a new endpoint wrapper in `crates/netbox/src/{dcim,ipam}/`:
- provide `list`, `paginate`, `get`, `create`, `patch`, `delete`.
- use `QueryBuilder` for filters; accept `Option<QueryBuilder>` in list/paginate.
- return `Page<T>` for list and `Result<T>` for single-item endpoints.
- keep signatures consistent.
- update `mod.rs` to export the api type and request/response structs.
- note: `Resource::paginate` returns `Result<Paginator<T>>`.

## documentation expectations
- add rustdoc for public apis and examples.
- update `README.md` and `CHANGELOG.md` for user-visible changes.
- update `docs/client.md` and `crates/netbox/docs/client.md` for client behavior changes.

## testing guidance
- add unit tests for new helper logic.
- prefer lightweight tests in `crates/netbox/src/*`.
- avoid network calls unless explicitly requested.
- smoke tests live in `crates/netbox/tests/smoke_local.rs` and are ignored by default.

## ground rules
- run tests before reporting a feature complete.
- call out coverage added or updated.
- if tests cannot be run, explain why.

## safety and conventions
- avoid breaking changes without discussion.
- keep client configuration validation strict.
- surface http status and response body on errors.
- keep ascii-only content unless a file already uses unicode.

## useful references
- `README.md`
- `CONTRIBUTING.md`
- `scripts/README.md`

## current behavior notes
- `Client::openapi_config()` returns `Result<Configuration>` and propagates reqwest builder errors.
- `QueryBuilder` serializes as repeated key/value pairs for use with `serde_urlencoded`.
- static analysis: `scripts/run_static.sh` sets `CARGO_HOME`, runs clippy with `--no-deps`, uses `PROPTEST_CASES=1`, and runs miri with isolation disabled.
- `.cargo/audit.toml` is tracked; other `.cargo` content is ignored.
