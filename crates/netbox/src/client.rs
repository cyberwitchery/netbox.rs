//! NetBox API client

use crate::config::ClientConfig;
use crate::core::CoreApi;
use crate::circuits::CircuitsApi;
use crate::dcim::DcimApi;
use crate::error::{Error, Result};
use crate::extras::ExtrasApi;
use crate::ipam::IpamApi;
use crate::plugins::PluginsApi;
use crate::schema::SchemaApi;
use crate::status::StatusApi;
use crate::tenancy::TenancyApi;
use crate::users::UsersApi;
use crate::virtualization::VirtualizationApi;
use crate::vpn::VpnApi;
use crate::wireless::WirelessApi;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// The main NetBox API client
///
/// # Example
///
/// ```no_run
/// use netbox::{Client, ClientConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ClientConfig::new("https://netbox.example.com", "your-api-token");
/// let client = Client::new(config)?;
///
/// // Use the client to access different API modules
/// // let devices = client.dcim().devices().list().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Client {
    config: Arc<ClientConfig>,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a new NetBox client
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid or the HTTP client cannot be created.
    pub fn new(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        // Build default headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Token {}", config.token))
                .map_err(|e| Error::Config(format!("Invalid token format: {}", e)))?,
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&config.user_agent)
                .map_err(|e| Error::Config(format!("Invalid user agent: {}", e)))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Build HTTP client
        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .danger_accept_invalid_certs(!config.verify_ssl)
            .build()
            .map_err(|e| Error::Config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config: Arc::new(config),
            http_client,
        })
    }

    /// Access DCIM (Data Center Infrastructure Management) API endpoints
    pub fn dcim(&self) -> DcimApi {
        DcimApi::new(self.clone())
    }

    /// Access circuits API endpoints
    pub fn circuits(&self) -> CircuitsApi {
        CircuitsApi::new(self.clone())
    }

    /// Access core API endpoints
    pub fn core(&self) -> CoreApi {
        CoreApi::new(self.clone())
    }

    /// Access extras API endpoints
    pub fn extras(&self) -> ExtrasApi {
        ExtrasApi::new(self.clone())
    }

    /// Access IPAM (IP Address Management) API endpoints
    pub fn ipam(&self) -> IpamApi {
        IpamApi::new(self.clone())
    }

    /// Access plugin API endpoints
    pub fn plugins(&self) -> PluginsApi {
        PluginsApi::new(self.clone())
    }

    /// Access the OpenAPI schema endpoint
    pub fn schema(&self) -> SchemaApi {
        SchemaApi::new(self.clone())
    }

    /// Access the NetBox status endpoint
    pub fn status(&self) -> StatusApi {
        StatusApi::new(self.clone())
    }

    /// Access tenancy API endpoints
    pub fn tenancy(&self) -> TenancyApi {
        TenancyApi::new(self.clone())
    }

    /// Access users API endpoints
    pub fn users(&self) -> UsersApi {
        UsersApi::new(self.clone())
    }

    /// Access virtualization API endpoints
    pub fn virtualization(&self) -> VirtualizationApi {
        VirtualizationApi::new(self.clone())
    }

    /// Access VPN API endpoints
    pub fn vpn(&self) -> VpnApi {
        VpnApi::new(self.clone())
    }

    /// Access wireless API endpoints
    pub fn wireless(&self) -> WirelessApi {
        WirelessApi::new(self.clone())
    }

    /// Get the client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Build a configuration for the generated OpenAPI client.
    pub fn openapi_config(&self) -> netbox_openapi::apis::configuration::Configuration {
        let client = reqwest_legacy::Client::builder()
            .no_proxy()
            .build()
            .unwrap_or_else(|_| reqwest_legacy::Client::new());

        netbox_openapi::apis::configuration::Configuration {
            base_path: self.config.base_url.as_str().trim_end_matches('/').to_string(),
            user_agent: Some(self.config.user_agent.clone()),
            client,
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: Some(netbox_openapi::apis::configuration::ApiKey {
                prefix: Some("Token".to_string()),
                key: self.config.token.clone(),
            }),
        }
    }

    /// Make a GET request to the API
    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_with_retries(Method::GET, path, None::<&()>)
            .await
    }

    /// Make a GET request with query parameters
    pub(crate) async fn get_with_params<T, Q>(&self, path: &str, query: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let mut attempts = 0;
        loop {
            let mut url = self.config.build_url(path)?;
            let query_string = serde_urlencoded::to_string(query).unwrap_or_default();
            if !query_string.is_empty() {
                url.set_query(Some(&query_string));
            }
            let response = self.http_client.get(url).send().await;
            let result = match response {
                Ok(response) => self.handle_response(response).await,
                Err(err) => Err(Error::from(err)),
            };

            match result {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if !Self::should_retry(&err, Method::GET, attempts, self.config.max_retries)
                    {
                        return Err(err);
                    }
                    attempts += 1;
                    sleep(Self::retry_delay(attempts)).await;
                }
            }
        }
    }

    /// Make a raw HTTP request and return JSON or null for empty bodies
    pub async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value> {
        let mut attempts = 0;
        loop {
            let url = self.config.build_url(path)?;
            let mut request = self.http_client.request(method.clone(), url);
            if let Some(body) = body {
                request = request.json(body);
            }
            let response = request.send().await.map_err(Error::from)?;
            let status = response.status();
            let result = if status.is_success() {
                let body_text = response.text().await.map_err(Error::from)?;
                if body_text.trim().is_empty() {
                    Ok(Value::Null)
                } else {
                    Ok(serde_json::from_str(&body_text)?)
                }
            } else {
                let body_text = response.text().await.unwrap_or_default();
                Err(Error::from_response(status, body_text))
            };

            match result {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if !Self::should_retry(&err, method.clone(), attempts, self.config.max_retries)
                    {
                        return Err(err);
                    }
                    attempts += 1;
                    sleep(Self::retry_delay(attempts)).await;
                }
            }
        }
    }

    /// Make a POST request to the API
    pub(crate) async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request(Method::POST, path, Some(body)).await
    }

    /// Make a PUT request to the API
    pub(crate) async fn put<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request(Method::PUT, path, Some(body)).await
    }

    /// Make a PATCH request to the API
    pub(crate) async fn patch<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request(Method::PATCH, path, Some(body)).await
    }

    /// Make a DELETE request to the API
    pub(crate) async fn delete(&self, path: &str) -> Result<()> {
        let url = self.config.build_url(path)?;
        let response = self.http_client.delete(url).send().await?;

        if response.status().is_success() || response.status() == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(Error::from_response(status, body))
        }
    }

    /// Make a generic HTTP request
    async fn request<T, B>(&self, method: Method, path: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_with_retries(method, path, body).await
    }

    async fn request_with_retries<T, B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let mut attempts = 0;
        loop {
            let result = self.request_once(method.clone(), path, body).await;
            match result {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if !Self::should_retry(&err, method.clone(), attempts, self.config.max_retries)
                    {
                        return Err(err);
                    }
                    attempts += 1;
                    sleep(Self::retry_delay(attempts)).await;
                }
            }
        }
    }

    async fn request_once<T, B>(&self, method: Method, path: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.config.build_url(path)?;

        let mut request = self.http_client.request(method, url);

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await.map_err(Error::from)?;
        self.handle_response(response).await
    }

    fn should_retry(err: &Error, method: Method, attempts: u32, max_retries: u32) -> bool {
        if attempts >= max_retries {
            return false;
        }
        if method != Method::GET {
            return false;
        }
        match err {
            Error::Http(inner) => inner.is_timeout() || inner.is_connect(),
            Error::ApiError { status, .. } => *status == 429 || *status >= 500,
            _ => false,
        }
    }

    fn retry_delay(attempt: u32) -> Duration {
        let backoff_ms = 200u64.saturating_mul(attempt as u64);
        Duration::from_millis(backoff_ms)
    }

    /// Handle HTTP response and deserialize or return error
    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            // Successful response, deserialize JSON
            response.json().await.map_err(Error::from)
        } else {
            // Error response
            let body = response.text().await.unwrap_or_default();
            Err(Error::from_response(status, body))
        }
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.config.base_url)
            .field("timeout", &self.config.timeout)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[test]
    fn test_client_creation() {
        let config = ClientConfig::new("https://netbox.example.com", "test-token");
        let client = Client::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_invalid_config() {
        let config = ClientConfig::new("https://netbox.example.com", "");
        let client = Client::new(config);
        assert!(client.is_err());
    }

    #[test]
    fn test_client_debug() {
        let config = ClientConfig::new("https://netbox.example.com", "test-token");
        let client = Client::new(config).unwrap();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("netbox.example.com"));
    }

    #[test]
    fn test_openapi_config() {
        let config = ClientConfig::new("https://netbox.example.com", "test-token");
        let client = Client::new(config).unwrap();
        let openapi_config = client.openapi_config();
        assert_eq!(openapi_config.base_path, "https://netbox.example.com");
        let api_key = openapi_config.api_key.expect("api key should be set");
        assert_eq!(api_key.prefix.as_deref(), Some("Token"));
        assert_eq!(api_key.key, "test-token");
    }

    #[test]
    fn test_should_retry_respects_limits_and_method() {
        let err = Error::ApiError {
            status: 500,
            message: "server".to_string(),
            body: "".to_string(),
        };
        assert!(Client::should_retry(&err, Method::GET, 0, 3));
        assert!(!Client::should_retry(&err, Method::POST, 0, 3));
        assert!(!Client::should_retry(&err, Method::GET, 3, 3));
    }

    #[test]
    fn test_should_retry_status_codes() {
        let err_429 = Error::ApiError {
            status: 429,
            message: "rate".to_string(),
            body: "".to_string(),
        };
        let err_500 = Error::ApiError {
            status: 500,
            message: "server".to_string(),
            body: "".to_string(),
        };
        let err_404 = Error::ApiError {
            status: 404,
            message: "missing".to_string(),
            body: "".to_string(),
        };
        assert!(Client::should_retry(&err_429, Method::GET, 0, 1));
        assert!(Client::should_retry(&err_500, Method::GET, 0, 1));
        assert!(!Client::should_retry(&err_404, Method::GET, 0, 1));
    }

    #[test]
    fn test_retry_delay_backoff() {
        assert_eq!(Client::retry_delay(0), Duration::from_millis(0));
        assert_eq!(Client::retry_delay(1), Duration::from_millis(200));
        assert_eq!(Client::retry_delay(2), Duration::from_millis(400));
    }

    #[tokio::test]
    async fn request_raw_returns_json() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/api/status/");
            then.status(200).json_body(json!({ "ready": true }));
        });

        let config = ClientConfig::new(&server.base_url(), "test-token");
        let client = Client::new(config).unwrap();
        let value = client
            .request_raw(Method::GET, "status/", None)
            .await
            .unwrap();
        assert_eq!(value, json!({ "ready": true }));
        mock.assert();
    }

    #[tokio::test]
    async fn request_raw_returns_null_on_empty_body() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/api/test/");
            then.status(204);
        });

        let config = ClientConfig::new(&server.base_url(), "test-token");
        let client = Client::new(config).unwrap();
        let value = client
            .request_raw(Method::DELETE, "test/", None)
            .await
            .unwrap();
        assert_eq!(value, Value::Null);
        mock.assert();
    }
}
