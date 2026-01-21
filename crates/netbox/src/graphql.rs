//! graphql endpoint helper.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let data = client
//!     .graphql()
//!     .query("{ devices { name } }", None)
//!     .await?;
//! println!("{}", data);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::{Error, Result};
use serde::Serialize;
use serde_json::Value;
use url::Url;

#[derive(Serialize)]
struct GraphqlRequest<'a> {
    query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<Value>,
}

/// api for graphql queries.
#[derive(Clone)]
pub struct GraphqlApi {
    client: Client,
}

impl GraphqlApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// run a read-only graphql query.
    ///
    /// returns the `data` field when present, otherwise the raw response json.
    pub async fn query(&self, query: &str, variables: Option<Value>) -> Result<Value> {
        let url = self.graphql_url()?;
        let body = GraphqlRequest { query, variables };

        let response = self
            .client
            .http_client()
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(Error::from)?;

        let status = response.status();
        let body_text = response.text().await.map_err(Error::from)?;

        if !status.is_success() {
            return Err(Error::from_response(status, body_text));
        }

        if body_text.trim().is_empty() {
            return Ok(Value::Null);
        }

        let value: Value = serde_json::from_str(&body_text)?;
        if let Some(message) = graphql_error_message(&value) {
            return Err(Error::ApiError {
                status: status.as_u16(),
                message,
                body: body_text,
            });
        }

        Ok(value.get("data").cloned().unwrap_or(value))
    }

    fn graphql_url(&self) -> Result<Url> {
        let base = self.client.config().base_url.as_str().trim_end_matches('/');
        let url = format!("{}/graphql/", base);
        Url::parse(&url).map_err(Error::from)
    }
}

fn graphql_error_message(value: &Value) -> Option<String> {
    let errors = value.get("errors")?;
    let messages = match errors {
        Value::Array(items) if !items.is_empty() => items
            .iter()
            .filter_map(|item| item.get("message").and_then(Value::as_str))
            .map(|message| message.to_string())
            .collect::<Vec<_>>(),
        Value::Array(_) => Vec::new(),
        _ => vec![errors.to_string()],
    };

    if messages.is_empty() {
        None
    } else {
        Some(messages.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::POST, MockServer};
    use serde_json::json;

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn graphql_hits_expected_path() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = GraphqlApi::new(client);

        server.mock(|when, then| {
            when.method(POST).path("/graphql/");
            then.status(200)
                .json_body(json!({ "data": { "devices": [] } }));
        });

        let data = api.query("{ devices { name } }", None).await.unwrap();
        assert_eq!(data["devices"], json!([]));
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn graphql_surfaces_errors() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = GraphqlApi::new(client);

        server.mock(|when, then| {
            when.method(POST).path("/graphql/");
            then.status(200).json_body(json!({
                "errors": [{ "message": "bad query" }]
            }));
        });

        let err = api.query("{ bad }", None).await.unwrap_err();
        assert!(matches!(err, Error::ApiError { .. }));
        assert!(err.to_string().contains("bad query"));
    }
}
