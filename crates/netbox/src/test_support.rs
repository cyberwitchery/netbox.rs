//! helpers shared by the unit tests.

use httpmock::MockServer;
use serde_json::Value;

use crate::{Client, ClientConfig, Resource};

/// a client for a host that is never contacted.
pub(crate) fn test_client() -> Client {
    let config = ClientConfig::new("https://netbox.example.com", "token");
    Client::new(config).unwrap()
}

/// a client for `server`.
pub(crate) fn mock_client(server: &MockServer) -> Client {
    let config = ClientConfig::new(server.base_url(), "test-token");
    Client::new(config).unwrap()
}

/// asserts that `key` is absent from the serialized `value`.
pub(crate) fn assert_missing(value: &Value, key: &str) {
    assert!(value.get(key).is_none(), "expected {} to be omitted", key);
}

/// asserts that `resource` lists from `expected`.
pub(crate) fn assert_path<T>(resource: Resource<T>, expected: &str)
where
    T: serde::de::DeserializeOwned,
{
    let paginator = resource.paginate(None).unwrap();
    assert_eq!(paginator.next_url(), Some(expected));
}
