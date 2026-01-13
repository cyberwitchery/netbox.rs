use netbox::extras::CreateTagRequest;
use netbox::{Client, ClientConfig, Result};
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
    let name = format!("codex-tag-{}", suffix);

    let tag = client
        .extras()
        .tags()
        .create(&CreateTagRequest {
            name: name.clone(),
            slug: name.clone(),
            color: Some("00aaff".to_string()),
            description: Some("example tag".to_string()),
            weight: None,
            object_types: None,
        })
        .await?;

    println!("Created tag {} with id {}", tag.slug, tag.id);

    client.extras().tags().delete(tag.id).await?;
    println!("Deleted tag {}", tag.slug);

    Ok(())
}
