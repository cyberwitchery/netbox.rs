# client library

this crate provides a typed, ergonomic client for the netbox 4.x rest api.

## features

- **typed resources** - strongly-typed request/response models for all netbox endpoints
- **crud operations** - list, get, create, update, patch, delete with query filtering
- **pagination** - iterator-based pagination with automatic page fetching
- **bulk operations** - batch create, update, patch, and delete
- **specialized endpoints** - ipam availability, dcim cable tracing, config rendering, task management
- **graphql** - read-only graphql query support
- **error handling** - structured errors with status codes and api messages

## modules

the client exposes netbox endpoints through typed module accessors:

- `client.dcim()` - devices, interfaces, sites, racks, cables, power
- `client.ipam()` - prefixes, ip addresses, vlans, vrfs, asns
- `client.circuits()` - providers, circuits, terminations
- `client.virtualization()` - clusters, virtual machines, vm interfaces
- `client.tenancy()` - tenants, contacts
- `client.extras()` - tags, custom fields, config contexts, webhooks
- `client.core()` - background tasks, data sources, jobs
- `client.users()` - users, groups, tokens, permissions
- `client.vpn()` - tunnels, ike/ipsec policies, l2vpns
- `client.wireless()` - wireless lans, links
- `client.plugins()` - netbox-branching plugin support

each module returns typed `Resource<T>` handles with standard crud methods, plus specialized action methods where applicable.

## resource pattern

all netbox endpoints follow the same pattern. access a module, then a resource, then call an action:

```text
client.{module}().{resource}().{action}()
```

every `Resource<T>` provides these standard methods:

| method | http | description |
|--------|------|-------------|
| `list(query)` | GET | fetch paginated results with optional filters |
| `get(id)` | GET | fetch a single item by id |
| `create(body)` | POST | create a new item |
| `update(id, body)` | PUT | full replacement update |
| `patch(id, body)` | PATCH | partial update |
| `delete(id)` | DELETE | remove an item |
| `bulk_create(items)` | POST | create multiple items |
| `bulk_update(items)` | PUT | full update multiple items |
| `bulk_patch(items)` | PATCH | partial update multiple items |
| `bulk_delete(items)` | DELETE | remove multiple items |
| `paginate(query)` | GET | returns an iterator for all pages |

## query filtering

use `QueryBuilder` to filter, sort, and limit results:

```text
QueryBuilder::new()
    .filter("key", "value")     // add a filter
    .filter("status", "active") // filters are additive
    .search("term")             // full-text search
    .order_by("name")           // sort ascending
    .order_by("-created")       // sort descending (prefix with -)
    .limit(50)                  // max results per page
    .offset(100)                // skip first N results
```

pass the builder to `list()` or `paginate()` to apply filters.

## install

```toml
[dependencies]
netbox = "0.3"
tokio = { version = "1.0", features = ["full"] }
```

optional tracing instrumentation:

```toml
[dependencies]
netbox = { version = "0.3", features = ["tracing"] }
tokio = { version = "1.0", features = ["full"] }
tracing-subscriber = "0.3"
```

tiny runtime setup example:

```rust,ignore
use netbox::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("netbox=trace")
        .init();

    let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
    let _ = client.status().status().await?;
    Ok(())
}
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

## http customization hooks

for cross-cutting behavior, you can inject a prebuilt `reqwest::Client`,
customize the internal client builder, and attach request/response hooks.

```rust,no_run
use netbox::{Client, ClientConfig, HttpHooks};
use reqwest::{Method, Request, StatusCode};
use std::time::Duration;

struct MetricsHook;

impl HttpHooks for MetricsHook {
    fn on_request(&self, _method: &Method, _path: &str, request: &mut Request) -> netbox::Result<()> {
        request
            .headers_mut()
            .insert("x-client-hook", "enabled".parse().expect("valid header value"));
        Ok(())
    }

    fn on_response(&self, method: &Method, path: &str, status: StatusCode, duration: Duration) {
        println!("{method} {path} -> {status} in {duration:?}");
    }
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let prebuilt = reqwest::Client::builder()
        .pool_max_idle_per_host(4)
        .build()?;

