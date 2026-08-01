//! local smoke tests against a live NetBox instance.
//!
//! run manually with:
//! NETBOX_TOKEN=... NETBOX_URL=http://localhost:8000 cargo test -p netbox --test smoke_local -- --ignored

use netbox::openapi::apis::status_api;
use netbox::{Client, ClientConfig, Page, QueryBuilder, Result};
use reqwest::Method;
use serde_json::{Value, json};

fn env_var(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn build_client() -> Option<Client> {
    let token = env_var("NETBOX_TOKEN")?;
    let url = env_var("NETBOX_URL").unwrap_or_else(|| "http://localhost:8000".to_string());
    let config = ClientConfig::new(url, token).with_max_retries(0);
    Client::new(config).ok()
}

fn id_from(value: &Value) -> Option<u64> {
    value.get("id").and_then(|v| v.as_u64())
}

async fn get_page(
    client: &Client,
    path: &str,
    query: Option<&QueryBuilder>,
) -> Result<Page<Value>> {
    let mut full_path = path.to_string();
    if let Some(query) = query {
        let query_string = serde_urlencoded::to_string(query)?;
        if !query_string.is_empty() {
            full_path = format!("{}?{}", path, query_string);
        }
    }
    let value = client.request_raw(Method::GET, &full_path, None).await?;
    Ok(serde_json::from_value(value)?)
}

async fn request_json(
    client: &Client,
    method: Method,
    path: &str,
    body: Option<&Value>,
) -> Result<Value> {
    client.request_raw(method, path, body).await
}

#[derive(Default)]
struct Created {
    tag_id: Option<u64>,
    tenant_group_id: Option<u64>,
    tenant_id: Option<u64>,
    config_context_id: Option<u64>,
    prefix_id: Option<u64>,
    ip_address_id: Option<u64>,
    wlan_group_id: Option<u64>,
    wlan_id: Option<u64>,
}

async fn cleanup(client: &Client, created: &Created) {
    if let Some(id) = created.wlan_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("wireless/wireless-lans/{}/", id),
            None,
        )
        .await;
    }
    if let Some(id) = created.wlan_group_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("wireless/wireless-lan-groups/{}/", id),
            None,
        )
        .await;
    }
    if let Some(id) = created.ip_address_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("ipam/ip-addresses/{}/", id),
            None,
        )
        .await;
    }
    if let Some(id) = created.prefix_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("ipam/prefixes/{}/", id),
            None,
        )
        .await;
    }
    if let Some(id) = created.config_context_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("extras/config-contexts/{}/", id),
            None,
        )
        .await;
    }
    if let Some(id) = created.tenant_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("tenancy/tenants/{}/", id),
            None,
        )
        .await;
    }
    if let Some(id) = created.tenant_group_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("tenancy/tenant-groups/{}/", id),
            None,
        )
        .await;
    }
    if let Some(id) = created.tag_id {
        let _ = request_json(
            client,
            Method::DELETE,
            &format!("extras/tags/{}/", id),
            None,
        )
        .await;
    }
}

