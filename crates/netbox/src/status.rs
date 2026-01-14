//! status endpoint for basic health and version info.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let status = client.status().status().await?;
//! println!("{}", status.len());
//! # Ok(())
//! # }
//! ```

use crate::error::Result;
use crate::Client;
use serde_json::Value;
use std::collections::HashMap;

/// status payload.
pub type Status = HashMap<String, Value>;

/// api for status endpoints.
#[derive(Clone)]
pub struct StatusApi {
    client: Client,
}

impl StatusApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// fetch netbox status.
    pub async fn status(&self) -> Result<Status> {
        self.client.get("status/").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::GET, MockServer};

    #[cfg_attr(miri, ignore)]

    #[tokio::test]
    async fn status_fetches_expected_path() {
        let server = MockServer::start();
        let base_url = server.base_url();
        let config = ClientConfig::new(&base_url, "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = StatusApi::new(client);

        let response = serde_json::json!({"netbox-version": "4.4.2"});
        server.mock(|when, then| {
            when.method(GET).path("/api/status/");
            then.status(200).json_body(response);
        });

        let status = api.status().await.unwrap();
        assert_eq!(
            status.get("netbox-version").and_then(Value::as_str),
            Some("4.4.2")
        );
    }
}
