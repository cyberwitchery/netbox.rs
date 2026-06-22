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
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use url::Url;

#[derive(Serialize, Clone)]
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
    /// requests go through the shared retry loop and execute_request path, so
    /// they get retry-on-429, exponential backoff, hook callbacks, and tracing.
    pub async fn query(&self, query: &str, variables: Option<Value>) -> Result<Value> {
        let graphql_path = "graphql/";
        let body = GraphqlRequest { query, variables };

        self.client
            .retry_loop(Method::POST, graphql_path, true, |_attempt| {
                let body = body.clone();
                async move {
                    let url = self.graphql_url()?;
                    let request = self.client.http_client().post(url).json(&body);
                    let response = self
                        .client
                        .execute_request(&Method::POST, graphql_path, request)
                        .await?;

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
            })
            .await
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
    use crate::{ClientConfig, HttpHooks};
    use httpmock::prelude::HttpMockRequest;
    use httpmock::{Method::POST, MockServer};
    use reqwest::StatusCode;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

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

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn graphql_retries_on_429() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(2);
        let client = Client::new(config).unwrap();
        let api = GraphqlApi::new(client);

        let call_count = Arc::new(AtomicUsize::new(0));
        let counter = call_count.clone();
        let fail = server.mock(|when, then| {
            when.method(POST)
                .path("/graphql/")
                .is_true(move |_: &HttpMockRequest| counter.fetch_add(1, Ordering::SeqCst) == 0);
            then.status(429).body("rate limited");
        });
        let succeed = server.mock(|when, then| {
            when.method(POST).path("/graphql/");
            then.status(200)
                .json_body(json!({ "data": { "devices": [] } }));
        });

        let data = api.query("{ devices { name } }", None).await.unwrap();
        assert_eq!(data["devices"], json!([]));
        fail.assert_calls(1);
        succeed.assert_calls(1);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn graphql_retries_on_500() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(2);
        let client = Client::new(config).unwrap();
        let api = GraphqlApi::new(client);

        let call_count = Arc::new(AtomicUsize::new(0));
        let counter = call_count.clone();
        let fail = server.mock(|when, then| {
            when.method(POST)
                .path("/graphql/")
                .is_true(move |_: &HttpMockRequest| counter.fetch_add(1, Ordering::SeqCst) == 0);
            then.status(500).body("internal error");
        });
        let succeed = server.mock(|when, then| {
            when.method(POST).path("/graphql/");
            then.status(200)
                .json_body(json!({ "data": { "sites": [] } }));
        });

        let data = api.query("{ sites { name } }", None).await.unwrap();
        assert_eq!(data["sites"], json!([]));
        fail.assert_calls(1);
        succeed.assert_calls(1);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn graphql_does_not_retry_when_max_retries_is_zero() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = GraphqlApi::new(client);

        let mock = server.mock(|when, then| {
            when.method(POST).path("/graphql/");
            then.status(429).body("rate limited");
        });

        let err = api.query("{ devices { name } }", None).await.unwrap_err();
        assert!(matches!(err, Error::ApiError { status: 429, .. }));
        mock.assert_calls(1);
    }

    struct StatusCapture {
        statuses: Arc<Mutex<Vec<u16>>>,
    }

    impl HttpHooks for StatusCapture {
        fn on_response(
            &self,
            _method: &reqwest::Method,
            _path: &str,
            status: StatusCode,
            _duration: Duration,
        ) {
            self.statuses.lock().unwrap().push(status.as_u16());
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn graphql_invokes_hooks() {
        let server = MockServer::start();
        let statuses = Arc::new(Mutex::new(Vec::new()));
        let hook = StatusCapture {
            statuses: statuses.clone(),
        };
        let config = ClientConfig::new(server.base_url(), "token")
            .with_max_retries(0)
            .with_http_hooks(hook);
        let client = Client::new(config).unwrap();
        let api = GraphqlApi::new(client);

        server.mock(|when, then| {
            when.method(POST).path("/graphql/");
            then.status(200)
                .json_body(json!({ "data": { "devices": [] } }));
        });

        api.query("{ devices { name } }", None).await.unwrap();
        let captured = statuses.lock().unwrap().clone();
        assert_eq!(captured, vec![200]);
    }
}
