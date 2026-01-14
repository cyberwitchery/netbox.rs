# netbox api generation scripts

scripts for generating the netbox openapi client.

## workflow

### 1. start netbox

use the official netbox-docker setup:
https://github.com/netbox-community/netbox-docker

wait until netbox is ready.

### 2. fetch openapi schema

```bash
./scripts/fetch_schema.sh http://localhost:8000
```

this writes `scripts/openapi-schema.json`.

### 3. generate rust bindings

```bash
./scripts/generate.sh
```

this writes to `crates/netbox-openapi/src/`.

the generated code is committed so users do not need java/npm/docker to build.

the generator uses a pinned image by default:
`openapitools/openapi-generator-cli:v6.6.0`.

override with:

```bash
OPENAPI_GENERATOR_VERSION=v6.6.0 ./scripts/generate.sh
OPENAPI_GENERATOR_IMAGE=openapitools/openapi-generator-cli:v6.6.0 ./scripts/generate.sh
```

notes about the generator:
- uses workspace version as `packageVersion`
- normalizes schema enums with value `"---------"` to `"none"`
- injects crate-level allow attributes in `crates/netbox-openapi/src/lib.rs`
- normalizes generated `Cargo.toml` for `rustls-tls`

## update for a new netbox version

1. update the netbox-docker image tag
2. run fetch + generate
3. update workspace version
4. run `cargo test`
5. update `CHANGELOG.md`
6. commit

## manual schema download

```bash
curl -fsSL https://netbox.example.com/api/schema/?format=json \
  -H "Authorization: Token YOUR_TOKEN" \
  -o scripts/openapi-schema.json
```

## generator options

we use openapi generator with the `rust` generator. to customize, edit `scripts/generate.sh` and add properties like:
- `packageName`
- `useSingleRequestParameter`

see https://openapi-generator.tech/docs/generators/rust

## local release script

run the local release checklist with:

```bash
./scripts/release_local.sh
```

set `SKIP_COVERAGE=1` to skip coverage.

notes:
- `netbox-openapi` must be published before `netbox`
- the script packages `netbox` with `--no-verify` because the dependency is not on crates.io yet

## local assurance script

run the local assurance checks with:

```bash
./scripts/run_assurance.sh
```

this runs:
- `cargo doc --workspace --all-features --no-deps`
- `cargo llvm-cov --workspace --all-features --ignore-filename-regex 'crates/netbox-openapi' --fail-under-lines 75`
- `./scripts/run_smoke.sh`
- `./scripts/run_cli_smoke.sh`

requirements:
- `cargo-llvm-cov` installed
- `NETBOX_TOKEN` set for smoke tests

## local static analysis script

run the static checks with:

```bash
./scripts/run_static.sh
```

this runs:
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo audit`
- `cargo +nightly miri test`

requirements:
- `cargo-audit` installed
- `miri` component installed
- nightly toolchain available for miri
