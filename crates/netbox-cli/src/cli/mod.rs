mod handlers;
mod output;
mod resources;
mod util;

pub(crate) use handlers::{
    handle_availability_action, handle_branch_action, handle_config_command,
    handle_dashboard_action, handle_get_action, handle_named_lookup, handle_sync_action,
    handle_task_action, handle_trace_action,
};
pub(crate) use output::{print_dry_run, print_output};
pub(crate) use resources::{
    CIRCUITS_RESOURCES, CORE_RESOURCES, DCIM_RESOURCES, EXTRAS_RESOURCES, IPAM_RESOURCES,
    PLUGINS_RESOURCES, TENANCY_RESOURCES, USERS_RESOURCES, VIRTUALIZATION_RESOURCES, VPN_RESOURCES,
    WIRELESS_RESOURCES, handle_resource_action, handle_resource_group, print_resources,
};
pub(crate) use util::{
    append_query, build_schema_path, load_graphql_query, load_graphql_vars, load_json,
    load_json_optional, normalize_api_path, request_raw_with_context, wrap_request_error,
};

#[cfg(test)]
pub(crate) mod test_util {
    use reqwest::Method;
    use serde_json::Value;
    use std::sync::{Arc, Mutex};

    use crate::{ApiClient, OutputConfig, OutputFormat};

    #[derive(Clone, Debug, PartialEq)]
    pub(crate) struct RecordedCall {
        pub(crate) method: Method,
        pub(crate) path: String,
        pub(crate) body: Option<Value>,
    }

    pub(crate) struct FakeApiClient {
        calls: Arc<Mutex<Vec<RecordedCall>>>,
        next: Arc<Mutex<Value>>,
    }

    impl FakeApiClient {
        pub(crate) fn new(response: Value) -> Self {
            Self {
                calls: Arc::new(Mutex::new(Vec::new())),
                next: Arc::new(Mutex::new(response)),
            }
        }

        pub(crate) fn calls(&self) -> Vec<RecordedCall> {
            self.calls.lock().unwrap().clone()
        }
    }

    pub(crate) struct ErrorApiClient;

    #[async_trait::async_trait]
    impl ApiClient for ErrorApiClient {
        async fn request_raw(
            &self,
            _method: Method,
            _path: &str,
            _body: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }

        async fn graphql(
            &self,
            _query: &str,
            _variables: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }

        async fn status(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }

        async fn schema(
            &self,
            _format: Option<&str>,
            _lang: Option<&str>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }
    }

    #[async_trait::async_trait]
    impl ApiClient for FakeApiClient {
        async fn request_raw(
            &self,
            method: Method,
            path: &str,
            body: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            let body = body.cloned();
            self.calls.lock().unwrap().push(RecordedCall {
                method,
                path: path.to_string(),
                body,
            });
            Ok(self.next.lock().unwrap().clone())
        }

        async fn graphql(
            &self,
            _query: &str,
            _variables: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.next.lock().unwrap().clone())
        }

        async fn status(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.next.lock().unwrap().clone())
        }

        async fn schema(
            &self,
            _format: Option<&str>,
            _lang: Option<&str>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.next.lock().unwrap().clone())
        }
    }

    pub(crate) fn output_config() -> OutputConfig {
        OutputConfig {
            format: OutputFormat::Json,
            select: None,
            columns: None,
            max_columns: 6,
            dry_run: false,
        }
    }
}
