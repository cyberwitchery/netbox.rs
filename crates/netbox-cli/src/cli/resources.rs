use reqwest::Method;
use serde_json::Value;

use super::output::{print_dry_run, print_output};
use super::util::{append_query, load_json, normalize_api_path, request_raw_with_context};
use crate::{ApiClient, OutputConfig, ResourceAction};

#[derive(Clone, Copy)]
pub(crate) struct ResourceEntry {
    pub(crate) name: &'static str,
    pub(crate) path: &'static str,
}

pub(crate) const DCIM_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cable-bundles",
        path: "dcim/cable-bundles/",
    },
    ResourceEntry {
        name: "cable-terminations",
        path: "dcim/cable-terminations/",
    },
    ResourceEntry {
        name: "cables",
        path: "dcim/cables/",
    },
    ResourceEntry {
        name: "console-port-templates",
        path: "dcim/console-port-templates/",
    },
    ResourceEntry {
        name: "console-ports",
        path: "dcim/console-ports/",
    },
    ResourceEntry {
        name: "console-server-port-templates",
        path: "dcim/console-server-port-templates/",
    },
    ResourceEntry {
        name: "console-server-ports",
        path: "dcim/console-server-ports/",
    },
    ResourceEntry {
        name: "device-bay-templates",
        path: "dcim/device-bay-templates/",
    },
    ResourceEntry {
        name: "device-bays",
        path: "dcim/device-bays/",
    },
    ResourceEntry {
        name: "device-roles",
        path: "dcim/device-roles/",
    },
    ResourceEntry {
        name: "device-types",
        path: "dcim/device-types/",
    },
    ResourceEntry {
        name: "devices",
        path: "dcim/devices/",
    },
    ResourceEntry {
        name: "front-port-templates",
        path: "dcim/front-port-templates/",
    },
    ResourceEntry {
        name: "front-ports",
        path: "dcim/front-ports/",
    },
    ResourceEntry {
        name: "interface-templates",
        path: "dcim/interface-templates/",
    },
    ResourceEntry {
        name: "interfaces",
        path: "dcim/interfaces/",
    },
    ResourceEntry {
        name: "inventory-item-roles",
        path: "dcim/inventory-item-roles/",
    },
    ResourceEntry {
        name: "inventory-item-templates",
        path: "dcim/inventory-item-templates/",
    },
    ResourceEntry {
        name: "inventory-items",
        path: "dcim/inventory-items/",
    },
    ResourceEntry {
        name: "locations",
        path: "dcim/locations/",
    },
    ResourceEntry {
        name: "mac-addresses",
        path: "dcim/mac-addresses/",
    },
    ResourceEntry {
        name: "manufacturers",
        path: "dcim/manufacturers/",
    },
    ResourceEntry {
        name: "module-bay-templates",
        path: "dcim/module-bay-templates/",
    },
    ResourceEntry {
        name: "module-bays",
        path: "dcim/module-bays/",
    },
    ResourceEntry {
        name: "module-type-profiles",
        path: "dcim/module-type-profiles/",
    },
    ResourceEntry {
        name: "module-types",
        path: "dcim/module-types/",
    },
    ResourceEntry {
        name: "modules",
        path: "dcim/modules/",
    },
    ResourceEntry {
        name: "platforms",
        path: "dcim/platforms/",
    },
    ResourceEntry {
        name: "power-feeds",
        path: "dcim/power-feeds/",
    },
    ResourceEntry {
        name: "power-outlet-templates",
        path: "dcim/power-outlet-templates/",
    },
    ResourceEntry {
        name: "power-outlets",
        path: "dcim/power-outlets/",
    },
    ResourceEntry {
        name: "power-panels",
        path: "dcim/power-panels/",
    },
    ResourceEntry {
        name: "power-port-templates",
        path: "dcim/power-port-templates/",
    },
    ResourceEntry {
        name: "power-ports",
        path: "dcim/power-ports/",
    },
    ResourceEntry {
        name: "rack-groups",
        path: "dcim/rack-groups/",
    },
    ResourceEntry {
        name: "rack-reservations",
        path: "dcim/rack-reservations/",
    },
    ResourceEntry {
        name: "rack-roles",
        path: "dcim/rack-roles/",
    },
    ResourceEntry {
        name: "rack-types",
        path: "dcim/rack-types/",
    },
    ResourceEntry {
        name: "racks",
        path: "dcim/racks/",
    },
    ResourceEntry {
        name: "rear-port-templates",
        path: "dcim/rear-port-templates/",
    },
    ResourceEntry {
        name: "rear-ports",
        path: "dcim/rear-ports/",
    },
    ResourceEntry {
        name: "regions",
        path: "dcim/regions/",
    },
    ResourceEntry {
        name: "site-groups",
        path: "dcim/site-groups/",
    },
    ResourceEntry {
        name: "sites",
        path: "dcim/sites/",
    },
    ResourceEntry {
        name: "virtual-chassis",
        path: "dcim/virtual-chassis/",
    },
    ResourceEntry {
        name: "virtual-device-contexts",
        path: "dcim/virtual-device-contexts/",
    },
];

