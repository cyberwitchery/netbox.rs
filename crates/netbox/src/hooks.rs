//! request/response hook extension points for advanced client customization.

use crate::error::Result;
use reqwest::{Method, Request, StatusCode};
use std::time::Duration;

/// hooks for observing and modifying request/response lifecycle events.
///
/// this can be used for cross-cutting behavior such as custom auth headers,
/// metrics emission, or request/response logging.
pub trait HttpHooks: Send + Sync {
    /// called before a request is sent. hooks may mutate the request.
    fn on_request(&self, _method: &Method, _path: &str, _request: &mut Request) -> Result<()> {
        Ok(())
    }

    /// called after a response is received (including non-2xx).
    fn on_response(&self, _method: &Method, _path: &str, _status: StatusCode, _duration: Duration) {
    }

    /// called when request dispatch fails before a response is received.
    fn on_error(&self, _method: &Method, _path: &str, _error: &crate::Error, _duration: Duration) {}
}
