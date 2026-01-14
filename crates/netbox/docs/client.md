# client library

this crate provides a typed, ergonomic client for the netbox rest api.

## install

```toml
[dependencies]
netbox = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## create a client

```rust,no_run
use netbox::{Client, ClientConfig};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("https://netbox.example.com", "token");
    let _client = Client::new(config)?;
    Ok(())
}
```

## auth and config

```rust,no_run
use std::time::Duration;
use netbox::ClientConfig;
use reqwest::header::{HeaderName, HeaderValue};

fn example() {
    let _config = ClientConfig::new("https://netbox.example.com", "token")
        .with_timeout(Duration::from_secs(60))
        .with_max_retries(5)
        .with_ssl_verification(false)
        .with_header(
            HeaderName::from_static("x-netbox-client"),
            HeaderValue::from_static("custom"),
        );
}
```

## http client access

```rust,no_run
use netbox::{Client, ClientConfig};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
    let http = client.http_client();
    let _ = http;
    Ok(())
}
```

## list and filter

```rust,no_run
use netbox::{Client, ClientConfig, QueryBuilder};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let query = QueryBuilder::new()
    .filter("status", "active")
    .limit(50)
    .order_by("name");

let page = client.dcim().devices().list(Some(query)).await?;
println!("{}", page.count);
# Ok(())
# }
```

## paginate

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let mut paginator = client.dcim().devices().paginate(None)?;
while let Some(page) = paginator.next_page().await? {
    for device in page.results {
        let display = device.display.as_deref().unwrap_or("<unknown>");
        println!("{display}");
    }
}
# Ok(())
# }
```

## create, update, delete

```rust,no_run
use netbox::{Client, ClientConfig};
use netbox::dcim::CreateDeviceRequest;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let request = CreateDeviceRequest {
    name: "switch-01".to_string(),
    device_type: 1,
    role: 1,
    site: 1,
    status: Some("active".to_string()),
    serial: None,
    asset_tag: None,
    tags: None,
};

let device = client.dcim().devices().create(&request).await?;
let device_id = device.id.expect("device id missing from response") as u64;
let _updated = client
    .dcim()
    .devices()
    .patch(device_id, &serde_json::json!({"status": "offline"}))
    .await?;
client.dcim().devices().delete(device_id).await?;
# Ok(())
# }
```

## status and schema

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let status = client.status().status().await?;
let schema = client.schema().schema(Some("json"), None).await?;
# Ok(())
# }
```

## smoke tests

local-only smoke tests for the client live in `crates/netbox/tests/smoke_local.rs` and are ignored by default.

run them with:

```bash
./scripts/run_smoke.sh
```

## connected device

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let devices = client
    .dcim()
    .connected_device("leaf-01", "Ethernet1")
    .await?;
println!("{}", devices.len());
# Ok(())
# }
```

## branching plugin

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let branches = client.plugins().branches().list(None).await?;
println!("{}", branches.count);
# Ok(())
# }
```

## error handling

```rust,no_run
use netbox::{Client, ClientConfig, Error};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
match client.dcim().devices().get(999u64).await {
    Ok(device) => println!("{}", device.display.as_deref().unwrap_or("<unknown>")),
    Err(Error::ApiError { status, .. }) if status == 404 => {
        println!("not found");
    }
    Err(e) if e.is_auth_error() => {
        println!("auth failed: {}", e);
    }
    Err(e) => {
        println!("error: {}", e);
    }
}
# Ok(())
# }
```

## raw api access

use this when you need an endpoint not yet wrapped by the high-level client.

```rust,no_run
use netbox::{Client, ClientConfig};
use netbox::openapi::apis::dcim_api;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let openapi_config = client.openapi_config()?;
let device = dcim_api::dcim_devices_retrieve(&openapi_config, 42).await?;
println!("{}", device.display.as_deref().unwrap_or("<unknown>"));
# Ok(())
# }
```

## coverage summary

high-level resource coverage exists for dcim, circuits, core, extras, ipam, tenancy, users, virtualization, vpn, and wireless. additional endpoints include status, schema, connected-device, and netbox-branching plugin resources.
