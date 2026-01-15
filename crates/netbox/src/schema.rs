//! schema endpoint for fetching the openapi document.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let schema = client.schema().schema(Some("json"), None).await?;
//! println!("{}", schema.len());
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// openapi schema payload.
pub type Schema = HashMap<String, Value>;

#[derive(Serialize)]
struct SchemaQuery<'a> {
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    r#format: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<&'a str>,
}

/// api for schema endpoints.
#[derive(Clone)]
pub struct SchemaApi {
    client: Client,
}

impl SchemaApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// fetch the openapi schema.
    pub async fn schema(&self, r#format: Option<&str>, lang: Option<&str>) -> Result<Schema> {
        let query = SchemaQuery { r#format, lang };
        self.client.get_with_params("schema/", &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::GET, MockServer};

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn schema_fetches_expected_path() {
        let server = MockServer::start();
        let base_url = server.base_url();
        let config = ClientConfig::new(base_url, "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = SchemaApi::new(client);

        let response = serde_json::json!({"openapi": "3.0.0"});
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/schema/")
                .query_param("format", "json");
            then.status(200).json_body(response);
        });

        let schema = api.schema(Some("json"), None).await.unwrap();
        assert_eq!(schema.get("openapi").and_then(Value::as_str), Some("3.0.0"));
    }
}
