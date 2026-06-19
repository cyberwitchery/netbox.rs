use reqwest::Method;
use serde_json::Value;

use super::output::{print_dry_run, print_output};
use super::util::{load_json, normalize_api_path, request_raw_with_context};
use crate::config::{ConfigFile, config_path, validate_profile};
use crate::{
    ApiClient, AvailabilityAction, BranchAction, ConfigAction, DashboardAction, NamedLookupAction,
    OutputConfig, TaskAction, TraceableResource,
};

pub(crate) async fn handle_dashboard_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    action: DashboardAction,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        DashboardAction::Get => {
            let response =
                request_raw_with_context(client, Method::GET, "extras/dashboard/", None).await?;
            print_output(&response, output)?;
        }
        DashboardAction::Update { input } => {
            let body: Value = load_json(&input)?;
            if output.dry_run {
                print_dry_run(Method::PUT, "extras/dashboard/", None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::PUT, "extras/dashboard/", Some(&body))
                        .await?;
                print_output(&response, output)?;
            }
        }
        DashboardAction::Patch { input } => {
            let body: Value = load_json(&input)?;
            if output.dry_run {
                print_dry_run(Method::PATCH, "extras/dashboard/", None, Some(&body))?;
            } else {
                let response = request_raw_with_context(
                    client,
                    Method::PATCH,
                    "extras/dashboard/",
                    Some(&body),
                )
                .await?;
                print_output(&response, output)?;
            }
        }
        DashboardAction::Delete => {
            if output.dry_run {
                print_dry_run(Method::DELETE, "extras/dashboard/", None, None)?;
            } else {
                let response =
                    request_raw_with_context(client, Method::DELETE, "extras/dashboard/", None)
                        .await?;
                if response == Value::Null {
                    println!("deleted dashboard");
                } else {
                    print_output(&response, output)?;
                }
            }
        }
    }

    Ok(())
}

pub(crate) async fn handle_named_lookup(
    client: &impl ApiClient,
    output: &OutputConfig,
    base_path: &str,
    action: NamedLookupAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = normalize_api_path(base_path);
    match action {
        NamedLookupAction::List => {
            let response = request_raw_with_context(client, Method::GET, &base_path, None).await?;
            print_output(&response, output)?;
        }
        NamedLookupAction::Get { name } => {
            let path = format!("{}/{}/", base_path.trim_end_matches('/'), name);
            let response = request_raw_with_context(client, Method::GET, &path, None).await?;
            print_output(&response, output)?;
        }
    }

    Ok(())
}

pub(crate) async fn handle_branch_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    id: u64,
    action: BranchAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let (suffix, body) = match action {
        BranchAction::Merge { input } => ("merge", load_json(&input)?),
        BranchAction::Revert { input } => ("revert", load_json(&input)?),
        BranchAction::Sync { input } => ("sync", load_json(&input)?),
    };

    let path = format!("plugins/branching/branches/{}/{}/", id, suffix);
    if output.dry_run {
        print_dry_run(Method::POST, &path, None, Some(&body))?;
    } else {
        let response = request_raw_with_context(client, Method::POST, &path, Some(&body)).await?;
        print_output(&response, output)?;
    }
    Ok(())
}

pub(crate) async fn handle_availability_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    path: &str,
    action: AvailabilityAction,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        AvailabilityAction::List => {
            let response = request_raw_with_context(client, Method::GET, path, None).await?;
            print_output(&response, output)?;
        }
        AvailabilityAction::Create { input } => {
            let body: Value = load_json(&input)?;
            if output.dry_run {
                print_dry_run(Method::POST, path, None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::POST, path, Some(&body)).await?;
                print_output(&response, output)?;
            }
        }
    }
    Ok(())
}

pub(crate) async fn handle_task_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    id: &str,
    action: TaskAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let suffix = match action {
        TaskAction::Enqueue => "enqueue",
        TaskAction::Stop => "stop",
        TaskAction::Requeue => "requeue",
        TaskAction::Delete => "delete",
    };

    let path = format!("core/background-tasks/{}/{}/", id, suffix);
    if output.dry_run {
        print_dry_run(Method::POST, &path, None, None)?;
    } else {
        let response =
            request_raw_with_context(client, Method::POST, &path, Some(&Value::Null)).await?;
        print_output(&response, output)?;
    }
    Ok(())
}

