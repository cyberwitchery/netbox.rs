use netbox::extras::CreateTagRequest;
use netbox::{Client, ClientConfig, Result};
use std::time::{SystemTime, UNIX_EPOCH};

fn client_from_env() -> Result<Client> {
    let token = std::env::var("NETBOX_TOKEN").map_err(|_| {
        netbox::Error::Config("NETBOX_TOKEN is required to run this example".to_string())
    })?;
    let url = std::env::var("NETBOX_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let mut config = ClientConfig::new(url, token).with_max_retries(0);
    if std::env::var("NETBOX_INSECURE").as_deref() == Ok("1") {
        config = config.with_ssl_verification(false);
    }
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

    let tag_id = tag.id.expect("tag id missing from response") as u64;
    println!("Created tag {} with id {}", tag.slug, tag_id);

    client.extras().tags().delete(tag_id).await?;
    println!("Deleted tag {}", tag.slug);

    Ok(())
}
