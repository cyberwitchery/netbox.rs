# client library

this crate provides a typed, ergonomic client for the netbox rest api.

## install

```toml
[dependencies]
netbox = "0.0.1"
tokio = { version = "1.0", features = ["full"] }
```

## create a client

```rust
use netbox::{Client, ClientConfig};

let config = ClientConfig::new("https://netbox.example.com", "token");
let client = Client::new(config)?;
```

## auth and config

```rust
use std::time::Duration;
use netbox::ClientConfig;

let config = ClientConfig::new("https://netbox.example.com", "token")
    .with_timeout(Duration::from_secs(60))
    .with_max_retries(5)
    .with_ssl_verification(false);
```

## list and filter

```rust
use netbox::QueryBuilder;

let query = QueryBuilder::new()
    .filter("status", "active")
    .limit(50)
    .order_by("name");

let page = client.dcim().devices().list(Some(query)).await?;
println!("{}", page.count);
```

## paginate

```rust
let mut paginator = client.dcim().devices().paginate(None);
while let Some(page) = paginator.next_page().await? {
    for device in page.results {
        println!("{}", device.display);
    }
}
```

## create, update, delete

```rust
use netbox::dcim::CreateDeviceRequest;

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
let updated = client.dcim().devices().patch(device.id, &serde_json::json!({"status": "offline"})).await?;
client.dcim().devices().delete(device.id).await?;
```

## status and schema

```rust
let status = client.status().status().await?;
let schema = client.schema().schema(Some("json"), None).await?;
```

## connected device

```rust
let devices = client
    .dcim()
    .connected_device("leaf-01", "Ethernet1")
    .await?;
```

## branching plugin

```rust
let branches = client.plugins().branches().list(None).await?;
```

## error handling

```rust
use netbox::Error;

match client.dcim().devices().get(999).await {
    Ok(device) => println!("{}", device.display),
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
```

## raw api access

use this when you need an endpoint not yet wrapped by the high-level client.

```rust
use netbox::openapi::apis::dcim_api;

let openapi_config = client.openapi_config();
let device = dcim_api::dcim_devices_retrieve(&openapi_config, 42).await?;
println!("{}", device.display);
```

## coverage summary

high-level resource coverage exists for dcim, circuits, core, extras, ipam, tenancy, users, virtualization, vpn, and wireless. additional endpoints include status, schema, connected-device, and netbox-branching plugin resources.