pub(crate) async fn handle_sync_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if output.dry_run {
        print_dry_run(Method::POST, path, None, None)?;
    } else {
        let response =
            request_raw_with_context(client, Method::POST, path, Some(&Value::Null)).await?;
        print_output(&response, output)?;
    }
    Ok(())
}

pub(crate) async fn handle_get_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = request_raw_with_context(client, Method::GET, path, None).await?;
    print_output(&response, output)?;
    Ok(())
}

pub(crate) async fn handle_trace_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    resource: TraceableResource,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = match resource {
        TraceableResource::Interface { id } => format!("dcim/interfaces/{}/trace/", id),
        TraceableResource::ConsolePort { id } => format!("dcim/console-ports/{}/trace/", id),
        TraceableResource::ConsoleServerPort { id } => {
            format!("dcim/console-server-ports/{}/trace/", id)
        }
        TraceableResource::PowerPort { id } => format!("dcim/power-ports/{}/trace/", id),
        TraceableResource::PowerOutlet { id } => format!("dcim/power-outlets/{}/trace/", id),
        TraceableResource::PowerFeed { id } => format!("dcim/power-feeds/{}/trace/", id),
    };
    let response = request_raw_with_context(client, Method::GET, &path, None).await?;
    print_output(&response, output)?;
    Ok(())
}