    let config = ClientConfig::new("https://netbox.example.com", "token")
        .with_http_client(prebuilt)
        .with_http_hooks(MetricsHook);

    let _client = Client::new(config)?;
    Ok(())
}
```

precedence and behavior:
- `with_http_client(...)` takes precedence over `with_http_client_builder(...)`.
- hooks run for all client-driven HTTP requests (`Resource<T>`, special endpoint helpers, and `request_raw`).
- hooks are additive with tracing; they can be used independently or together.

safety notes:
- avoid logging or exporting authentication tokens, cookies, or full sensitive payloads from hooks.
- prefer additive mutations (headers/metadata) over replacing request method/url/body unexpectedly.
- keep hook logic lightweight; slow hooks directly add latency to every request.

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

note: for local dev instances with self-signed certs, call `with_ssl_verification(false)` in your config.
note: `paginate` returns `Result<Paginator<T>>`. handle errors before calling `next_page`.

## dynamic resources

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let resource = client.resource("ipam/vrfs/");
let page = resource.list(None).await?;
println!("{}", page.count);
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

## bulk operations

```rust,no_run
use netbox::{BulkDelete, BulkUpdate, Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;

let updates = vec![
    BulkUpdate::new(1, serde_json::json!({"status": "offline"})),
    BulkUpdate::new(2, serde_json::json!({"status": "offline"})),
];
let _updated = client.dcim().devices().bulk_patch(&updates).await?;

let deletes = vec![BulkDelete::new(1), BulkDelete::new(2)];
client.dcim().devices().bulk_delete(&deletes).await?;
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

## graphql (read-only)

returns the `data` field when present.

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let data = client
    .graphql()
    .query("{ devices { name } }", None)
    .await?;
println!("{}", data);
# Ok(())
# }
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

## ipam availability

query and allocate available IPs, prefixes, VLANs, and ASNs.

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;

// list available IPs in a prefix
let available = client.ipam().available_ips_in_prefix(1).await?;
println!("{} IPs available", available.len());

// allocate IPs from a prefix
let requests = vec![serde_json::json!({"description": "allocated via api"})];
let created = client.ipam().create_available_ips_in_prefix(1, &requests).await?;
println!("created {} IPs", created.len());

// list available VLANs in a group
let vlans = client.ipam().available_vlans_in_group(1).await?;
println!("{} VLANs available", vlans.len());
# Ok(())
# }
```

## dcim trace

trace cable paths for interfaces and power components.

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let trace = client.dcim().trace_interface(42).await?;
println!("{:?}", trace);
# Ok(())
# }
```

## core task management

manage background tasks and sync data sources.

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;

// sync a data source
let source = client.core().sync_data_source(1).await?;
println!("{:?}", source);

// manage background tasks
let task = client.core().enqueue_task("abc123").await?;
println!("{:?}", task);
# Ok(())
# }
```

## extras sync and render

sync config contexts/templates from data sources and render templates.

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;

// sync a config context from its data source
let context = client.extras().sync_config_context(1).await?;
println!("{:?}", context);

// render a config template
let output = client.extras().render_config_template(1).await?;
println!("{}", output);
# Ok(())
# }
```

## virtualization render config

render configuration for virtual machines.

```rust,no_run
use netbox::{Client, ClientConfig};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
let config = client.virtualization().render_vm_config(1).await?;
println!("{}", config);
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
let device = dcim_api::dcim_devices_retrieve(&openapi_config, 42, None, None, None).await?;
println!("{}", device.display.as_deref().unwrap_or("<unknown>"));
# Ok(())
# }
```

## coverage summary

high-level resource coverage exists for dcim, circuits, core, extras, ipam, tenancy, users, virtualization, vpn, and wireless. additional endpoints include status, schema, connected-device, graphql (read-only), and netbox-branching plugin resources.

action endpoints include ipam availability queries (prefixes, ip-ranges, vlan-groups, asn-ranges), dcim cable tracing, core task management and data source sync, extras config sync/render operations, and virtualization vm config rendering.
