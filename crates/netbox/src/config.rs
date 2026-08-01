//! client configuration

use crate::error::{Error, Result};
use crate::hooks::HttpHooks;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// configuration for the netbox client
#[derive(Clone)]
pub struct ClientConfig {
    /// original base url input
    pub(crate) raw_base_url: String,

    /// base url of the netbox instance (e.g., "<https://netbox.example.com>")
    pub(crate) base_url: Url,

    /// whether the provided base url parsed successfully
    pub(crate) base_url_valid: bool,

    /// api authentication token
    pub(crate) token: String,

    /// request timeout duration
    pub(crate) timeout: Duration,

    /// maximum number of retries for failed requests
    pub(crate) max_retries: u32,

    /// user agent string
    pub(crate) user_agent: String,

    /// whether to verify ssl certificates
    pub(crate) verify_ssl: bool,

    /// additional headers to send with every request
    pub(crate) extra_headers: HeaderMap,

    /// optional prebuilt reqwest client.
    pub(crate) http_client: Option<reqwest::Client>,

    /// optional callback to customize reqwest client builder.
    pub(crate) http_client_builder:
        Option<Arc<dyn Fn(reqwest::ClientBuilder) -> reqwest::ClientBuilder + Send + Sync>>,

    /// optional request/response hooks.
    pub(crate) http_hooks: Option<Arc<dyn HttpHooks>>,
}

impl ClientConfig {
    /// create a new client configuration
    ///
    /// # Arguments
    ///
    /// * `base_url` - the base url of the netbox instance (with or without trailing slash)
    /// * `token` - the api authentication token
    ///
    /// # Example
    ///
    /// ```
    /// use netbox::ClientConfig;
    ///
    /// let config = ClientConfig::new("https://netbox.example.com", "your-token-here");
    /// ```
    pub fn new(base_url: impl AsRef<str>, token: impl Into<String>) -> Self {
        let base_url_str = base_url.as_ref();

        let normalized = base_url_str.trim_end_matches('/');

        // parse URL, this will be validated when building the client
        let (base_url, base_url_valid) = match Url::parse(normalized)
            .or_else(|_| Url::parse(&format!("https://{}", normalized)))
        {
            Ok(url) => (url, true),
            Err(_) => (Url::parse("https://invalid.invalid").unwrap(), false),
        };

        Self {
            raw_base_url: base_url_str.to_string(),
            base_url,
            base_url_valid,
            token: token.into(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            user_agent: format!("netbox-rs/{} (Rust)", env!("CARGO_PKG_VERSION")),
            verify_ssl: true,
            extra_headers: HeaderMap::new(),
            http_client: None,
            http_client_builder: None,
            http_hooks: None,
        }
    }

    /// set the request timeout
    ///
    /// default: 30 seconds
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// set the maximum number of retries
    ///
    /// default: 3
    ///
    /// retries apply to get/delete requests (and operations marked idempotent) for transient network errors and 429/5xx responses.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// set a custom user agent string
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// disable ssl certificate verification (not recommended for production)
    ///
    /// default: enabled
    pub fn with_ssl_verification(mut self, verify: bool) -> Self {
        self.verify_ssl = verify;
        self
    }

    /// add a header to every request
    pub fn with_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.extra_headers.insert(name, value);
        self
    }