async fn cleanup_existing(client: &Client) {
    if let Ok(tags) = get_page(client, "extras/tags/", None).await {
        for tag in tags
            .results
            .into_iter()
            .filter(|tag| tag.get("slug").and_then(Value::as_str) == Some("codex-smoke"))
        {
            if let Some(id) = id_from(&tag) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("extras/tags/{}/", id),
                    None,
                )
                .await;
            }
        }
    }

    if let Ok(groups) = get_page(client, "tenancy/tenant-groups/", None).await {
        for group in groups
            .results
            .into_iter()
            .filter(|group| group.get("slug").and_then(Value::as_str) == Some("codex-group"))
        {
            if let Some(id) = id_from(&group) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("tenancy/tenant-groups/{}/", id),
                    None,
                )
                .await;
            }
        }
    }

    if let Ok(tenants) = get_page(client, "tenancy/tenants/", None).await {
        for tenant in tenants
            .results
            .into_iter()
            .filter(|tenant| tenant.get("slug").and_then(Value::as_str) == Some("codex-tenant"))
        {
            if let Some(id) = id_from(&tenant) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("tenancy/tenants/{}/", id),
                    None,
                )
                .await;
            }
        }
    }

    if let Ok(contexts) = get_page(
        client,
        "extras/config-contexts/",
        Some(&QueryBuilder::new().filter("name", "codex-context")),
    )
    .await
    {
        for context in contexts.results {
            if let Some(id) = id_from(&context) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("extras/config-contexts/{}/", id),
                    None,
                )
                .await;
            }
        }
    }

    if let Ok(prefixes) = get_page(
        client,
        "ipam/prefixes/",
        Some(&QueryBuilder::new().filter("prefix", "10.10.0.0/24")),
    )
    .await
    {
        for prefix in prefixes.results {
            if let Some(id) = id_from(&prefix) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("ipam/prefixes/{}/", id),
                    None,
                )
                .await;
            }
        }
    }

    if let Ok(addresses) = get_page(
        client,
        "ipam/ip-addresses/",
        Some(&QueryBuilder::new().filter("address", "10.10.0.10/24")),
    )
    .await
    {
        for address in addresses.results {
            if let Some(id) = id_from(&address) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("ipam/ip-addresses/{}/", id),
                    None,
                )
                .await;
            }
        }
    }

    if let Ok(wlan_groups) = get_page(
        client,
        "wireless/wireless-lan-groups/",
        Some(&QueryBuilder::new().filter("slug", "codex-wlan-group")),
    )
    .await
    {
        for group in wlan_groups.results {
            if let Some(id) = id_from(&group) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("wireless/wireless-lan-groups/{}/", id),
                    None,
                )
                .await;
            }
        }
    }

    if let Ok(wlans) = get_page(
        client,
        "wireless/wireless-lans/",
        Some(&QueryBuilder::new().filter("ssid", "Codex Guest")),
    )
    .await
    {
        for wlan in wlans.results {
            if let Some(id) = id_from(&wlan) {
                let _ = request_json(
                    client,
                    Method::DELETE,
                    &format!("wireless/wireless-lans/{}/", id),
                    None,
                )
                .await;
            }
        }
    }
}