pub(crate) const IPAM_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "aggregates",
        path: "ipam/aggregates/",
    },
    ResourceEntry {
        name: "asn-ranges",
        path: "ipam/asn-ranges/",
    },
    ResourceEntry {
        name: "asns",
        path: "ipam/asns/",
    },
    ResourceEntry {
        name: "fhrp-group-assignments",
        path: "ipam/fhrp-group-assignments/",
    },
    ResourceEntry {
        name: "fhrp-groups",
        path: "ipam/fhrp-groups/",
    },
    ResourceEntry {
        name: "ip-addresses",
        path: "ipam/ip-addresses/",
    },
    ResourceEntry {
        name: "ip-ranges",
        path: "ipam/ip-ranges/",
    },
    ResourceEntry {
        name: "prefixes",
        path: "ipam/prefixes/",
    },
    ResourceEntry {
        name: "rirs",
        path: "ipam/rirs/",
    },
    ResourceEntry {
        name: "roles",
        path: "ipam/roles/",
    },
    ResourceEntry {
        name: "route-targets",
        path: "ipam/route-targets/",
    },
    ResourceEntry {
        name: "service-templates",
        path: "ipam/service-templates/",
    },
    ResourceEntry {
        name: "services",
        path: "ipam/services/",
    },
    ResourceEntry {
        name: "vlan-groups",
        path: "ipam/vlan-groups/",
    },
    ResourceEntry {
        name: "vlan-translation-policies",
        path: "ipam/vlan-translation-policies/",
    },
    ResourceEntry {
        name: "vlan-translation-rules",
        path: "ipam/vlan-translation-rules/",
    },
    ResourceEntry {
        name: "vlans",
        path: "ipam/vlans/",
    },
    ResourceEntry {
        name: "vrfs",
        path: "ipam/vrfs/",
    },
];

pub(crate) const CIRCUITS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "circuit-group-assignments",
        path: "circuits/circuit-group-assignments/",
    },
    ResourceEntry {
        name: "circuit-groups",
        path: "circuits/circuit-groups/",
    },
    ResourceEntry {
        name: "circuit-terminations",
        path: "circuits/circuit-terminations/",
    },
    ResourceEntry {
        name: "circuit-types",
        path: "circuits/circuit-types/",
    },
    ResourceEntry {
        name: "circuits",
        path: "circuits/circuits/",
    },
    ResourceEntry {
        name: "provider-accounts",
        path: "circuits/provider-accounts/",
    },
    ResourceEntry {
        name: "provider-networks",
        path: "circuits/provider-networks/",
    },
    ResourceEntry {
        name: "providers",
        path: "circuits/providers/",
    },
    ResourceEntry {
        name: "virtual-circuit-terminations",
        path: "circuits/virtual-circuit-terminations/",
    },
    ResourceEntry {
        name: "virtual-circuit-types",
        path: "circuits/virtual-circuit-types/",
    },
    ResourceEntry {
        name: "virtual-circuits",
        path: "circuits/virtual-circuits/",
    },
];

pub(crate) const TENANCY_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "contact-assignments",
        path: "tenancy/contact-assignments/",
    },
    ResourceEntry {
        name: "contact-groups",
        path: "tenancy/contact-groups/",
    },
    ResourceEntry {
        name: "contact-roles",
        path: "tenancy/contact-roles/",
    },
    ResourceEntry {
        name: "contacts",
        path: "tenancy/contacts/",
    },
    ResourceEntry {
        name: "tenant-groups",
        path: "tenancy/tenant-groups/",
    },
    ResourceEntry {
        name: "tenants",
        path: "tenancy/tenants/",
    },
];