    /// add a set of headers to every request
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.extra_headers.extend(headers);
        self
    }

    /// inject a prebuilt reqwest client.
    ///
    /// when set, the client uses this reqwest instance directly.
    /// this takes precedence over `with_http_client_builder`.
    ///
    /// use this when you need full control over reqwest behavior
    /// (custom middleware stack, proxying, transport tuning, etc.).
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// customize the reqwest client builder before build.
    ///
    /// this is ignored if `with_http_client` is also used.
    ///
    /// use this when you want to adjust builder settings while retaining
    /// netbox default header/auth/timeout wiring.
    pub fn with_http_client_builder<F>(mut self, builder: F) -> Self
    where
        F: Fn(reqwest::ClientBuilder) -> reqwest::ClientBuilder + Send + Sync + 'static,
    {
        self.http_client_builder = Some(Arc::new(builder));
        self
    }

    /// attach hooks for request/response lifecycle customization.
    ///
    /// hooks can mutate outgoing requests and observe responses/errors.
    /// avoid putting secrets into logs/metrics from hook implementations.
    pub fn with_http_hooks<H>(mut self, hooks: H) -> Self
    where
        H: HttpHooks + 'static,
    {
        self.http_hooks = Some(Arc::new(hooks));
        self
    }

    /// access extra headers configured on this client
    pub fn extra_headers(&self) -> &HeaderMap {
        &self.extra_headers
    }

    /// validate the configuration
    pub(crate) fn validate(&self) -> Result<()> {
        if !self.base_url_valid {
            return Err(Error::Config(format!(
                "Invalid base URL: {}",
                self.raw_base_url
            )));
        }

        if self.base_url.scheme() != "http" && self.base_url.scheme() != "https" {
            return Err(Error::Config(format!(
                "Invalid URL scheme: {}. Must be http or https",
                self.base_url.scheme()
            )));
        }

        if self.token.is_empty() {
            return Err(Error::Config("API token cannot be empty".to_string()));
        }

        Ok(())
    }

    /// build the full api url by joining with a path
    ///
    /// this handles trailing slashes correctly. netbox returns absolute
    /// `next`/`previous` pagination cursors; when `path` is already an absolute
    /// url it is followed directly, but only if its origin matches the base url
    /// (so a rogue server can't redirect token-bearing requests to another host).
    pub(crate) fn build_url(&self, path: &str) -> Result<Url> {
        if is_absolute_http_url(path) {
            let url = Url::parse(path)?;
            if url.origin() != self.base_url.origin() {
                return Err(Error::Pagination(format!(
                    "pagination cursor points to unexpected origin {} (expected {}); refusing to follow it",
                    url.origin().ascii_serialization(),
                    self.base_url.origin().ascii_serialization(),
                )));
            }
            return Ok(url);
        }

        let path = path.trim_start_matches('/');
        let base_str = self.base_url.as_str().trim_end_matches('/');
        let url_str = format!("{}/api/{}", base_str, path);
        Url::parse(&url_str).map_err(Error::from)
    }
}

/// whether `path` is an absolute http(s) url (as returned in netbox pagination
/// cursors) rather than a relative api path. the scheme match is ascii-case
/// insensitive; slicing on byte prefixes avoids panicking on non-ascii input.
fn is_absolute_http_url(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes
        .get(..7)
        .is_some_and(|p| p.eq_ignore_ascii_case(b"http://"))
        || bytes
            .get(..8)
            .is_some_and(|p| p.eq_ignore_ascii_case(b"https://"))
}

impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("user_agent", &self.user_agent)
            .field("verify_ssl", &self.verify_ssl)
            .field("extra_headers", &self.extra_headers.len())
            .field("http_client", &self.http_client.is_some())
            .field("http_client_builder", &self.http_client_builder.is_some())
            .field("http_hooks", &self.http_hooks.is_some())
            .field("token", &"<redacted>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hooks::HttpHooks;
    use reqwest::Method;
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    struct TestHooks;

    impl HttpHooks for TestHooks {
        fn on_request(
            &self,
            _method: &Method,
            _path: &str,
            _request: &mut reqwest::Request,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_new_config() {
        let config = ClientConfig::new("https://netbox.example.com", "test-token");
        // URL parsing may add a trailing slash, so we check the trimmed version
        assert_eq!(
            config.base_url.as_str().trim_end_matches('/'),
            "https://netbox.example.com"
        );
        assert_eq!(config.token, "test-token");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert!(config.verify_ssl);
    }

    #[test]
    fn test_normalize_url_with_trailing_slash() {
        let config = ClientConfig::new("https://netbox.example.com/", "token");
        let config2 = ClientConfig::new("https://netbox.example.com", "token");
        assert_eq!(
            config.base_url.as_str().trim_end_matches('/'),
            config2.base_url.as_str().trim_end_matches('/')
        );
    }

    #[test]
    fn test_build_url() {
        let config = ClientConfig::new("https://netbox.example.com", "token");

        let url = config.build_url("/dcim/devices/").unwrap();
        assert_eq!(url.as_str(), "https://netbox.example.com/api/dcim/devices/");

        let url = config.build_url("dcim/devices/").unwrap();
        assert_eq!(url.as_str(), "https://netbox.example.com/api/dcim/devices/");
    }

    #[test]
    fn test_build_url_absolute_same_origin() {
        let config = ClientConfig::new("https://netbox.example.com", "token");

        // absolute pagination cursors on the same origin are followed as-is
        let url = config
            .build_url("https://netbox.example.com/api/dcim/devices/?limit=50&offset=50")
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://netbox.example.com/api/dcim/devices/?limit=50&offset=50"
        );

        // an explicit default port is the same origin (url normalizes it away)
        let url = config
            .build_url("https://netbox.example.com:443/api/dcim/devices/?offset=50")
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://netbox.example.com/api/dcim/devices/?offset=50"
        );
    }

    #[test]
    fn test_build_url_absolute_foreign_origin_rejected() {
        let config = ClientConfig::new("https://netbox.example.com", "token");

        let err = config
            .build_url("https://evil.example.com/api/dcim/devices/?offset=50")
            .unwrap_err();
        assert!(matches!(err, Error::Pagination(_)));
        let msg = err.to_string();
        assert!(msg.contains("evil.example.com"));
        assert!(msg.contains("netbox.example.com"));

        // same host, different scheme is a different origin too
        let err = config
            .build_url("http://netbox.example.com/api/dcim/devices/?offset=50")
            .unwrap_err();
        assert!(matches!(err, Error::Pagination(_)));

        // same host and scheme, different port is rejected
        let err = config
            .build_url("https://netbox.example.com:8443/api/dcim/devices/?offset=50")
            .unwrap_err();
        assert!(matches!(err, Error::Pagination(_)));
    }

    #[test]
    fn test_validation() {
        let config = ClientConfig::new("https://netbox.example.com", "token");
        assert!(config.validate().is_ok());

        let empty_token = ClientConfig::new("https://netbox.example.com", "");
        assert!(empty_token.validate().is_err());
    }

    #[test]
    fn test_builder_methods() {
        let config = ClientConfig::new("https://netbox.example.com", "token")
            .with_timeout(Duration::from_secs(60))
            .with_max_retries(5)
            .with_user_agent("custom-agent")
            .with_ssl_verification(false);

        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.user_agent, "custom-agent");
        assert!(!config.verify_ssl);
    }

    #[test]
    fn test_with_header() {
        let header_name = HeaderName::from_static("x-custom");
        let header_value = HeaderValue::from_static("value");
        let config = ClientConfig::new("https://netbox.example.com", "token")
            .with_header(header_name.clone(), header_value.clone());

        let stored = config.extra_headers.get(&header_name).unwrap();
        assert_eq!(stored, &header_value);
    }

    #[test]
    fn test_with_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-one"),
            HeaderValue::from_static("one"),
        );
        headers.insert(
            HeaderName::from_static("x-two"),
            HeaderValue::from_static("two"),
        );

        let config =
            ClientConfig::new("https://netbox.example.com", "token").with_headers(headers.clone());

        for (name, value) in headers.iter() {
            assert_eq!(config.extra_headers.get(name).unwrap(), value);
        }
    }

    #[test]
    fn test_with_http_client() {
        let prebuilt = reqwest::Client::new();
        let config =
            ClientConfig::new("https://netbox.example.com", "token").with_http_client(prebuilt);
        assert!(config.http_client.is_some());
    }

    #[test]
    fn test_with_http_client_builder() {
        let config = ClientConfig::new("https://netbox.example.com", "token")
            .with_http_client_builder(|builder| builder.pool_max_idle_per_host(2));
        assert!(config.http_client_builder.is_some());
    }

    #[test]
    fn test_with_http_hooks() {
        let config =
            ClientConfig::new("https://netbox.example.com", "token").with_http_hooks(TestHooks);
        assert!(config.http_hooks.is_some());
    }
}
