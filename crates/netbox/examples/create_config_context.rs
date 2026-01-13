use netbox::extras::CreateConfigContextRequest;
use netbox::{Client, ClientConfig, Result};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

fn client_from_env() -> Result<Client> {
    let token = std::env::var("NETBOX_TOKEN").map_err(|_| {
        netbox::Error::Config("NETBOX_TOKEN is required to run this example".to_string())
    })?;
    let url = std::env::var("NETBOX_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let config = ClientConfig::new(url, token).with_max_retries(0);
    Client::new(config)
}

fn unique_suffix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = client_from_env()?;
    let suffix = unique_suffix();
    let name = format!("codex-context-{}", suffix);

    let context = client
        .extras()
        .config_contexts()
        .create(&CreateConfigContextRequest {
            name: name.clone(),
            weight: None,
            profile: None,
            description: None,
            is_active: Some(true),
            regions: None,
            site_groups: None,
            sites: None,
            locations: None,
            device_types: None,
            roles: None,
            platforms: None,
            cluster_types: None,
            cluster_groups: None,
            clusters: None,
            tenant_groups: None,
            tenants: None,
            tags: None,
            data_source: None,
            data: Some(json!({"example": true})),
        })
        .await?;

    let context_id = context.id.expect("config context id missing from response");
    println!("Created config context {} with id {}", context.name, context_id);

    client.extras().config_contexts().delete(context_id).await?;
    println!("Deleted config context {}", context.name);

    Ok(())
}
