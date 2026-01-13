//! Plugin API endpoints.

use crate::error::Result;
use crate::resource::Resource;
use crate::Client;

/// Branch model.
pub type Branch = crate::models::Branch;
/// Branch event model.
pub type BranchEvent = crate::models::BranchEvent;
/// Change diff model.
pub type ChangeDiff = crate::models::ChangeDiff;
/// Commit request model.
pub type CommitRequest = crate::models::CommitRequest;
/// Job model.
pub type Job = crate::models::Job;
/// Writable branch request model.
pub type WritableBranchRequest = crate::models::WritableBranchRequest;
/// Patched writable branch request model.
pub type PatchedWritableBranchRequest = crate::models::PatchedWritableBranchRequest;

/// Resource for branch events.
pub type BranchEventsApi = Resource<crate::models::BranchEvent>;
/// Resource for branches.
pub type BranchesApi = Resource<crate::models::Branch>;
/// Resource for changes.
pub type ChangesApi = Resource<crate::models::ChangeDiff>;

/// API for plugin endpoints.
#[derive(Clone)]
pub struct PluginsApi {
    client: Client,
}

impl PluginsApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the branch events resource.
    pub fn branch_events(&self) -> BranchEventsApi {
        Resource::new(self.client.clone(), "plugins/branching/branch-events/")
    }

    /// Returns the branches resource.
    pub fn branches(&self) -> BranchesApi {
        Resource::new(self.client.clone(), "plugins/branching/branches/")
    }

    /// Returns the changes resource.
    pub fn changes(&self) -> ChangesApi {
        Resource::new(self.client.clone(), "plugins/branching/changes/")
    }

    /// Merge a branch (enqueue job).
    pub async fn merge_branch(&self, id: i32, body: &CommitRequest) -> Result<Job> {
        self.client
            .post(&format!("plugins/branching/branches/{}/merge/", id), body)
            .await
    }

    /// Revert a branch (enqueue job).
    pub async fn revert_branch(&self, id: i32, body: &CommitRequest) -> Result<Job> {
        self.client
            .post(&format!("plugins/branching/branches/{}/revert/", id), body)
            .await
    }

    /// Sync a branch (enqueue job).
    pub async fn sync_branch(&self, id: i32, body: &CommitRequest) -> Result<Job> {
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
        let paginator = resource.paginate(None);
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn plugins_accessors_return_expected_paths() {
        let api = PluginsApi::new(test_client("https://netbox.example.com"));

        assert_path(api.branch_events(), "plugins/branching/branch-events/");
        assert_path(api.branches(), "plugins/branching/branches/");
        assert_path(api.changes(), "plugins/branching/changes/");
    }

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
            when.method(POST).path("/api/plugins/branching/branches/1/merge/");
            then.status(200).json_body(job_response.clone());
        });

        server.mock(|when, then| {
            when.method(POST).path("/api/plugins/branching/branches/1/revert/");
            then.status(200).json_body(job_response.clone());
        });

        server.mock(|when, then| {
            when.method(POST).path("/api/plugins/branching/branches/1/sync/");
            then.status(200).json_body(job_response.clone());
        });

        let commit = CommitRequest { commit: Some(true) };

        api.merge_branch(1, &commit).await.unwrap();
        api.revert_branch(1, &commit).await.unwrap();
        api.sync_branch(1, &commit).await.unwrap();
    }
}
