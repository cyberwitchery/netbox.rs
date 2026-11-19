//! core endpoints for background tasks, data sources, and system metadata.
//!
//! includes task management (enqueue, stop, requeue, delete) and data source sync.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let changes = client.core().object_changes().list(None).await?;
//! println!("{}", changes.count);
//! # Ok(())
//! # }
//! ```
//!
//! task management:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! // enqueue a background task
//! client.core().enqueue_task("task-id").await?;
//!
//! // sync a data source
//! client.core().sync_data_source(7).await?;
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::config::path_segment;
use crate::error::Result;
use crate::resource::Resource;
use serde_json::Value;
use std::collections::HashMap;

/// background queue summary model.
pub type BackgroundQueue = HashMap<String, Value>;
/// background task model.
pub type BackgroundTask = crate::models::BackgroundTask;
/// background worker summary model.
pub type BackgroundWorker = HashMap<String, Value>;
/// data file model.
pub type DataFile = crate::models::DataFile;
/// data source model.
pub type DataSource = crate::models::DataSource;
/// job model.
pub type Job = crate::models::Job;
/// object change model.
pub type ObjectChange = crate::models::ObjectChange;
/// object type model.
pub type ObjectType = crate::models::ObjectType;

/// resource for background tasks.
pub type BackgroundTasksApi = Resource<crate::models::BackgroundTask>;
/// resource for data files.
pub type DataFilesApi = Resource<crate::models::DataFile>;
/// resource for data sources.
pub type DataSourcesApi = Resource<crate::models::DataSource>;
/// resource for jobs.
pub type JobsApi = Resource<crate::models::Job>;
/// resource for object changes.
pub type ObjectChangesApi = Resource<crate::models::ObjectChange>;
/// resource for object types.
pub type ObjectTypesApi = Resource<crate::models::ObjectType>;

/// api for core endpoints.
#[derive(Clone)]
pub struct CoreApi {
    client: Client,
}

