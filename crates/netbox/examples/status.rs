use netbox::{Client, ClientConfig};

fn build_client() -> Result<Client, Box<dyn std::error::Error>> {
    let base_url = std::env::var("NETBOX_URL").unwrap_or_else(|_| "http://localhost:8000".into());
    let token = std::env::var("NETBOX_TOKEN")?;
    let mut config = ClientConfig::new(base_url, token);
    if std::env::var("NETBOX_INSECURE").as_deref() == Ok("1") {
        config = config.with_ssl_verification(false);
    }
    Ok(Client::new(config)?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client()?;
    let status = client.status().status().await?;
    let netbox_version = status
        .get("netbox-version")
        .and_then(|value| value.as_str())
        .unwrap_or("<unknown>");
    let workers = status
        .get("rq-workers-running")
        .and_then(|value| value.as_i64())
        .unwrap_or(0);
    println!("netbox-version: {}", netbox_version);
    println!("rq-workers-running: {}", workers);
    Ok(())
}
