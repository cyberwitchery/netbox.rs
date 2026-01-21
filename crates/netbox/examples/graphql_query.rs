use netbox::{Client, ClientConfig};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("NETBOX_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let token = std::env::var("NETBOX_TOKEN")?;
    let mut config = ClientConfig::new(url, token);
    if std::env::var("NETBOX_INSECURE").as_deref() == Ok("1") {
        config = config.with_ssl_verification(false);
    }

    let client = Client::new(config)?;
    let variables = json!({ "limit": 5 });
    let data = client
        .graphql()
        .query("query ($limit: Int!) { devices(limit: $limit) { name } }", Some(variables))
        .await?;

    println!("{}", serde_json::to_string_pretty(&data)?);
    Ok(())
}
