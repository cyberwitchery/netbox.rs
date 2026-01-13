#![doc = include_str!("../docs/cli.md")]

use clap::{Args, Parser, Subcommand};
use netbox::{Client, ClientConfig};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::{to_string_pretty, Value};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy)]
struct ResourceEntry {
    name: &'static str,
    path: &'static str,
}

const DCIM_RESOURCES: &[ResourceEntry] = &[
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

const IPAM_RESOURCES: &[ResourceEntry] = &[
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

const CIRCUITS_RESOURCES: &[ResourceEntry] = &[
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

const TENANCY_RESOURCES: &[ResourceEntry] = &[
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

const EXTRAS_RESOURCES: &[ResourceEntry] = &[
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

const CORE_RESOURCES: &[ResourceEntry] = &[
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

const USERS_RESOURCES: &[ResourceEntry] = &[
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

const VIRTUALIZATION_RESOURCES: &[ResourceEntry] = &[
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
        name: "virtual-machines",
        path: "virtualization/virtual-machines/",
    },
];

const VPN_RESOURCES: &[ResourceEntry] = &[
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

const WIRELESS_RESOURCES: &[ResourceEntry] = &[
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

const PLUGINS_RESOURCES: &[ResourceEntry] = &[
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

#[derive(Parser)]
#[command(name = "netbox-cli")]
#[command(about = "CLI tool for testing NetBox API client", long_about = None)]
struct Cli {
    /// NetBox instance URL
    #[arg(short, long, env)]
    url: String,

    /// API token
    #[arg(short, long, env)]
    token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List resources by group (or all resources)
    Resources {
        /// Resource group name (dcim, ipam, circuits, tenancy, extras, core, users, virtualization, vpn, wireless, plugins)
        group: Option<String>,
    },
    /// DCIM resources (devices, racks, interfaces, ...)
    Dcim {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// IPAM resources (prefixes, addresses, vlans, ...)
    Ipam {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Circuits resources (providers, circuits, ...)
    Circuits {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Tenancy resources (tenants, contacts, ...)
    Tenancy {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Extras resources (tags, scripts, custom fields, ...)
    Extras {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Core resources (jobs, object changes, ...)
    Core {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Users resources (users, groups, tokens, ...)
    Users {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Virtualization resources (clusters, vms, ...)
    Virtualization {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// VPN resources (tunnels, ike, ipsec, ...)
    Vpn {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Wireless resources (lans, links, ...)
    Wireless {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Plugin resources (branching data)
    Plugins {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Extras dashboard operations
    ExtrasDashboard {
        #[command(subcommand)]
        action: DashboardAction,
    },
    /// Core background queue summaries
    CoreBackgroundQueues {
        #[command(subcommand)]
        action: NamedLookupAction,
    },
    /// Core background worker summaries
    CoreBackgroundWorkers {
        #[command(subcommand)]
        action: NamedLookupAction,
    },
    /// Fetch current user config
    UsersConfig,
    /// Fetch NetBox status
    Status,
    /// Fetch OpenAPI schema
    Schema {
        /// Schema format (json, yaml)
        #[arg(long)]
        format: Option<String>,
        /// Schema language
        #[arg(long)]
        lang: Option<String>,
    },
    /// Find a device connected to a peer device/interface
    ConnectedDevice {
        /// Peer device name
        #[arg(long)]
        peer_device: String,
        /// Peer interface name
        #[arg(long)]
        peer_interface: String,
    },
    /// Provision a token with username/password
    ProvisionToken {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Branch actions (branching plugin)
    PluginBranchAction {
        id: i32,
        #[command(subcommand)]
        action: BranchAction,
    },
    /// Make a raw API request (covers all endpoints)
    Raw {
        /// HTTP method (GET, POST, PATCH, PUT, DELETE)
        #[arg(long)]
        method: String,
        /// API path, e.g. "dcim/devices/"
        #[arg(long)]
        path: String,
        /// Query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
        #[command(flatten)]
        input: JsonInputOptional,
    },
}

#[derive(Subcommand)]
enum ResourceAction {
    /// List resources
    List {
        /// Query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
    },
    /// Get a resource by id
    Get { id: i32 },
    /// Create a resource
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a resource (PUT)
    Update {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Patch a resource
    Patch {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Delete a resource
    Delete { id: i32 },
}

#[derive(Subcommand)]
enum DashboardAction {
    /// Fetch the dashboard config
    Get,
    /// Update the dashboard config (PUT)
    Update {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Patch the dashboard config
    Patch {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Delete the dashboard config
    Delete,
}

#[derive(Subcommand)]
enum NamedLookupAction {
    /// List summaries
    List,
    /// Get a summary by name
    Get { name: String },
}

#[derive(Subcommand)]
enum BranchAction {
    /// Merge a branch
    Merge {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Revert a branch
    Revert {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Sync a branch
    Sync {
        #[command(flatten)]
        input: JsonInput,
    },
}

#[derive(Args, Debug)]
struct JsonInput {
    /// JSON payload string
    #[arg(long, required_unless_present = "file")]
    json: Option<String>,
    /// JSON payload file path
    #[arg(long, required_unless_present = "json")]
    file: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct JsonInputOptional {
    /// JSON payload string
    #[arg(long)]
    json: Option<String>,
    /// JSON payload file path
    #[arg(long)]
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = ClientConfig::new(&cli.url, &cli.token);
    let client = Client::new(config)?;

    match cli.command {
        Commands::Resources { group } => {
            print_resources(group.as_deref());
        }
        Commands::Dcim { resource, action } => {
            handle_resource_group(&client, "dcim", DCIM_RESOURCES, &resource, action).await?;
        }
        Commands::Ipam { resource, action } => {
            handle_resource_group(&client, "ipam", IPAM_RESOURCES, &resource, action).await?;
        }
        Commands::Circuits { resource, action } => {
            handle_resource_group(&client, "circuits", CIRCUITS_RESOURCES, &resource, action)
                .await?;
        }
        Commands::Tenancy { resource, action } => {
            handle_resource_group(&client, "tenancy", TENANCY_RESOURCES, &resource, action).await?;
        }
        Commands::Extras { resource, action } => {
            handle_resource_group(&client, "extras", EXTRAS_RESOURCES, &resource, action).await?;
        }
        Commands::Core { resource, action } => {
            handle_resource_group(&client, "core", CORE_RESOURCES, &resource, action).await?;
        }
        Commands::Users { resource, action } => {
            handle_resource_group(&client, "users", USERS_RESOURCES, &resource, action).await?;
        }
        Commands::Virtualization { resource, action } => {
            handle_resource_group(
                &client,
                "virtualization",
                VIRTUALIZATION_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Vpn { resource, action } => {
            handle_resource_group(&client, "vpn", VPN_RESOURCES, &resource, action).await?;
        }
        Commands::Wireless { resource, action } => {
            handle_resource_group(&client, "wireless", WIRELESS_RESOURCES, &resource, action)
                .await?;
        }
        Commands::Plugins { resource, action } => {
            handle_resource_group(&client, "plugins", PLUGINS_RESOURCES, &resource, action).await?;
        }
        Commands::ExtrasDashboard { action } => {
            handle_dashboard_action(&client, action).await?;
        }
        Commands::CoreBackgroundQueues { action } => {
            handle_named_lookup(&client, "core/background-queues/", action).await?;
        }
        Commands::CoreBackgroundWorkers { action } => {
            handle_named_lookup(&client, "core/background-workers/", action).await?;
        }
        Commands::UsersConfig => {
            let response = client.request_raw(Method::GET, "users/config/", None).await?;
            print_json(&response)?;
        }
        Commands::Status => {
            let status = client.status().status().await?;
            let value = serde_json::to_value(status)?;
            print_json(&value)?;
        }
        Commands::Schema { format, lang } => {
            let schema = client
                .schema()
                .schema(format.as_deref(), lang.as_deref())
                .await?;
            let value = serde_json::to_value(schema)?;
            print_json(&value)?;
        }
        Commands::ConnectedDevice {
            peer_device,
            peer_interface,
        } => {
            let response = client
                .request_raw(
                    Method::GET,
                    &append_query(
                        "dcim/connected-device/",
                        &[
                            format!("peer_device={}", peer_device),
                            format!("peer_interface={}", peer_interface),
                        ],
                    )?,
                    None,
                )
                .await?;
            print_json(&response)?;
        }
        Commands::ProvisionToken { input } => {
            let request: Value = load_json(&input)?;
            let response = client
                .request_raw(Method::POST, "users/tokens/provision/", Some(&request))
                .await?;
            print_json(&response)?;
        }
        Commands::PluginBranchAction { id, action } => {
            handle_branch_action(&client, id, action).await?;
        }
        Commands::Raw {
            method,
            path,
            query,
            input,
        } => {
            let method = Method::from_bytes(method.as_bytes())?;
            let body: Option<Value> = load_json_optional(&input)?;
            let path = normalize_api_path(&path);
            let full_path = append_query(&path, &query)?;
            let response = client
                .request_raw(method, &full_path, body.as_ref())
                .await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

async fn handle_resource_group(
    client: &Client,
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
    handle_resource_action(client, path, action).await
}

async fn handle_resource_action(
    client: &Client,
    path: &str,
    action: ResourceAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = normalize_api_path(path);
    match action {
        ResourceAction::List { query } => {
            let full_path = append_query(&path, &query)?;
            let response = client.request_raw(Method::GET, &full_path, None).await?;
            print_json(&response)?;
        }
        ResourceAction::Get { id } => {
            let full_path = resource_path_with_id(&path, id);
            let response = client.request_raw(Method::GET, &full_path, None).await?;
            print_json(&response)?;
        }
        ResourceAction::Create { input } => {
            let body: Value = load_json(&input)?;
            let response = client.request_raw(Method::POST, &path, Some(&body)).await?;
            print_json(&response)?;
        }
        ResourceAction::Update { id, input } => {
            let body: Value = load_json(&input)?;
            let full_path = resource_path_with_id(&path, id);
            let response = client
                .request_raw(Method::PUT, &full_path, Some(&body))
                .await?;
            print_json(&response)?;
        }
        ResourceAction::Patch { id, input } => {
            let body: Value = load_json(&input)?;
            let full_path = resource_path_with_id(&path, id);
            let response = client
                .request_raw(Method::PATCH, &full_path, Some(&body))
                .await?;
            print_json(&response)?;
        }
        ResourceAction::Delete { id } => {
            let full_path = resource_path_with_id(&path, id);
            let response = client.request_raw(Method::DELETE, &full_path, None).await?;
            if response == Value::Null {
                println!("deleted {}", id);
            } else {
                print_json(&response)?;
            }
        }
    }

    Ok(())
}

async fn handle_dashboard_action(
    client: &Client,
    action: DashboardAction,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        DashboardAction::Get => {
            let response = client.request_raw(Method::GET, "extras/dashboard/", None).await?;
            print_json(&response)?;
        }
        DashboardAction::Update { input } => {
            let body: Value = load_json(&input)?;
            let response = client
                .request_raw(Method::PUT, "extras/dashboard/", Some(&body))
                .await?;
            print_json(&response)?;
        }
        DashboardAction::Patch { input } => {
            let body: Value = load_json(&input)?;
            let response = client
                .request_raw(Method::PATCH, "extras/dashboard/", Some(&body))
                .await?;
            print_json(&response)?;
        }
        DashboardAction::Delete => {
            let response = client
                .request_raw(Method::DELETE, "extras/dashboard/", None)
                .await?;
            if response == Value::Null {
                println!("deleted dashboard");
            } else {
                print_json(&response)?;
            }
        }
    }

    Ok(())
}

async fn handle_named_lookup(
    client: &Client,
    base_path: &str,
    action: NamedLookupAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = normalize_api_path(base_path);
    match action {
        NamedLookupAction::List => {
            let response = client.request_raw(Method::GET, &base_path, None).await?;
            print_json(&response)?;
        }
        NamedLookupAction::Get { name } => {
            let path = format!("{}{}/", base_path.trim_end_matches('/'), name);
            let response = client.request_raw(Method::GET, &path, None).await?;
            print_json(&response)?;
        }
    }

    Ok(())
}

async fn handle_branch_action(
    client: &Client,
    id: i32,
    action: BranchAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let (suffix, body) = match action {
        BranchAction::Merge { input } => ("merge", load_json(&input)?),
        BranchAction::Revert { input } => ("revert", load_json(&input)?),
        BranchAction::Sync { input } => ("sync", load_json(&input)?),
    };

    let path = format!("plugins/branching/branches/{}/{}/", id, suffix);
    let response = client
        .request_raw(Method::POST, &path, Some(&body))
        .await?;
    print_json(&response)?;
    Ok(())
}

fn print_resources(group: Option<&str>) {
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

fn find_resource_path(resources: &[ResourceEntry], name: &str) -> Option<&'static str> {
    resources
        .iter()
        .find(|entry| entry.name == name)
        .map(|entry| entry.path)
}

fn resource_path_with_id(path: &str, id: i32) -> String {
    format!("{}/{}/", path.trim_end_matches('/'), id)
}

fn normalize_api_path(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    match trimmed.strip_prefix("api/") {
        Some(stripped) => stripped.to_string(),
        None => trimmed.to_string(),
    }
}

fn print_json(value: &Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", to_string_pretty(value)?);
    Ok(())
}

fn load_json<T>(input: &JsonInput) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let content = if let Some(json) = &input.json {
        json.clone()
    } else if let Some(path) = &input.file {
        fs::read_to_string(path)?
    } else {
        return Err("Provide --json or --file".into());
    };

    Ok(serde_json::from_str(&content)?)
}

fn load_json_optional<T>(input: &JsonInputOptional) -> Result<Option<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let content = if let Some(json) = &input.json {
        Some(json.clone())
    } else if let Some(path) = &input.file {
        Some(fs::read_to_string(path)?)
    } else {
        None
    };

    match content {
        Some(content) => Ok(Some(serde_json::from_str(&content)?)),
        None => Ok(None),
    }
}

fn append_query(path: &str, query: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let pairs = parse_query_pairs(query)?;
    if pairs.is_empty() {
        return Ok(path.to_string());
    }

    let query_string = serde_urlencoded::to_string(pairs)?;
    let separator = if path.contains('?') { "&" } else { "?" };
    Ok(format!("{}{}{}", path, separator, query_string))
}

fn parse_query_pairs(
    query: &[String],
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut pairs = Vec::with_capacity(query.len());
    for item in query {
        let mut parts = item.splitn(2, '=');
        let key = parts.next().unwrap_or_default();
        let value = parts.next();
        if key.is_empty() || value.is_none() {
            return Err(format!("Invalid query parameter: {}", item).into());
        }
        pairs.push((key.to_string(), value.unwrap().to_string()));
    }
    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn load_json_from_inline() {
        let input = JsonInput {
            json: Some(r#"{"name":"carrier","slug":"carrier"}"#.to_string()),
            file: None,
        };
        let value: Value = load_json(&input).unwrap();
        assert_eq!(value["name"], "carrier");
        assert_eq!(value["slug"], "carrier");
    }

    #[test]
    fn load_json_from_file() {
        let mut path = env::temp_dir();
        path.push("netbox-cli-test.json");
        fs::write(&path, r#"{"name":"carrier","slug":"carrier"}"#).unwrap();

        let input = JsonInput {
            json: None,
            file: Some(path.clone()),
        };
        let value: Value = load_json(&input).unwrap();
        assert_eq!(value["name"], "carrier");
        assert_eq!(value["slug"], "carrier");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_json_requires_input() {
        let input = JsonInput {
            json: None,
            file: None,
        };
        let result: Result<Value, _> = load_json(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_json_optional_handles_none() {
        let input = JsonInputOptional {
            json: None,
            file: None,
        };
        let value: Option<Value> = load_json_optional(&input).unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn append_query_encodes_pairs() {
        let path = "dcim/devices/";
        let query = vec!["name=leaf 1".to_string(), "limit=5".to_string()];
        let full = append_query(path, &query).unwrap();
        assert_eq!(full, "dcim/devices/?name=leaf+1&limit=5");
    }

    #[test]
    fn append_query_rejects_missing_value() {
        let path = "dcim/devices/";
        let query = vec!["name".to_string()];
        let result = append_query(path, &query);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_api_path_strips_prefix() {
        assert_eq!(normalize_api_path("api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("/api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("dcim/devices/"), "dcim/devices/");
    }

    #[test]
    fn resource_path_with_id_appends_trailing_slash() {
        let path = resource_path_with_id("dcim/devices/", 42);
        assert_eq!(path, "dcim/devices/42/");
    }
}