impl CoreApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// fetch the background queues summary.
    pub async fn background_queues(&self) -> Result<BackgroundQueue> {
        self.client.get("core/background-queues/").await
    }

    /// fetch a background queue summary by name.
    pub async fn background_queue(&self, name: &str) -> Result<BackgroundQueue> {
        self.client
            .get(&format!("core/background-queues/{}/", path_segment(name)?))
            .await
    }

    /// returns the background tasks resource.
    pub fn background_tasks(&self) -> BackgroundTasksApi {
        Resource::new(self.client.clone(), "core/background-tasks/")
    }

    /// fetch the background workers summary.
    pub async fn background_workers(&self) -> Result<BackgroundWorker> {
        self.client.get("core/background-workers/").await
    }

    /// fetch a background worker summary by name.
    pub async fn background_worker(&self, name: &str) -> Result<BackgroundWorker> {
        self.client
            .get(&format!("core/background-workers/{}/", path_segment(name)?))
            .await
    }

    /// returns the data files resource.
    pub fn data_files(&self) -> DataFilesApi {
        Resource::new(self.client.clone(), "core/data-files/")
    }

    /// returns the data sources resource.
    pub fn data_sources(&self) -> DataSourcesApi {
        Resource::new(self.client.clone(), "core/data-sources/")
    }

    /// returns the jobs resource.
    pub fn jobs(&self) -> JobsApi {
        Resource::new(self.client.clone(), "core/jobs/")
    }

    /// returns the object changes resource.
    pub fn object_changes(&self) -> ObjectChangesApi {
        Resource::new(self.client.clone(), "core/object-changes/")
    }

    /// returns the object types resource.
    pub fn object_types(&self) -> ObjectTypesApi {
        Resource::new(self.client.clone(), "core/object-types/")
    }

    /// enqueue a background task.
    pub async fn enqueue_task(&self, id: &str) -> Result<BackgroundTask> {
        self.client
            .post(
                &format!("core/background-tasks/{}/enqueue/", path_segment(id)?),
                &(),
            )
            .await
    }

    /// stop a background task.
    pub async fn stop_task(&self, id: &str) -> Result<BackgroundTask> {
        self.client
            .post(
                &format!("core/background-tasks/{}/stop/", path_segment(id)?),
                &(),
            )
            .await
    }

    /// requeue a background task.
    pub async fn requeue_task(&self, id: &str) -> Result<BackgroundTask> {
        self.client
            .post(
                &format!("core/background-tasks/{}/requeue/", path_segment(id)?),
                &(),
            )
            .await
    }

    /// delete a background task.
    pub async fn delete_task(&self, id: &str) -> Result<BackgroundTask> {
        self.client
            .post(
                &format!("core/background-tasks/{}/delete/", path_segment(id)?),
                &(),
            )
            .await
    }

    /// sync a data source.
    pub async fn sync_data_source(&self, id: u64) -> Result<DataSource> {
        self.client
            .post(&format!("core/data-sources/{}/sync/", id), &())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::prelude::*;
    use serde_json::json;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://netbox.example.com", "token");
        Client::new(config).unwrap()
    }

    fn mock_client(server: &MockServer) -> Client {
        let config = ClientConfig::new(server.base_url(), "test-token");
        Client::new(config).unwrap()
    }

    fn assert_path<T>(resource: Resource<T>, expected: &str)
    where
        T: serde::de::DeserializeOwned,
    {
        let paginator = resource.paginate(None).unwrap();
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn core_accessors_return_expected_paths() {
        let api = CoreApi::new(test_client());

        assert_path(api.background_tasks(), "core/background-tasks/");
        assert_path(api.data_files(), "core/data-files/");
        assert_path(api.data_sources(), "core/data-sources/");
        assert_path(api.jobs(), "core/jobs/");
        assert_path(api.object_changes(), "core/object-changes/");
        assert_path(api.object_types(), "core/object-types/");
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn names_and_ids_cannot_leave_their_path() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "test-token").with_max_retries(0);
        let api = CoreApi::new(Client::new(config).unwrap());

        let users = server.mock(|when, then| {
            when.path_includes("/users/");
            then.status(200).json_body(json!({}));
        });
        let tasks = server.mock(|when, then| {
            when.path("/api/core/background-tasks/");
            then.status(200).json_body(json!({}));
        });

        let _ = api.background_queue("../../users/tokens/7").await;
        let _ = api.background_worker("x/../../../users/users/?a=b#").await;
        let _ = api.enqueue_task("../../users/tokens/7/#").await;
        let _ = api.stop_task("%2e%2e/%2e%2e/users/tokens/7").await;
        let _ = api.requeue_task("../../users/tokens/7").await;
        let _ = api.delete_task("../../users/tokens/7").await;
        assert_eq!(users.calls_async().await, 0, "a name reached /api/users/");

        assert!(api.delete_task("..").await.is_err());
        assert!(api.delete_task(".").await.is_err());
        assert!(api.delete_task("").await.is_err());
        assert_eq!(tasks.calls_async().await, 0, "an id reached the collection");
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn task_actions_use_expected_paths() {
        let server = MockServer::start();
        let client = mock_client(&server);

        let enqueue_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/core/background-tasks/abc123/enqueue/");
            then.status(200).json_body(json!({}));
        });
        let _ = client.core().enqueue_task("abc123").await;
        enqueue_mock.assert();

        let stop_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/core/background-tasks/task-id/stop/");
            then.status(200).json_body(json!({}));
        });
        let _ = client.core().stop_task("task-id").await;
        stop_mock.assert();

        let requeue_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/core/background-tasks/xyz/requeue/");
            then.status(200).json_body(json!({}));
        });
        let _ = client.core().requeue_task("xyz").await;
        requeue_mock.assert();

        let delete_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/core/background-tasks/del-me/delete/");
            then.status(200).json_body(json!({}));
        });
        let _ = client.core().delete_task("del-me").await;
        delete_mock.assert();
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn sync_data_source_uses_expected_path() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/core/data-sources/7/sync/");
            then.status(200).json_body(json!({}));
        });

        let client = mock_client(&server);
        let _ = client.core().sync_data_source(7).await;
        mock.assert();
    }
}
