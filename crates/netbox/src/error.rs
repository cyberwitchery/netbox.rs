//! error types for the netbox client

/// result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// main error type for netbox operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// http request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// invalid url
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// api returned an error response
    #[error("NetBox API error (status {status}): {message}")]
    ApiError {
        /// http status code
        status: u16,
        /// error message from api
        message: String,
        /// full response body for debugging
        body: String,
    },

    /// json serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// authentication failed
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// pagination error
    #[error("Pagination error: {0}")]
    Pagination(String),
}

impl Error {
    /// create a new api error from response
    pub fn from_response(status: reqwest::StatusCode, body: String) -> Self {
        // Try to extract error message from JSON response
        let message = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            // NetBox often returns errors in different formats:
            // {"detail": "error message"}
            // {"field_name": ["error1", "error2"]}
            // {"non_field_errors": ["error"]}
            if let Some(detail) = json.get("detail").and_then(|v| v.as_str()) {
                detail.to_string()
            } else if let Some(obj) = json.as_object() {
                // Collect all field errors
                let errors: Vec<String> = obj
                    .iter()
                    .map(|(key, value)| {
                        let err_msgs = match value {
                            serde_json::Value::String(s) => vec![s.clone()],
                            serde_json::Value::Array(arr) => arr
                                .iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect(),
                            _ => vec![value.to_string()],
                        };
                        format!("{}: {}", key, err_msgs.join(", "))
                    })
                    .collect();
                errors.join("; ")
            } else {
                body.chars().take(200).collect()
            }
        } else {
            // Not JSON, truncate plain text
            body.chars().take(200).collect()
        };

        Error::ApiError {
            status: status.as_u16(),
            message,
            body,
        }
    }

    /// check if error is a 404 Not Found
    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            Error::ApiError { status: 404, .. } | Error::NotFound(_)
        )
    }

    /// check if error is authentication related
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            Error::ApiError {
                status: 401 | 403,
                ..
            } | Error::Auth(_)
        )
    }

    /// get http status code if this is an api error
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::ApiError { status, .. } => Some(*status),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_json_detail() {
        let body = r#"{"detail": "Not found."}"#;
        let err = Error::from_response(reqwest::StatusCode::NOT_FOUND, body.to_string());

        assert!(err.is_not_found());
        assert_eq!(err.status_code(), Some(404));
        assert!(err.to_string().contains("Not found"));
    }

    #[test]
    fn test_error_from_field_errors() {
        let body = r#"{"name": ["This field is required."], "value": ["Invalid value."]}"#;
        let err = Error::from_response(reqwest::StatusCode::BAD_REQUEST, body.to_string());

        let msg = err.to_string();
        assert!(msg.contains("name"));
        assert!(msg.contains("required"));
        assert!(msg.contains("value"));
    }

    #[test]
    fn test_auth_error_detection() {
        let err = Error::from_response(
            reqwest::StatusCode::UNAUTHORIZED,
            r#"{"detail": "Invalid token"}"#.to_string(),
        );
        assert!(err.is_auth_error());
    }
}
