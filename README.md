# netbox.rs

rust client for the netbox 4.x rest api. it was co-evolved using ai. pre-release, but stable and improving.

[![ci](https://github.com/cyberwitchery/netbox.rs/workflows/CI/badge.svg)](https://github.com/cyberwitchery/netbox.rs/actions)
[![crates.io](https://img.shields.io/crates/v/netbox.svg)](https://crates.io/crates/netbox)
[![docs.rs](https://docs.rs/netbox/badge.svg)](https://docs.rs/netbox)

## features

- ergonomic, typed api
- automatic pagination
- query builder for filters
- token auth
- configurable timeouts, retries, ssl
- graphql (read-only) helper
- documented examples
- unit tests and smoke tests

## install

add to `Cargo.toml`:

```toml
[dependencies]
netbox = "0.2.0"
tokio = { version = "1.0", features = ["full"] }
```

## version compatibility

current releases are pre-1.0.0 and do not track netbox versions.
this client targets netbox 4.x.
patch releases include fixes and client improvements.

## quick start

```rust
use netbox::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("https://netbox.example.com", "your-api-token");
    let client = Client::new(config)?;

    let devices = client.dcim().devices().list(None).await?;
    for device in devices.results {
        println!("device: {} (id: {})", device.display, device.id);
    }

    Ok(())
}
```

## examples

for a fuller client guide, see `crates/netbox/docs/client.md`.

for a docs index, see `docs/index.md`.

for runnable examples, see `crates/netbox/examples/README.md`.

### auth

```rust
use netbox::ClientConfig;

let config = ClientConfig::new("https://netbox.example.com", "token")
    .with_timeout(std::time::Duration::from_secs(60))
    .with_max_retries(5)
    .with_ssl_verification(false);

let client = Client::new(config)?;
```

### list with filters

```rust
use netbox::QueryBuilder;

let query = QueryBuilder::new()
    .filter("site", "dc1")
    .filter("status", "active")
    .limit(50)
    .order_by("name");

let devices = client.dcim().devices().list(Some(query)).await?;
println!("found {} devices", devices.count);
```

### pagination

```rust
let mut paginator = client.dcim().devices().paginate(None)?;

while let Some(page) = paginator.next_page().await? {
    for device in page.results {
        println!("{}", device.display);
    }
}

let all_devices = client
    .dcim()
    .devices()
    .paginate(None)?
    .collect_all()
    .await?;
```

### bulk operations

```rust
use netbox::{BulkDelete, BulkUpdate};

let updates = vec![
    BulkUpdate::new(1, serde_json::json!({"status": "offline"})),
    BulkUpdate::new(2, serde_json::json!({"status": "offline"})),
];
client.dcim().devices().bulk_patch(&updates).await?;

let deletes = vec![BulkDelete::new(1), BulkDelete::new(2)];
client.dcim().devices().bulk_delete(&deletes).await?;
```

### status and schema

```rust
let status = client.status().status().await?;
println!("netbox version: {:?}", status.get("netbox-version"));

let schema = client.schema().schema(Some("json"), None).await?;
println!("schema keys: {}", schema.len());
```

### graphql (read-only)

```rust
let data = client
    .graphql()
    .query("{ devices { name } }", None)
    .await?;
println!("graphql data: {}", data);
```

### connected device lookup

```rust
let devices = client
    .dcim()
    .connected_device("leaf-01", "Ethernet1")
    .await?;
println!("found {} devices", devices.len());
```

### branching plugin

```rust
let branches = client.plugins().branches().list(None).await?;
println!("branches: {}", branches.count);
```

### error handling

```rust
use netbox::Error;

match client.dcim().devices().get(999).await {
    Ok(device) => println!("found: {}", device.display),
    Err(Error::ApiError { status, .. }) if status == 404 => {
        println!("device not found");
    }
    Err(e) if e.is_auth_error() => {
        println!("auth failed: {}", e);
    }
    Err(e) => {
        println!("error: {}", e);
    }
}
```

## cli

`netbox-cli` is a full-featured cli for the netbox api. it covers standard crud resources and exposes a raw mode for any endpoint.

see `crates/netbox-cli/docs/cli.md` for a complete guide.

install:

```bash
cargo install netbox-cli
```

quickstart:

```bash
netbox-cli --url https://netbox.example.com --token $TOKEN dcim devices list
```

common commands:

```bash
netbox-cli dcim devices list
netbox-cli ipam prefixes list
netbox-cli vpn tunnels list
```

create or update with json:

```bash
netbox-cli circuits circuits create --json '{"cid":"CIR-1001","provider":1,"type":1}'
netbox-cli virtualization virtual-machines update 42 --file vm-update.json
```

raw requests:

```bash
netbox-cli raw --method GET --path dcim/devices/ --query "name=leaf-1" --query "limit=5"
netbox-cli raw --method POST --path ipam/vrfs/ --json '{"name":"blue","rd":"65000:100"}'
```

output formats and selection:

```bash
netbox-cli dcim devices list --output table
netbox-cli dcim devices list --select results.name
```

dry run for writes:

```bash
netbox-cli dcim devices create --json '{"name":"switch-01","device_type":1,"role":1,"site":1}' --dry-run
```

graphql:

```bash
netbox-cli graphql --query '{ devices { name } }'
```

run `netbox-cli --help` for all subcommands.

## api coverage

all core netbox modules are covered via typed `Resource<T>` wrappers. additional endpoints include status, schema, connected-device, and netbox-branching plugin resources.

note: `paginate` returns `Result<Paginator<T>>`; handle errors before calling `next_page`.

for endpoints not wrapped yet, use the generated openapi client:

```rust
use netbox::{Client, ClientConfig};
use netbox::openapi::apis::dcim_api;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let config = ClientConfig::new("https://netbox.example.com", "token");
let client = Client::new(config)?;

let openapi_config = client.openapi_config()?;
let device = dcim_api::dcim_devices_retrieve(&openapi_config, 42).await?;
println!("device: {}", device.display);
# Ok(())
# }
```

## development

### prerequisites

- rust 1.85+
- a netbox instance for schema generation and smoke tests

### setup

```bash
git clone https://github.com/network-auto/netbox.rs
cd netbox.rs
cargo test
cargo clippy --all-targets --all-features
cargo fmt --all
```

### docs

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo doc --workspace --all-features --no-deps
```

open `target/doc/netbox/index.html` for the library docs.

### regenerate api bindings

see `scripts/README.md` for details.

basic flow:

```bash
./scripts/fetch_schema.sh http://localhost:8000
./scripts/generate.sh
cargo test
```

for container setup, follow the netbox-docker instructions:
https://github.com/netbox-community/netbox-docker

## contributing

see `CONTRIBUTING.md`.

## license

mit. see `LICENSE`.

## resources

- https://docs.netbox.dev/
- https://docs.netbox.dev/en/stable/integrations/rest-api/
- https://docs.rs/netbox