pub(crate) fn handle_config_command(
    action: &ConfigAction,
    profile_name: &str,
    config_file: Option<&ConfigFile>,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Path => match config_path() {
            Some(path) => println!("{}", path.display()),
            None => println!("(could not determine config directory)"),
        },
        ConfigAction::List => match config_file {
            Some(cf) => {
                let mut names: Vec<_> = cf.profile_names();
                names.sort();
                for name in names {
                    if name == profile_name {
                        println!("{} (active)", name);
                    } else {
                        println!("{}", name);
                    }
                }
            }
            None => {
                println!("(no config file found)");
                if let Some(path) = config_path() {
                    println!("expected at: {}", path.display());
                }
            }
        },
        ConfigAction::Show => match config_file {
            Some(cf) => {
                if let Some(profile) = cf.get_profile(profile_name) {
                    let toml = toml::to_string_pretty(profile)?;
                    println!("[{}]", profile_name);
                    print!("{}", toml);
                } else {
                    return Err(format!("profile '{}' not found", profile_name).into());
                }
            }
            None => {
                return Err("no config file found".into());
            }
        },
        ConfigAction::Validate => {
            match config_file {
                Some(cf) => {
                    if let Some(profile) = cf.get_profile(profile_name) {
                        match validate_profile(profile) {
                            Ok(()) => {
                                println!("profile '{}' is valid", profile_name);
                                // try to resolve token to catch command errors
                                match profile.resolve_token() {
                                    Ok(Some(_)) => println!("  token: ok"),
                                    Ok(None) => println!(
                                        "  token: (not set, will need --token or NETBOX_TOKEN)"
                                    ),
                                    Err(e) => println!("  token: error - {}", e),
                                }
                            }
                            Err(e) => {
                                return Err(
                                    format!("profile '{}' invalid: {}", profile_name, e).into()
                                );
                            }
                        }
                    } else {
                        return Err(format!("profile '{}' not found", profile_name).into());
                    }
                }
                None => {
                    return Err("no config file found".into());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JsonInput;
    use crate::cli::test_util::*;
    use serde_json::{Value, json};

    #[tokio::test]
    async fn handle_dashboard_action_paths() {
        let client = FakeApiClient::new(Value::Null);
        handle_dashboard_action(&client, &output_config(), DashboardAction::Get)
            .await
            .unwrap();
        handle_dashboard_action(&client, &output_config(), DashboardAction::Delete)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].path, "extras/dashboard/");
        assert_eq!(calls[1].path, "extras/dashboard/");
    }

    #[tokio::test]
    async fn handle_named_lookup_get_builds_path() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let action = NamedLookupAction::Get {
            name: "queue-1".to_string(),
        };
        handle_named_lookup(&client, &output_config(), "core/background-queues/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].path, "core/background-queues/queue-1/");
    }

    #[tokio::test]
    async fn handle_branch_action_builds_path() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let input = JsonInput {
            json: Some(r#"{"confirm":true}"#.to_string()),
            file: None,
        };
        handle_branch_action(&client, &output_config(), 9, BranchAction::Merge { input })
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::POST);
        assert_eq!(calls[0].path, "plugins/branching/branches/9/merge/");
    }

    #[tokio::test]
    async fn handle_availability_action_list_calls_get() {
        let client = FakeApiClient::new(json!([]));
        handle_availability_action(
            &client,
            &output_config(),
            "ipam/prefixes/1/available-ips/",
            AvailabilityAction::List,
        )
        .await
        .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::GET);
        assert_eq!(calls[0].path, "ipam/prefixes/1/available-ips/");
    }

    #[tokio::test]
    async fn handle_availability_action_create_calls_post() {
        let client = FakeApiClient::new(json!([]));
        let input = JsonInput {
            json: Some(r#"[{"description":"test"}]"#.to_string()),
            file: None,
        };
        handle_availability_action(
            &client,
            &output_config(),
            "ipam/prefixes/1/available-ips/",
            AvailabilityAction::Create { input },
        )
        .await
        .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::POST);
        assert_eq!(calls[0].path, "ipam/prefixes/1/available-ips/");
    }

    #[tokio::test]
    async fn handle_task_action_builds_paths() {
        let client = FakeApiClient::new(json!({}));
        handle_task_action(&client, &output_config(), "abc123", TaskAction::Enqueue)
            .await
            .unwrap();
        handle_task_action(&client, &output_config(), "abc123", TaskAction::Stop)
            .await
            .unwrap();
        handle_task_action(&client, &output_config(), "abc123", TaskAction::Requeue)
            .await
            .unwrap();
        handle_task_action(&client, &output_config(), "abc123", TaskAction::Delete)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].path, "core/background-tasks/abc123/enqueue/");
        assert_eq!(calls[1].path, "core/background-tasks/abc123/stop/");
        assert_eq!(calls[2].path, "core/background-tasks/abc123/requeue/");
        assert_eq!(calls[3].path, "core/background-tasks/abc123/delete/");
        for call in &calls {
            assert_eq!(call.method, Method::POST);
        }
    }

    #[tokio::test]
    async fn handle_sync_action_calls_post() {
        let client = FakeApiClient::new(json!({}));
        handle_sync_action(&client, &output_config(), "core/data-sources/7/sync/")
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::POST);
        assert_eq!(calls[0].path, "core/data-sources/7/sync/");
    }

    #[tokio::test]
    async fn handle_get_action_calls_get() {
        let client = FakeApiClient::new(json!({}));
        handle_get_action(
            &client,
            &output_config(),
            "extras/custom-field-choice-sets/5/choices/",
        )
        .await
        .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::GET);
        assert_eq!(calls[0].path, "extras/custom-field-choice-sets/5/choices/");
    }

    #[tokio::test]
    async fn handle_trace_action_builds_paths() {
        let client = FakeApiClient::new(json!({}));
        handle_trace_action(
            &client,
            &output_config(),
            TraceableResource::Interface { id: 1 },
        )
        .await
        .unwrap();
        handle_trace_action(
            &client,
            &output_config(),
            TraceableResource::ConsolePort { id: 2 },
        )
        .await
        .unwrap();
        handle_trace_action(
            &client,
            &output_config(),
            TraceableResource::ConsoleServerPort { id: 3 },
        )
        .await
        .unwrap();
        handle_trace_action(
            &client,
            &output_config(),
            TraceableResource::PowerPort { id: 4 },
        )
        .await
        .unwrap();
        handle_trace_action(
            &client,
            &output_config(),
            TraceableResource::PowerOutlet { id: 5 },
        )
        .await
        .unwrap();
        handle_trace_action(
            &client,
            &output_config(),
            TraceableResource::PowerFeed { id: 6 },
        )
        .await
        .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].path, "dcim/interfaces/1/trace/");
        assert_eq!(calls[1].path, "dcim/console-ports/2/trace/");
        assert_eq!(calls[2].path, "dcim/console-server-ports/3/trace/");
        assert_eq!(calls[3].path, "dcim/power-ports/4/trace/");
        assert_eq!(calls[4].path, "dcim/power-outlets/5/trace/");
        assert_eq!(calls[5].path, "dcim/power-feeds/6/trace/");
        for call in &calls {
            assert_eq!(call.method, Method::GET);
        }
    }
}
