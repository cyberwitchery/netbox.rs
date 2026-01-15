//! core endpoints for background tasks, data sources, and system metadata.
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

use crate::Client;
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
            .get(&format!("core/background-queues/{}/", name))
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
            .get(&format!("core/background-workers/{}/", name))
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
}
