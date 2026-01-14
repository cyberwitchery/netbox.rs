//! plugin endpoints, including netbox-branching resources.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let branches = client.plugins().branches().list(None).await?;
//! println!("{}", branches.count);
//! # Ok(())
//! # }
//! ```

use crate::error::Result;
use crate::resource::Resource;
use crate::Client;

/// branch model.
pub type Branch = crate::models::Branch;
/// branch event model.
pub type BranchEvent = crate::models::BranchEvent;
/// change diff model.
pub type ChangeDiff = crate::models::ChangeDiff;
/// commit request model.
pub type CommitRequest = crate::models::CommitRequest;
/// job model.
pub type Job = crate::models::Job;
/// writable branch request model.
pub type WritableBranchRequest = crate::models::WritableBranchRequest;
/// patched writable branch request model.
pub type PatchedWritableBranchRequest = crate::models::PatchedWritableBranchRequest;

/// resource for branch events.
pub type BranchEventsApi = Resource<crate::models::BranchEvent>;
/// resource for branches.
pub type BranchesApi = Resource<crate::models::Branch>;
/// resource for changes.
pub type ChangesApi = Resource<crate::models::ChangeDiff>;

/// api for plugin endpoints.
#[derive(Clone)]
pub struct PluginsApi {
    client: Client,
}

impl PluginsApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the branch events resource.
    pub fn branch_events(&self) -> BranchEventsApi {
        Resource::new(self.client.clone(), "plugins/branching/branch-events/")
    }

    /// returns the branches resource.
    pub fn branches(&self) -> BranchesApi {
        Resource::new(self.client.clone(), "plugins/branching/branches/")
    }

    /// returns the changes resource.
    pub fn changes(&self) -> ChangesApi {
        Resource::new(self.client.clone(), "plugins/branching/changes/")
    }

    /// merge a branch (enqueue job).
    pub async fn merge_branch(&self, id: u64, body: &CommitRequest) -> Result<Job> {
        self.client
            .post(&format!("plugins/branching/branches/{}/merge/", id), body)
            .await
    }

    /// revert a branch (enqueue job).
    pub async fn revert_branch(&self, id: u64, body: &CommitRequest) -> Result<Job> {
        self.client
            .post(&format!("plugins/branching/branches/{}/revert/", id), body)
            .await
    }

    /// sync a branch (enqueue job).
    pub async fn sync_branch(&self, id: u64, body: &CommitRequest) -> Result<Job> {
        self.client
            .post(&format!("plugins/branching/branches/{}/sync/", id), body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::POST, MockServer};

    fn test_client(base_url: &str) -> Client {
        let config = ClientConfig::new(base_url, "token").with_max_retries(0);
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
    fn plugins_accessors_return_expected_paths() {
        let api = PluginsApi::new(test_client("https://netbox.example.com"));

        assert_path(api.branch_events(), "plugins/branching/branch-events/");
        assert_path(api.branches(), "plugins/branching/branches/");
        assert_path(api.changes(), "plugins/branching/changes/");
    }

    #[cfg_attr(miri, ignore)]

    #[tokio::test]
    async fn branch_actions_use_expected_paths() {
        let server = MockServer::start();
        let api = PluginsApi::new(test_client(&server.base_url()));

        let job_response = serde_json::json!({
            "id": 1,
            "url": "http://example.com/api/extras/jobs/1/",
            "display_url": "http://example.com/extras/jobs/1/",
            "display": "job",
            "object_type": "plugins.branch",
            "object_id": null,
            "name": "job",
            "status": {"value": "pending", "label": "Pending"},
            "created": "2024-01-01T00:00:00Z",
            "scheduled": null,
            "interval": null,
            "started": null,
            "completed": null,
            "user": {
                "id": 1,
                "url": "http://example.com/api/users/users/1/",
                "display": "admin",
                "username": "admin"
            },
            "data": null,
            "error": "",
            "job_id": "00000000-0000-0000-0000-000000000000",
            "log_entries": []
        });

        server.mock(|when, then| {
            when.method(POST)
                .path("/api/plugins/branching/branches/1/merge/");
            then.status(200).json_body(job_response.clone());
        });

        server.mock(|when, then| {
            when.method(POST)
                .path("/api/plugins/branching/branches/1/revert/");
            then.status(200).json_body(job_response.clone());
        });

        server.mock(|when, then| {
            when.method(POST)
                .path("/api/plugins/branching/branches/1/sync/");
            then.status(200).json_body(job_response.clone());
        });

        let commit = CommitRequest { commit: Some(true) };

        api.merge_branch(1u64, &commit).await.unwrap();
        api.revert_branch(1u64, &commit).await.unwrap();
        api.sync_branch(1u64, &commit).await.unwrap();
    }
}
