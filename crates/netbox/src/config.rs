//! client configuration

use crate::error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
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
}

impl ClientConfig {
    /// create a new client configuration
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base url of the netbox instance (with or without trailing slash)
    /// * `token` - The api authentication token
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

        // Normalize base URL: ensure it doesn't end with a slash
        let normalized = base_url_str.trim_end_matches('/');

        // Parse URL, this will be validated when building the client
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
    /// retries apply to get requests for transient network errors and 429/5xx responses.
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

        // Validate base URL
        if self.base_url.scheme() != "http" && self.base_url.scheme() != "https" {
            return Err(Error::Config(format!(
                "Invalid URL scheme: {}. Must be http or https",
                self.base_url.scheme()
            )));
        }

        // Validate token
        if self.token.is_empty() {
            return Err(Error::Config("API token cannot be empty".to_string()));
        }

        Ok(())
    }

    /// build the full api url by joining with a path
    ///
    /// this handles trailing slashes correctly.
    pub(crate) fn build_url(&self, path: &str) -> Result<Url> {
        let path = path.trim_start_matches('/');
        let base_str = self.base_url.as_str().trim_end_matches('/');
        let url_str = format!("{}/api/{}", base_str, path);
        Url::parse(&url_str).map_err(Error::from)
    }
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
            .field("token", &"<redacted>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

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
        // Both should normalize to the same thing
        let config2 = ClientConfig::new("https://netbox.example.com", "token");
        assert_eq!(
            config.base_url.as_str().trim_end_matches('/'),
            config2.base_url.as_str().trim_end_matches('/')
        );
    }

    #[test]
    fn test_build_url() {
        let config = ClientConfig::new("https://netbox.example.com", "token");

        // Test with leading slash
        let url = config.build_url("/dcim/devices/").unwrap();
        assert_eq!(url.as_str(), "https://netbox.example.com/api/dcim/devices/");

        // Test without leading slash
        let url = config.build_url("dcim/devices/").unwrap();
        assert_eq!(url.as_str(), "https://netbox.example.com/api/dcim/devices/");
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
}