async fn run_smoke(client: &Client, created: &mut Created) -> Result<()> {
    eprintln!("smoke: status endpoint");
    let status = client.status().status().await?;
    assert!(
        status.contains_key("netbox-version"),
        "status missing netbox-version"
    );

    eprintln!("smoke: schema endpoint");
    let schema = client.schema().schema(Some("json"), None).await?;
    assert!(schema.contains_key("openapi"), "schema missing openapi");

    eprintln!("smoke: users config endpoint");
    let _config = client.users().config().await?;

    eprintln!("smoke: openapi status");
    let openapi = client.openapi_config()?;
    let openapi_status = status_api::status_retrieve(&openapi, None, None, None)
        .await
        .map_err(|err| netbox::Error::Config(format!("openapi status failed: {err}")))?;
    assert!(
        openapi_status.contains_key("netbox-version"),
        "openapi status missing netbox-version"
    );

    eprintln!("smoke: graphql introspection");
    let gql = client
        .graphql()
        .query("query { __schema { queryType { name } } }", None)
        .await?;
    assert!(
        gql.get("__schema").is_some(),
        "graphql introspection missing __schema"
    );

    eprintln!("smoke: create tag");
    let tag = request_json(
        client,
        Method::POST,
        "extras/tags/",
        Some(&json!({
            "name": "codex-smoke",
            "slug": "codex-smoke",
            "color": "00aaff",
            "description": "smoke test tag"
        })),
    )
    .await?;
    created.tag_id = id_from(&tag);
    if let Some(tag_id) = created.tag_id {
        let tag_model = client.extras().tags().get(tag_id).await?;
        assert_eq!(tag_model.slug, "codex-smoke");
    }

    eprintln!("smoke: list tags (typed list)");
    let tag_list = client
        .extras()
        .tags()
        .list(Some(QueryBuilder::new().filter("slug", "codex-smoke")))
        .await?;
    assert!(!tag_list.results.is_empty(), "expected tag list");

    eprintln!("smoke: paginate tags");
    let mut tag_pages = client
        .extras()
        .tags()
        .paginate(Some(QueryBuilder::new().limit(1)))?;
    let tag_page = tag_pages.next_page().await?;
    assert!(tag_page.is_some(), "expected at least one tag page");

    eprintln!("smoke: create tenant group");
    let tenant_group = request_json(
        client,
        Method::POST,
        "tenancy/tenant-groups/",
        Some(&json!({
            "name": "codex-group",
            "slug": "codex-group"
        })),
    )
    .await?;
    created.tenant_group_id = id_from(&tenant_group);

    eprintln!("smoke: create tenant");
    let tenant = request_json(
        client,
        Method::POST,
        "tenancy/tenants/",
        Some(&json!({
            "name": "codex-tenant",
            "slug": "codex-tenant",
            "group": created.tenant_group_id,
            "tags": [{"slug": "codex-smoke"}]
        })),
    )
    .await?;
    created.tenant_id = id_from(&tenant);

    eprintln!("smoke: create config context");
    let config_context = request_json(
        client,
        Method::POST,
        "extras/config-contexts/",
        Some(&json!({
            "name": "codex-context",
            "data": {"hello": "world"},
            "is_active": true,
            "tags": ["codex-smoke"]
        })),
    )
    .await?;
    created.config_context_id = id_from(&config_context);

    eprintln!("smoke: create prefix");
    let prefix = request_json(
        client,
        Method::POST,
        "ipam/prefixes/",
        Some(&json!({
            "prefix": "10.10.0.0/24",
            "status": "active",
            "tenant": created.tenant_id,
            "tags": [{"slug": "codex-smoke"}]
        })),
    )
    .await?;
    created.prefix_id = id_from(&prefix);

    eprintln!("smoke: create ip address");
    let ip_address = request_json(
        client,
        Method::POST,
        "ipam/ip-addresses/",
        Some(&json!({
            "address": "10.10.0.10/24",
            "status": "active",
            "tenant": created.tenant_id,
            "tags": [{"slug": "codex-smoke"}]
        })),
    )
    .await?;
    created.ip_address_id = id_from(&ip_address);

    eprintln!("smoke: create wireless lan group");
    let wlan_group = request_json(
        client,
        Method::POST,
        "wireless/wireless-lan-groups/",
        Some(&json!({
            "name": "codex-wlan-group",
            "slug": "codex-wlan-group",
            "tags": [{"slug": "codex-smoke"}]
        })),
    )
    .await?;
    created.wlan_group_id = id_from(&wlan_group);

    eprintln!("smoke: create wireless lan");
    let wlan = request_json(
        client,
        Method::POST,
        "wireless/wireless-lans/",
        Some(&json!({
            "ssid": "Codex Guest",
            "group": created.wlan_group_id,
            "status": "active",
            "tenant": created.tenant_id,
            "tags": [{"slug": "codex-smoke"}]
        })),
    )
    .await?;
    created.wlan_id = id_from(&wlan);

    eprintln!("smoke: patch tag");
    let patched = request_json(
        client,
        Method::PATCH,
        &format!("extras/tags/{}/", created.tag_id.unwrap_or_default()),
        Some(&json!({"description": "smoke test tag updated"})),
    )
    .await?;
    assert_eq!(patched["description"], "smoke test tag updated");

    eprintln!("smoke: list tenants");
    let tenants = get_page(
        client,
        "tenancy/tenants/",
        Some(&QueryBuilder::new().limit(2)),
    )
    .await?;
    assert!(!tenants.results.is_empty(), "expected at least one tenant");

    eprintln!("smoke: paginate tenants");
    let mut tenant_pages = client
        .tenancy()
        .tenants()
        .paginate(Some(QueryBuilder::new().limit(1)))?;
    let tenant_page = tenant_pages.next_page().await?;
    assert!(tenant_page.is_some(), "expected at least one tenant page");

    // smoke test for availability endpoints
    if let Some(prefix_id) = created.prefix_id {
        eprintln!("smoke: available IPs in prefix");
        let available = client.ipam().available_ips_in_prefix(prefix_id).await?;
        assert!(
            !available.is_empty(),
            "expected at least one available IP in prefix"
        );

        eprintln!("smoke: available child prefixes in prefix");
        let _available_prefixes = client
            .ipam()
            .available_prefixes_in_prefix(prefix_id)
            .await?;
        // may be empty for /24, that's ok
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn smoke_local_netbox_crud() {
    let Some(client) = build_client() else {
        eprintln!("NETBOX_TOKEN is not set; skipping smoke test");
        return;
    };

    let mut created = Created::default();
    cleanup_existing(&client).await;
    let result = run_smoke(&client, &mut created).await;
    cleanup(&client, &created).await;
    if let Err(err) = result {
        panic!("smoke test failed: {err}");
    }
}