pub(crate) const EXTRAS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "bookmarks",
        path: "extras/bookmarks/",
    },
    ResourceEntry {
        name: "config-context-profiles",
        path: "extras/config-context-profiles/",
    },
    ResourceEntry {
        name: "config-contexts",
        path: "extras/config-contexts/",
    },
    ResourceEntry {
        name: "config-templates",
        path: "extras/config-templates/",
    },
    ResourceEntry {
        name: "custom-field-choice-sets",
        path: "extras/custom-field-choice-sets/",
    },
    ResourceEntry {
        name: "custom-fields",
        path: "extras/custom-fields/",
    },
    ResourceEntry {
        name: "custom-links",
        path: "extras/custom-links/",
    },
    ResourceEntry {
        name: "event-rules",
        path: "extras/event-rules/",
    },
    ResourceEntry {
        name: "export-templates",
        path: "extras/export-templates/",
    },
    ResourceEntry {
        name: "image-attachments",
        path: "extras/image-attachments/",
    },
    ResourceEntry {
        name: "journal-entries",
        path: "extras/journal-entries/",
    },
    ResourceEntry {
        name: "notification-groups",
        path: "extras/notification-groups/",
    },
    ResourceEntry {
        name: "notifications",
        path: "extras/notifications/",
    },
    ResourceEntry {
        name: "object-types",
        path: "extras/object-types/",
    },
    ResourceEntry {
        name: "saved-filters",
        path: "extras/saved-filters/",
    },
    ResourceEntry {
        name: "scripts",
        path: "extras/scripts/",
    },
    ResourceEntry {
        name: "subscriptions",
        path: "extras/subscriptions/",
    },
    ResourceEntry {
        name: "table-configs",
        path: "extras/table-configs/",
    },
    ResourceEntry {
        name: "tagged-objects",
        path: "extras/tagged-objects/",
    },
    ResourceEntry {
        name: "tags",
        path: "extras/tags/",
    },
    ResourceEntry {
        name: "webhooks",
        path: "extras/webhooks/",
    },
];

pub(crate) const CORE_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "background-tasks",
        path: "core/background-tasks/",
    },
    ResourceEntry {
        name: "data-files",
        path: "core/data-files/",
    },
    ResourceEntry {
        name: "data-sources",
        path: "core/data-sources/",
    },
    ResourceEntry {
        name: "jobs",
        path: "core/jobs/",
    },
    ResourceEntry {
        name: "object-changes",
        path: "core/object-changes/",
    },
    ResourceEntry {
        name: "object-types",
        path: "core/object-types/",
    },
];

pub(crate) const USERS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "groups",
        path: "users/groups/",
    },
    ResourceEntry {
        name: "permissions",
        path: "users/permissions/",
    },
    ResourceEntry {
        name: "tokens",
        path: "users/tokens/",
    },
    ResourceEntry {
        name: "users",
        path: "users/users/",
    },
];

pub(crate) const VIRTUALIZATION_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cluster-groups",
        path: "virtualization/cluster-groups/",
    },
    ResourceEntry {
        name: "cluster-types",
        path: "virtualization/cluster-types/",
    },
    ResourceEntry {
        name: "clusters",
        path: "virtualization/clusters/",
    },
    ResourceEntry {
        name: "interfaces",
        path: "virtualization/interfaces/",
    },
    ResourceEntry {
        name: "virtual-disks",
        path: "virtualization/virtual-disks/",
    },
    ResourceEntry {
        name: "virtual-machine-types",
        path: "virtualization/virtual-machine-types/",
    },
    ResourceEntry {
        name: "virtual-machines",
        path: "virtualization/virtual-machines/",
    },
];

pub(crate) const VPN_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "ike-policies",
        path: "vpn/ike-policies/",
    },
    ResourceEntry {
        name: "ike-proposals",
        path: "vpn/ike-proposals/",
    },
    ResourceEntry {
        name: "ipsec-policies",
        path: "vpn/ipsec-policies/",
    },
    ResourceEntry {
        name: "ipsec-profiles",
        path: "vpn/ipsec-profiles/",
    },
    ResourceEntry {
        name: "ipsec-proposals",
        path: "vpn/ipsec-proposals/",
    },
    ResourceEntry {
        name: "l2vpn-terminations",
        path: "vpn/l2vpn-terminations/",
    },
    ResourceEntry {
        name: "l2vpns",
        path: "vpn/l2vpns/",
    },
    ResourceEntry {
        name: "tunnel-groups",
        path: "vpn/tunnel-groups/",
    },
    ResourceEntry {
        name: "tunnel-terminations",
        path: "vpn/tunnel-terminations/",
    },
    ResourceEntry {
        name: "tunnels",
        path: "vpn/tunnels/",
    },
];

