use netbox::{Client, ClientConfig, QueryBuilder, Result};

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

#[tokio::main]
async fn main() -> Result<()> {
    let client = client_from_env()?;
    let tenants = client
        .tenancy()
        .tenants()
        .list(Some(QueryBuilder::new().limit(5)))
        .await?;

    println!("Tenant count: {}", tenants.count);
    for tenant in tenants.results {
        println!("- {} ({})", tenant.name, tenant.slug);
    }

    Ok(())
}
