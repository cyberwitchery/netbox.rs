//! generic api resource wrapper for standard netbox crud endpoints.

use crate::error::Result;
use crate::pagination::{Page, Paginator};
use crate::query::QueryBuilder;
use crate::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// generic resource wrapper for list/get/create/update/patch/delete operations.
#[derive(Clone)]
pub struct Resource<T> {
    client: Client,
    path: &'static str,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Resource<T>
where
    T: DeserializeOwned,
{
    pub(crate) fn new(client: Client, path: &'static str) -> Self {
        Self {
            client,
            path,
            _marker: std::marker::PhantomData,
        }
    }

    /// list all resources for this endpoint.
    pub async fn list(&self, query: Option<QueryBuilder>) -> Result<Page<T>> {
        let query = query.unwrap_or_default();
        self.client.get_with_params(self.path, &query).await
    }

    /// get a paginator for iterating through all resources.
    pub fn paginate(&self, query: Option<QueryBuilder>) -> Paginator<T> {
        let path = if let Some(q) = query {
            let query_string = serde_urlencoded::to_string(&q).unwrap_or_default();
            if query_string.is_empty() {
                self.path.to_string()
            } else {
                format!("{}?{}", self.path.trim_end_matches('/'), query_string)
            }
        } else {
            self.path.to_string()
        };
        Paginator::new(self.client.clone(), path)
    }

    /// get a resource by id.
    pub async fn get(&self, id: i32) -> Result<T> {
        self.client.get(&format!("{}{}/", self.path, id)).await
    }

    /// create a resource.
    pub async fn create<B>(&self, body: &B) -> Result<T>
    where
        B: Serialize,
    {
        self.client.post(self.path, body).await
    }

    /// update a resource (full update).
    pub async fn update<B>(&self, id: i32, body: &B) -> Result<T>
    where
        B: Serialize,
    {
        self.client
            .put(&format!("{}{}/", self.path, id), body)
            .await
    }

    /// partially update a resource.
    pub async fn patch<B>(&self, id: i32, body: &B) -> Result<T>
    where
        B: Serialize,
    {
        self.client
            .patch(&format!("{}{}/", self.path, id), body)
            .await
    }

    /// delete a resource.
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.client.delete(&format!("{}{}/", self.path, id)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::Method::GET;
    use httpmock::{Method::DELETE, Method::PATCH, Method::POST, Method::PUT, MockServer};

    fn test_client() -> Client {
        let config = ClientConfig::new("https://netbox.example.com", "token");
        Client::new(config).unwrap()
    }

    #[test]
    fn paginate_without_query_uses_base_path() {
        let resource: Resource<serde_json::Value> = Resource::new(test_client(), "dcim/devices/");
        let paginator = resource.paginate(None);
        assert_eq!(paginator.next_url(), Some("dcim/devices/"));
    }

    #[test]
    fn paginate_with_query_encodes_path() {
        let resource: Resource<serde_json::Value> = Resource::new(test_client(), "dcim/devices/");
        let query = QueryBuilder::new().filter("status", "active").limit(10);
        let paginator = resource.paginate(Some(query));
        let query = QueryBuilder::new().filter("status", "active").limit(10);
        let expected_query = serde_urlencoded::to_string(&query).unwrap_or_default();
        let expected = if expected_query.is_empty() {
            "dcim/devices/".to_string()
        } else {
            format!("dcim/devices?{}", expected_query)
        };
        assert_eq!(paginator.next_url(), Some(expected.as_str()));
    }

    #[tokio::test]
    async fn resource_crud_calls_expected_paths() {
        let server = MockServer::start();
        let base_url = server.base_url();
        let config = ClientConfig::new(&base_url, "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let resource: Resource<serde_json::Value> = Resource::new(client, "dcim/devices/");

        let list_response = serde_json::json!({
            "count": 1,
            "next": null,
            "previous": null,
            "results": [{"id": 1}]
        });

        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/devices/");
            then.status(200).json_body(list_response.clone());
        });

        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/devices/1/");
            then.status(200).json_body(serde_json::json!({"id": 1}));
        });

        server.mock(|when, then| {
            when.method(POST).path("/api/dcim/devices/");
            then.status(201).json_body(serde_json::json!({"id": 2}));
        });

        server.mock(|when, then| {
            when.method(PUT).path("/api/dcim/devices/1/");
            then.status(200)
                .json_body(serde_json::json!({"id": 1, "updated": true}));
        });

        server.mock(|when, then| {
            when.method(PATCH).path("/api/dcim/devices/1/");
            then.status(200)
                .json_body(serde_json::json!({"id": 1, "patched": true}));
        });

        server.mock(|when, then| {
            when.method(DELETE).path("/api/dcim/devices/1/");
            then.status(204);
        });

        let page = resource.list(None).await.unwrap();
        assert_eq!(page.count, 1);
        assert_eq!(page.results[0]["id"], 1);

        let item = resource.get(1).await.unwrap();
        assert_eq!(item["id"], 1);

        let created = resource
            .create(&serde_json::json!({"name": "device"}))
            .await
            .unwrap();
        assert_eq!(created["id"], 2);

        let updated = resource
            .update(1, &serde_json::json!({"name": "device"}))
            .await
            .unwrap();
        assert_eq!(updated["updated"], true);

        let patched = resource
            .patch(1, &serde_json::json!({"name": "device"}))
            .await
            .unwrap();
        assert_eq!(patched["patched"], true);

        resource.delete(1).await.unwrap();
    }

    #[tokio::test]
    async fn list_with_query_encodes_parameters() {
        let server = MockServer::start();
        let base_url = server.base_url();
        let config = ClientConfig::new(&base_url, "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let resource: Resource<serde_json::Value> = Resource::new(client, "dcim/devices/");

        let list_response = serde_json::json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });

        server.mock(|when, then| {
            let query = QueryBuilder::new().limit(5);
            let query_string = serde_urlencoded::to_string(&query).unwrap_or_default();
            let pairs = url::form_urlencoded::parse(query_string.as_bytes())
                .into_owned()
                .collect::<Vec<_>>();

            let mut when = when.method(GET).path("/api/dcim/devices/");
            for (key, value) in pairs {
                when = when.query_param(key, value);
            }
            then.status(200).json_body(list_response);
        });

        let query = QueryBuilder::new().limit(5);
        let page = resource.list(Some(query)).await.unwrap();
        assert_eq!(page.count, 0);
    }
}