pub(crate) const WIRELESS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "wireless-lan-groups",
        path: "wireless/wireless-lan-groups/",
    },
    ResourceEntry {
        name: "wireless-lans",
        path: "wireless/wireless-lans/",
    },
    ResourceEntry {
        name: "wireless-links",
        path: "wireless/wireless-links/",
    },
];

pub(crate) const PLUGINS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "branches",
        path: "plugins/branching/branches/",
    },
    ResourceEntry {
        name: "branch-events",
        path: "plugins/branching/branch-events/",
    },
    ResourceEntry {
        name: "changes",
        path: "plugins/branching/changes/",
    },
];

pub(crate) async fn handle_resource_group(
    client: &impl ApiClient,
    output: &OutputConfig,
    group: &str,
    resources: &[ResourceEntry],
    resource: &str,
    action: ResourceAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = find_resource_path(resources, resource).ok_or_else(|| {
        format!(
            "unknown {} resource '{}'. use `netbox-cli resources {}` to list options.",
            group, resource, group
        )
    })?;
    handle_resource_action(client, output, path, action).await
}

pub(crate) async fn handle_resource_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    path: &str,
    action: ResourceAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = normalize_api_path(path);
    match action {
        ResourceAction::List { query } => {
            let full_path = append_query(&path, &query)?;
            let response = request_raw_with_context(client, Method::GET, &full_path, None).await?;
            print_output(&response, output)?;
        }
        ResourceAction::Get { id } => {
            let full_path = resource_path_with_id(&path, id);
            let response = request_raw_with_context(client, Method::GET, &full_path, None).await?;
            print_output(&response, output)?;
        }
        ResourceAction::Create { input } => {
            let body: Value = load_json(&input)?;
            if output.dry_run {
                print_dry_run(Method::POST, &path, None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::POST, &path, Some(&body)).await?;
                print_output(&response, output)?;
            }
        }
        ResourceAction::Update { id, input } => {
            let body: Value = load_json(&input)?;
            let full_path = resource_path_with_id(&path, id);
            if output.dry_run {
                print_dry_run(Method::PUT, &full_path, None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::PUT, &full_path, Some(&body)).await?;
                print_output(&response, output)?;
            }
        }
        ResourceAction::Patch { id, input } => {
            let body: Value = load_json(&input)?;
            let full_path = resource_path_with_id(&path, id);
            if output.dry_run {
                print_dry_run(Method::PATCH, &full_path, None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::PATCH, &full_path, Some(&body))
                        .await?;
                print_output(&response, output)?;
            }
        }
        ResourceAction::Delete { id } => {
            let full_path = resource_path_with_id(&path, id);
            if output.dry_run {
                print_dry_run(Method::DELETE, &full_path, None, None)?;
            } else {
                let response =
                    request_raw_with_context(client, Method::DELETE, &full_path, None).await?;
                if response == Value::Null {
                    println!("deleted {}", id);
                } else {
                    print_output(&response, output)?;
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn print_resources(group: Option<&str>) {
    match group {
        None => {
            println!("dcim");
            list_resource_group(DCIM_RESOURCES);
            println!("ipam");
            list_resource_group(IPAM_RESOURCES);
            println!("circuits");
            list_resource_group(CIRCUITS_RESOURCES);
            println!("tenancy");
            list_resource_group(TENANCY_RESOURCES);
            println!("extras");
            list_resource_group(EXTRAS_RESOURCES);
            println!("core");
            list_resource_group(CORE_RESOURCES);
            println!("users");
            list_resource_group(USERS_RESOURCES);
            println!("virtualization");
            list_resource_group(VIRTUALIZATION_RESOURCES);
            println!("vpn");
            list_resource_group(VPN_RESOURCES);
            println!("wireless");
            list_resource_group(WIRELESS_RESOURCES);
            println!("plugins");
            list_resource_group(PLUGINS_RESOURCES);
        }
        Some("dcim") => list_resource_group(DCIM_RESOURCES),
        Some("ipam") => list_resource_group(IPAM_RESOURCES),
        Some("circuits") => list_resource_group(CIRCUITS_RESOURCES),
        Some("tenancy") => list_resource_group(TENANCY_RESOURCES),
        Some("extras") => list_resource_group(EXTRAS_RESOURCES),
        Some("core") => list_resource_group(CORE_RESOURCES),
        Some("users") => list_resource_group(USERS_RESOURCES),
        Some("virtualization") => list_resource_group(VIRTUALIZATION_RESOURCES),
        Some("vpn") => list_resource_group(VPN_RESOURCES),
        Some("wireless") => list_resource_group(WIRELESS_RESOURCES),
        Some("plugins") => list_resource_group(PLUGINS_RESOURCES),
        Some(other) => {
            println!("unknown group '{}'", other);
        }
    }
}

fn list_resource_group(resources: &[ResourceEntry]) {
    for entry in resources {
        println!("  {}", entry.name);
    }
}

pub(crate) fn find_resource_path(resources: &[ResourceEntry], name: &str) -> Option<&'static str> {
    resources
        .iter()
        .find(|entry| entry.name == name)
        .map(|entry| entry.path)
}

pub(crate) fn resource_path_with_id(path: &str, id: u64) -> String {
    format!("{}/{}/", path.trim_end_matches('/'), id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JsonInput;
    use crate::cli::test_util::*;
    use serde_json::{Value, json};

    #[test]
    fn find_resource_path_matches_known_resource() {
        let path = find_resource_path(DCIM_RESOURCES, "devices");
        assert_eq!(path, Some("dcim/devices/"));
        let missing = find_resource_path(DCIM_RESOURCES, "not-a-device");
        assert!(missing.is_none());
    }

    #[test]
    fn resource_path_with_id_appends_trailing_slash() {
        let path = resource_path_with_id("dcim/devices/", 42);
        assert_eq!(path, "dcim/devices/42/");
    }

    #[tokio::test]
    async fn handle_resource_action_list_calls_get() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let action = ResourceAction::List {
            query: vec!["name=leaf-1".to_string()],
        };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].method, Method::GET);
        assert_eq!(calls[0].path, "dcim/devices/?name=leaf-1");
        assert!(calls[0].body.is_none());
    }

    #[tokio::test]
    async fn handle_resource_action_get_calls_get() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let action = ResourceAction::Get { id: 42 };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::GET);
        assert_eq!(calls[0].path, "dcim/devices/42/");
    }

    #[tokio::test]
    async fn handle_resource_action_create_calls_post() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Create { input };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::POST);
        assert_eq!(calls[0].path, "dcim/devices/");
        assert_eq!(calls[0].body.as_ref().unwrap()["name"], "leaf-1");
    }

    #[tokio::test]
    async fn handle_resource_action_update_calls_put() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Update { id: 7, input };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::PUT);
        assert_eq!(calls[0].path, "dcim/devices/7/");
    }

    #[tokio::test]
    async fn handle_resource_action_patch_calls_patch() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Patch { id: 7, input };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::PATCH);
        assert_eq!(calls[0].path, "dcim/devices/7/");
    }

    #[tokio::test]
    async fn handle_resource_action_delete_calls_delete() {
        let client = FakeApiClient::new(Value::Null);
        let action = ResourceAction::Delete { id: 7 };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::DELETE);
        assert_eq!(calls[0].path, "dcim/devices/7/");
    }

    #[tokio::test]
    async fn handle_resource_group_unknown_resource_errors() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let result = handle_resource_group(
            &client,
            &output_config(),
            "dcim",
            DCIM_RESOURCES,
            "not-a-device",
            ResourceAction::List { query: vec![] },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn handle_resource_action_bubbles_api_error() {
        let client = ErrorApiClient;
        let action = ResourceAction::List { query: vec![] };
        let result =
            handle_resource_action(&client, &output_config(), "dcim/devices/", action).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn handle_resource_action_create_dry_run_skips_api() {
        let client = ErrorApiClient;
        let mut output = output_config();
        output.dry_run = true;
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Create { input };
        handle_resource_action(&client, &output, "dcim/devices/", action)
            .await
            .unwrap();
    }
}
