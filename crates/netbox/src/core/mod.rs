//! Core API endpoints.

use crate::error::Result;
use crate::resource::Resource;
use crate::Client;
use serde_json::Value;
use std::collections::HashMap;

/// Background queue summary model.
pub type BackgroundQueue = HashMap<String, Value>;
/// Background task model.
pub type BackgroundTask = crate::models::BackgroundTask;
/// Background worker summary model.
pub type BackgroundWorker = HashMap<String, Value>;
/// Data file model.
pub type DataFile = crate::models::DataFile;
/// Data source model.
pub type DataSource = crate::models::DataSource;
/// Job model.
pub type Job = crate::models::Job;
/// Object change model.
pub type ObjectChange = crate::models::ObjectChange;
/// Object type model.
pub type ObjectType = crate::models::ObjectType;

/// Resource for background tasks.
pub type BackgroundTasksApi = Resource<crate::models::BackgroundTask>;
/// Resource for data files.
pub type DataFilesApi = Resource<crate::models::DataFile>;
/// Resource for data sources.
pub type DataSourcesApi = Resource<crate::models::DataSource>;
/// Resource for jobs.
pub type JobsApi = Resource<crate::models::Job>;
/// Resource for object changes.
pub type ObjectChangesApi = Resource<crate::models::ObjectChange>;
/// Resource for object types.
pub type ObjectTypesApi = Resource<crate::models::ObjectType>;

/// API for core endpoints.
#[derive(Clone)]
pub struct CoreApi {
    client: Client,
}

impl CoreApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Fetch the background queues summary.
    pub async fn background_queues(&self) -> Result<BackgroundQueue> {
        self.client.get("core/background-queues/").await
    }

    /// Fetch a background queue summary by name.
    pub async fn background_queue(&self, name: &str) -> Result<BackgroundQueue> {
        self.client
            .get(&format!("core/background-queues/{}/", name))
            .await
    }

    /// Returns the background tasks resource.
    pub fn background_tasks(&self) -> BackgroundTasksApi {
        Resource::new(self.client.clone(), "core/background-tasks/")
    }

    /// Fetch the background workers summary.
    pub async fn background_workers(&self) -> Result<BackgroundWorker> {
        self.client.get("core/background-workers/").await
    }

    /// Fetch a background worker summary by name.
    pub async fn background_worker(&self, name: &str) -> Result<BackgroundWorker> {
        self.client
            .get(&format!("core/background-workers/{}/", name))
            .await
    }

    /// Returns the data files resource.
    pub fn data_files(&self) -> DataFilesApi {
        Resource::new(self.client.clone(), "core/data-files/")
    }

    /// Returns the data sources resource.
    pub fn data_sources(&self) -> DataSourcesApi {
        Resource::new(self.client.clone(), "core/data-sources/")
    }

    /// Returns the jobs resource.
    pub fn jobs(&self) -> JobsApi {
        Resource::new(self.client.clone(), "core/jobs/")
    }

    /// Returns the object changes resource.
    pub fn object_changes(&self) -> ObjectChangesApi {
        Resource::new(self.client.clone(), "core/object-changes/")
    }

    /// Returns the object types resource.
    pub fn object_types(&self) -> ObjectTypesApi {
        Resource::new(self.client.clone(), "core/object-types/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://netbox.example.com", "token");
        Client::new(config).unwrap()
    }

    fn assert_path<T>(resource: Resource<T>, expected: &str)
    where
        T: serde::de::DeserializeOwned,
    {
        let paginator = resource.paginate(None);
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
}
