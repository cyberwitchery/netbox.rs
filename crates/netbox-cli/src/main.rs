#![doc = include_str!("../docs/cli.md")]

mod config;

use clap::{Args, Parser, Subcommand, ValueEnum};
use comfy_table::{Cell, ContentArrangement, Table};
use config::{ConfigFile, Profile, config_path, load_config, validate_profile};
use netbox::{Client, ClientConfig};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::{Value, to_string_pretty};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use terminal_size::{Width, terminal_size};

#[async_trait::async_trait]
trait ApiClient {
    async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
    async fn graphql(
        &self,
        query: &str,
        variables: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
    async fn status(&self) -> Result<Value, Box<dyn std::error::Error>>;
    async fn schema(
        &self,
        format: Option<&str>,
        lang: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
}

struct NetboxApiClient {
    inner: Client,
}

#[async_trait::async_trait]
impl ApiClient for NetboxApiClient {
    async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        Ok(self.inner.request_raw(method, path, body).await?)
    }

    async fn graphql(
        &self,
        query: &str,
        variables: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let data = self
            .inner
            .graphql()
            .query(query, variables.cloned())
            .await?;
        Ok(data)
    }

    async fn status(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let status = self.inner.status().status().await?;
        Ok(serde_json::to_value(status)?)
    }

    async fn schema(
        &self,
        format: Option<&str>,
        lang: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let schema = self.inner.schema().schema(format, lang).await?;
        Ok(serde_json::to_value(schema)?)
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
    Table,
}

#[derive(Clone, Debug)]
struct OutputConfig {
    format: OutputFormat,
    select: Option<String>,
    columns: Option<Vec<String>>,
    max_columns: usize,
    dry_run: bool,
}

#[derive(Debug)]
struct RequestError {
    method: Method,
    path: String,
    source: Box<dyn std::error::Error + 'static>,
}

impl RequestError {
    fn new(
        method: Method,
        path: impl Into<String>,
        source: Box<dyn std::error::Error + 'static>,
    ) -> Self {
        Self {
            method,
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = format_netbox_error(&self.method, &self.path, self.source.as_ref()) {
            return write!(f, "{message}");
        }
        write!(
            f,
            "request failed: {} {}: {}",
            self.method.as_str(),
            self.path,
            self.source
        )
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}

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
    /// NetBox instance URL (overrides config file)
    #[arg(short, long, env = "NETBOX_URL")]
    url: Option<String>,

    /// API token (overrides config file)
    #[arg(short, long, env = "NETBOX_TOKEN")]
    token: Option<String>,

    /// Config profile to use (default: "default")
    #[arg(short, long, default_value = "default")]
    profile: String,

    /// Output format (json, yaml, table)
    #[arg(long, value_enum)]
    output: Option<OutputFormat>,

    /// Select a field from the response (dot path)
    #[arg(long)]
    select: Option<String>,

    /// Columns to show in table output (comma-separated)
    #[arg(long, value_delimiter = ',')]
    columns: Option<Vec<String>>,

    /// Maximum columns in table output (default: 6)
    #[arg(long, default_value = "6")]
    max_columns: usize,

    /// Print the request and skip write operations
    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show the resolved configuration for a profile
    Show,
    /// List all available profiles
    List,
    /// Validate a profile configuration
    Validate,
    /// Show the config file path
    Path,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage configuration profiles
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
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
    /// Run a read-only graphql query
    Graphql {
        #[command(flatten)]
        input: GraphqlInput,
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
        id: u64,
        #[command(subcommand)]
        action: BranchAction,
    },
    /// List or create available IPs in a prefix
    IpamPrefixAvailableIps {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// List or create available prefixes in a prefix
    IpamPrefixAvailablePrefixes {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// List or create available IPs in an IP range
    IpamRangeAvailableIps {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// List or create available VLANs in a VLAN group
    IpamVlanGroupAvailableVlans {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// List or create available ASNs in an ASN range
    IpamAsnRangeAvailableAsns {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// Background task actions
    CoreTaskAction {
        id: String,
        #[command(subcommand)]
        action: TaskAction,
    },
    /// Sync a data source
    CoreDataSourceSync { id: u64 },
    /// Sync a config context
    ExtrasConfigContextSync { id: u64 },
    /// Sync a config context profile
    ExtrasConfigContextProfileSync { id: u64 },
    /// Sync a config template
    ExtrasConfigTemplateSync { id: u64 },
    /// Render a config template
    ExtrasConfigTemplateRender { id: u64 },
    /// Sync an export template
    ExtrasExportTemplateSync { id: u64 },
    /// Get custom field choices
    ExtrasCustomFieldChoices { id: u64 },
    /// Get circuit termination paths
    CircuitsTerminationPaths { id: u64 },
    /// Get virtual circuit termination paths
    CircuitsVirtualTerminationPaths { id: u64 },
    /// Trace DCIM resources (interfaces, ports, feeds)
    DcimTrace {
        #[command(subcommand)]
        resource: TraceableResource,
    },
    /// Render VM config
    VirtualizationRenderConfig { id: u64 },
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
    Get { id: u64 },
    /// Create a resource
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a resource (PUT)
    Update {
        id: u64,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Patch a resource
    Patch {
        id: u64,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Delete a resource
    Delete { id: u64 },
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

#[derive(Subcommand)]
enum AvailabilityAction {
    /// List available resources
    List,
    /// Create resources from available pool
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
}

#[derive(Subcommand)]
enum TaskAction {
    /// Enqueue a background task
    Enqueue,
    /// Stop a background task
    Stop,
    /// Requeue a background task
    Requeue,
    /// Delete a background task
    Delete,
}

#[derive(Subcommand)]
enum TraceableResource {
    /// Trace an interface
    Interface { id: u64 },
    /// Trace a console port
    ConsolePort { id: u64 },
    /// Trace a console server port
    ConsoleServerPort { id: u64 },
    /// Trace a power port
    PowerPort { id: u64 },
    /// Trace a power outlet
    PowerOutlet { id: u64 },
    /// Trace a power feed
    PowerFeed { id: u64 },
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

#[derive(Args, Debug)]
struct GraphqlInput {
    /// GraphQL query string
    #[arg(long, required_unless_present = "query_file")]
    query: Option<String>,
    /// GraphQL query file path
    #[arg(long, required_unless_present = "query")]
    query_file: Option<PathBuf>,
    /// JSON variables payload
    #[arg(long)]
    vars: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load config file
    let config_file = load_config().ok().flatten();

    // Handle config commands first (no API access needed)
    if let Commands::Config { action } = &cli.command {
        return handle_config_command(action, &cli.profile, config_file.as_ref());
    }

    // Resolve profile from config file
    let mut profile = Profile::default();
    if let Some(ref cf) = config_file {
        if let Some(p) = cf.get_profile(&cli.profile) {
            profile = p.clone();
        } else if cli.profile != "default" {
            return Err(format!("profile '{}' not found in config file", cli.profile).into());
        }
    }

    // CLI args override config
    if cli.url.is_some() {
        profile.url = cli.url.clone();
    }
    if cli.token.is_some() {
        profile.token = cli.token.clone();
    }
    if cli.output.is_some() {
        profile.output = cli.output.map(|o| format!("{:?}", o).to_lowercase());
    }

    // Resolve URL and token
    let url = profile
        .url
        .clone()
        .ok_or("url not specified (use --url, NETBOX_URL, or config file)")?;
    let token = profile.resolve_token()?.ok_or(
        "token not specified (use --token, NETBOX_TOKEN, token_env, or token_command in config)",
    )?;

    // Build client config
    let mut client_config = ClientConfig::new(&url, &token);
    if let Some(timeout) = profile.timeout {
        client_config = client_config.with_timeout(Duration::from_secs(timeout));
    }
    if let Some(retries) = profile.retries {
        client_config = client_config.with_max_retries(retries);
    }
    if let Some(ssl_verify) = profile.ssl_verify {
        client_config = client_config.with_ssl_verification(ssl_verify);
    }

    let client = Client::new(client_config)?;
    let api = NetboxApiClient { inner: client };

    // Resolve output format
    let output_format = cli.output.unwrap_or_else(|| {
        profile
            .output
            .as_deref()
            .and_then(|s| match s {
                "json" => Some(OutputFormat::Json),
                "yaml" => Some(OutputFormat::Yaml),
                "table" => Some(OutputFormat::Table),
                _ => None,
            })
            .unwrap_or(OutputFormat::Json)
    });

    let output = OutputConfig {
        format: output_format,
        select: cli.select.clone(),
        columns: cli.columns.clone(),
        max_columns: cli.max_columns,
        dry_run: cli.dry_run,
    };

    match cli.command {
        Commands::Config { .. } => unreachable!(), // handled above
        Commands::Resources { group } => {
            print_resources(group.as_deref());
        }
        Commands::Dcim { resource, action } => {
            handle_resource_group(&api, &output, "dcim", DCIM_RESOURCES, &resource, action).await?;
        }
        Commands::Ipam { resource, action } => {
            handle_resource_group(&api, &output, "ipam", IPAM_RESOURCES, &resource, action).await?;
        }
        Commands::Circuits { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "circuits",
                CIRCUITS_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Tenancy { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "tenancy",
                TENANCY_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Extras { resource, action } => {
            handle_resource_group(&api, &output, "extras", EXTRAS_RESOURCES, &resource, action)
                .await?;
        }
        Commands::Core { resource, action } => {
            handle_resource_group(&api, &output, "core", CORE_RESOURCES, &resource, action).await?;
        }
        Commands::Users { resource, action } => {
            handle_resource_group(&api, &output, "users", USERS_RESOURCES, &resource, action)
                .await?;
        }
        Commands::Virtualization { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "virtualization",
                VIRTUALIZATION_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Vpn { resource, action } => {
            handle_resource_group(&api, &output, "vpn", VPN_RESOURCES, &resource, action).await?;
        }
        Commands::Wireless { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "wireless",
                WIRELESS_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Plugins { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "plugins",
                PLUGINS_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::ExtrasDashboard { action } => {
            handle_dashboard_action(&api, &output, action).await?;
        }
        Commands::CoreBackgroundQueues { action } => {
            handle_named_lookup(&api, &output, "core/background-queues/", action).await?;
        }
        Commands::CoreBackgroundWorkers { action } => {
            handle_named_lookup(&api, &output, "core/background-workers/", action).await?;
        }
        Commands::UsersConfig => {
            let response =
                request_raw_with_context(&api, Method::GET, "users/config/", None).await?;
            print_output(&response, &output)?;
        }
        Commands::Status => {
            let value = api
                .status()
                .await
                .map_err(|err| wrap_request_error(Method::GET, "status/", err))?;
            print_output(&value, &output)?;
        }
        Commands::Schema { format, lang } => {
            let schema_path = build_schema_path(format.as_deref(), lang.as_deref())?;
            let value = api
                .schema(format.as_deref(), lang.as_deref())
                .await
                .map_err(|err| wrap_request_error(Method::GET, &schema_path, err))?;
            print_output(&value, &output)?;
        }
        Commands::Graphql { input } => {
            let query = load_graphql_query(&input)?;
            let vars = load_graphql_vars(&input)?;
            let response = api
                .graphql(&query, vars.as_ref())
                .await
                .map_err(|err| wrap_request_error(Method::POST, "graphql/", err))?;
            print_output(&response, &output)?;
        }
        Commands::ConnectedDevice {
            peer_device,
            peer_interface,
        } => {
            let path = append_query(
                "dcim/connected-device/",
                &[
                    format!("peer_device={}", peer_device),
                    format!("peer_interface={}", peer_interface),
                ],
            )?;
            let response = request_raw_with_context(&api, Method::GET, &path, None).await?;
            print_output(&response, &output)?;
        }
        Commands::ProvisionToken { input } => {
            let request: Value = load_json(&input)?;
            if output.dry_run {
                print_dry_run(
                    Method::POST,
                    "users/tokens/provision/",
                    None,
                    Some(&request),
                )?;
            } else {
                let response = request_raw_with_context(
                    &api,
                    Method::POST,
                    "users/tokens/provision/",
                    Some(&request),
                )
                .await?;
                print_output(&response, &output)?;
            }
        }
        Commands::PluginBranchAction { id, action } => {
            handle_branch_action(&api, &output, id, action).await?;
        }
        Commands::IpamPrefixAvailableIps { id, action } => {
            let path = format!("ipam/prefixes/{}/available-ips/", id);
            handle_availability_action(&api, &output, &path, action).await?;
        }
        Commands::IpamPrefixAvailablePrefixes { id, action } => {
            let path = format!("ipam/prefixes/{}/available-prefixes/", id);
            handle_availability_action(&api, &output, &path, action).await?;
        }
        Commands::IpamRangeAvailableIps { id, action } => {
            let path = format!("ipam/ip-ranges/{}/available-ips/", id);
            handle_availability_action(&api, &output, &path, action).await?;
        }
        Commands::IpamVlanGroupAvailableVlans { id, action } => {
            let path = format!("ipam/vlan-groups/{}/available-vlans/", id);
            handle_availability_action(&api, &output, &path, action).await?;
        }
        Commands::IpamAsnRangeAvailableAsns { id, action } => {
            let path = format!("ipam/asn-ranges/{}/available-asns/", id);
            handle_availability_action(&api, &output, &path, action).await?;
        }
        Commands::CoreTaskAction { id, action } => {
            handle_task_action(&api, &output, &id, action).await?;
        }
        Commands::CoreDataSourceSync { id } => {
            let path = format!("core/data-sources/{}/sync/", id);
            handle_sync_action(&api, &output, &path).await?;
        }
        Commands::ExtrasConfigContextSync { id } => {
            let path = format!("extras/config-contexts/{}/sync/", id);
            handle_sync_action(&api, &output, &path).await?;
        }
        Commands::ExtrasConfigContextProfileSync { id } => {
            let path = format!("extras/config-context-profiles/{}/sync/", id);
            handle_sync_action(&api, &output, &path).await?;
        }
        Commands::ExtrasConfigTemplateSync { id } => {
            let path = format!("extras/config-templates/{}/sync/", id);
            handle_sync_action(&api, &output, &path).await?;
        }
        Commands::ExtrasConfigTemplateRender { id } => {
            let path = format!("extras/config-templates/{}/render/", id);
            handle_sync_action(&api, &output, &path).await?;
        }
        Commands::ExtrasExportTemplateSync { id } => {
            let path = format!("extras/export-templates/{}/sync/", id);
            handle_sync_action(&api, &output, &path).await?;
        }
        Commands::ExtrasCustomFieldChoices { id } => {
            let path = format!("extras/custom-field-choice-sets/{}/choices/", id);
            handle_get_action(&api, &output, &path).await?;
        }
        Commands::CircuitsTerminationPaths { id } => {
            let path = format!("circuits/circuit-terminations/{}/paths/", id);
            handle_get_action(&api, &output, &path).await?;
        }
        Commands::CircuitsVirtualTerminationPaths { id } => {
            let path = format!("circuits/virtual-circuit-terminations/{}/paths/", id);
            handle_get_action(&api, &output, &path).await?;
        }
        Commands::DcimTrace { resource } => {
            handle_trace_action(&api, &output, resource).await?;
        }
        Commands::VirtualizationRenderConfig { id } => {
            let path = format!("virtualization/virtual-machines/{}/render-config/", id);
            handle_sync_action(&api, &output, &path).await?;
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
            if output.dry_run && method != Method::GET {
                print_dry_run(method, &full_path, None, body.as_ref())?;
            } else {
                let response =
                    request_raw_with_context(&api, method, &full_path, body.as_ref()).await?;
                print_output(&response, &output)?;
            }
        }
    }

    Ok(())
}

async fn handle_resource_group(
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

async fn handle_resource_action(
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

async fn handle_dashboard_action(
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

async fn handle_named_lookup(
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

async fn handle_branch_action(
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

async fn handle_availability_action(
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

async fn handle_task_action(
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

async fn handle_sync_action(
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

async fn handle_get_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = request_raw_with_context(client, Method::GET, path, None).await?;
    print_output(&response, output)?;
    Ok(())
}

async fn handle_trace_action(
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

fn handle_config_command(
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
                                // Try to resolve token to catch command errors
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

fn resource_path_with_id(path: &str, id: u64) -> String {
    format!("{}/{}/", path.trim_end_matches('/'), id)
}

fn normalize_api_path(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    match trimmed.strip_prefix("api/") {
        Some(stripped) => stripped.to_string(),
        None => trimmed.to_string(),
    }
}

fn print_output(value: &Value, output: &OutputConfig) -> Result<(), Box<dyn std::error::Error>> {
    let formatted = format_output(value, output)?;
    println!("{formatted}");
    Ok(())
}

async fn request_raw_with_context(
    client: &impl ApiClient,
    method: Method,
    path: &str,
    body: Option<&Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    client
        .request_raw(method.clone(), path, body)
        .await
        .map_err(|err| wrap_request_error(method, path, err))
}

fn wrap_request_error(
    method: Method,
    path: &str,
    err: Box<dyn std::error::Error + 'static>,
) -> Box<dyn std::error::Error> {
    Box::new(RequestError::new(method, path, err))
}

fn format_output(
    value: &Value,
    output: &OutputConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let selected = match output.select.as_deref() {
        Some(path) => select_value(value, path),
        None => value.clone(),
    };

    match output.format {
        OutputFormat::Json => Ok(to_string_pretty(&selected)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&selected)?),
        OutputFormat::Table => Ok(format_table(
            &selected,
            output.columns.as_deref(),
            output.max_columns,
        )),
    }
}

fn format_table(value: &Value, columns: Option<&[String]>, max_columns: usize) -> String {
    let width = terminal_width().unwrap_or(120).min(u16::MAX as usize) as u16;
    if let Value::Object(map) = value {
        if let Some(Value::Array(items)) = map.get("results") {
            let summary = format_table_summary(map);
            let table = table_from_items(items, width, columns, max_columns);
            return if summary.is_empty() {
                table
            } else {
                format!("{summary}\n{table}")
            };
        }
    }

    match value {
        Value::Array(items) => table_from_items(items, width, columns, max_columns),
        Value::Object(map) => {
            let mut table = base_table(width);
            let headers: Vec<String> = if let Some(cols) = columns {
                cols.to_vec()
            } else {
                map.keys().take(max_columns).cloned().collect()
            };
            table.set_header(headers.iter().map(Cell::new));
            let row = headers
                .iter()
                .map(|key| Cell::new(value_to_cell(map.get(key))))
                .collect::<Vec<_>>();
            table.add_row(row);
            table.to_string()
        }
        _ => {
            let mut table = base_table(width);
            table.set_header(vec![Cell::new("value")]);
            table.add_row(vec![Cell::new(value_to_cell(Some(value)))]);
            table.to_string()
        }
    }
}

fn terminal_width() -> Option<usize> {
    terminal_size().map(|(Width(width), _)| width as usize)
}

fn value_to_cell(value: Option<&Value>) -> String {
    match value {
        Some(Value::Null) | None => "".to_string(),
        Some(Value::String(value)) => value.clone(),
        Some(Value::Number(value)) => value.to_string(),
        Some(Value::Bool(value)) => value.to_string(),
        Some(Value::Array(items)) => format!("[{}]", items.len()),
        Some(Value::Object(map)) => extract_display(map)
            .or_else(|| {
                map.get("id")
                    .and_then(Value::as_i64)
                    .map(|id| id.to_string())
            })
            .unwrap_or_else(|| compact_json(&Value::Object(map.clone()))),
    }
}

fn base_table(width: u16) -> Table {
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::ASCII_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(width);
    table
}

fn table_from_items(
    items: &[Value],
    width: u16,
    columns: Option<&[String]>,
    max_columns: usize,
) -> String {
    let mut table = base_table(width);
    if items.is_empty() {
        let headers = columns
            .filter(|cols| !cols.is_empty())
            .map(|cols| cols.to_vec())
            .unwrap_or_else(|| vec!["value".to_string()]);
        table.set_header(headers.iter().map(Cell::new));
        return table.to_string();
    }

    if let Some(Value::Object(first)) = items.first() {
        let headers = if let Some(cols) = columns {
            cols.to_vec()
        } else {
            infer_columns(items, first, max_columns)
        };
        table.set_header(headers.iter().map(Cell::new));
        for item in items {
            if let Value::Object(map) = item {
                let row = headers
                    .iter()
                    .map(|key| Cell::new(value_to_cell(map.get(key))))
                    .collect::<Vec<_>>();
                table.add_row(row);
            } else {
                table.add_row(vec![Cell::new(value_to_cell(Some(item)))]);
            }
        }
    } else if let Some(cols) = columns {
        // Empty result set with explicit columns: render the headers anyway.
        table.set_header(cols.iter().map(Cell::new));
    } else {
        table.set_header(vec![Cell::new("value")]);
        for item in items {
            table.add_row(vec![Cell::new(value_to_cell(Some(item)))]);
        }
    }
    table.to_string()
}

fn infer_columns(
    items: &[Value],
    first: &serde_json::Map<String, Value>,
    max_columns: usize,
) -> Vec<String> {
    let preferred = [
        "id",
        "name",
        "display",
        "slug",
        "status",
        "site",
        "role",
        "device_type",
        "manufacturer",
        "model",
        "url",
    ];

    let mut columns = Vec::new();
    for key in preferred {
        if first.contains_key(key) && columns.len() < max_columns {
            columns.push(key.to_string());
        }
    }

    if columns.is_empty() {
        columns = first.keys().take(max_columns).cloned().collect();
    }

    if columns.len() < max_columns {
        let mut additional = first
            .keys()
            .filter(|key| !columns.iter().any(|col| col == *key))
            .take(max_columns - columns.len())
            .cloned()
            .collect::<Vec<_>>();
        columns.append(&mut additional);
    }

    if columns.len() > max_columns {
        columns.truncate(max_columns);
    }

    if columns.len() > 1 && items.iter().any(|item| matches!(item, Value::Object(_))) {
        columns
    } else {
        vec!["value".to_string()]
    }
}

fn format_table_summary(map: &serde_json::Map<String, Value>) -> String {
    let count = map
        .get("count")
        .and_then(Value::as_i64)
        .map(|v| v.to_string());
    let next = map
        .get("next")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let previous = map
        .get("previous")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let mut parts = Vec::new();
    if let Some(count) = count {
        parts.push(format!("count: {count}"));
    }
    if !next.is_empty() {
        parts.push(format!("next: {next}"));
    }
    if !previous.is_empty() {
        parts.push(format!("previous: {previous}"));
    }
    parts.join(" | ")
}

fn extract_display(map: &serde_json::Map<String, Value>) -> Option<String> {
    for key in ["display", "name", "label", "value", "slug"] {
        if let Some(Value::String(value)) = map.get(key) {
            return Some(value.clone());
        }
    }
    None
}

fn compact_json(value: &Value) -> String {
    let raw = serde_json::to_string(value).unwrap_or_else(|_| "<invalid>".to_string());
    if raw.len() > 120 {
        let end = raw.floor_char_boundary(117);
        format!("{}...", &raw[..end])
    } else {
        raw
    }
}

fn format_netbox_error(
    method: &Method,
    path: &str,
    err: &(dyn std::error::Error + 'static),
) -> Option<String> {
    let netbox_err = err.downcast_ref::<netbox::Error>()?;
    let netbox::Error::ApiError {
        status,
        message,
        body,
    } = netbox_err
    else {
        return None;
    };

    let mut detail = format!("status {}", status);
    if let Some(request_id) = extract_request_id(body) {
        detail.push_str(&format!(", request_id {request_id}"));
    }
    let mut summary = format!("request failed: {} {} ({detail})", method.as_str(), path);
    if !message.is_empty() {
        summary.push_str(": ");
        summary.push_str(message);
    }
    Some(summary)
}

fn extract_request_id(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    for key in ["request_id", "requestId", "request-id"] {
        if let Some(Value::String(id)) = value.get(key) {
            return Some(id.clone());
        }
    }
    None
}

fn select_value(value: &Value, path: &str) -> Value {
    let segments: Vec<&str> = path.split('.').filter(|seg| !seg.is_empty()).collect();
    select_value_segments(value, &segments)
}

fn select_value_segments(value: &Value, segments: &[&str]) -> Value {
    if segments.is_empty() {
        return value.clone();
    }

    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| select_value_segments(item, segments))
                .collect(),
        ),
        Value::Object(map) => map
            .get(segments[0])
            .map(|next| select_value_segments(next, &segments[1..]))
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

fn print_dry_run(
    method: Method,
    path: &str,
    query: Option<&[String]>,
    body: Option<&Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_path = match query {
        Some(query) => append_query(path, query)?,
        None => path.to_string(),
    };
    let payload = dry_run_payload(method, &full_path, body);
    println!("{}", to_string_pretty(&payload)?);
    Ok(())
}

fn dry_run_payload(method: Method, path: &str, body: Option<&Value>) -> Value {
    serde_json::json!({
        "method": method.as_str(),
        "path": path,
        "body": body,
    })
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

fn load_graphql_query(input: &GraphqlInput) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(query) = &input.query {
        return Ok(query.clone());
    }
    if let Some(path) = &input.query_file {
        return Ok(fs::read_to_string(path)?);
    }
    Err("Provide --query or --query-file".into())
}

fn load_graphql_vars(input: &GraphqlInput) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match &input.vars {
        Some(vars) => Ok(Some(serde_json::from_str(vars)?)),
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

fn build_schema_path(
    format: Option<&str>,
    lang: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut query = Vec::new();
    if let Some(format) = format {
        query.push(format!("format={}", format));
    }
    if let Some(lang) = lang {
        query.push(format!("lang={}", lang));
    }
    append_query("schema/", &query)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::env;
    use std::error::Error;
    use std::sync::{Arc, Mutex};

    fn parse_args(args: &[&str]) -> Cli {
        Cli::parse_from(args)
    }

    fn base_args() -> Vec<&'static str> {
        vec![
            "netbox-cli",
            "--url",
            "http://localhost:8000",
            "--token",
            "token",
        ]
    }

    fn env_api_client() -> Result<Option<NetboxApiClient>, Box<dyn Error>> {
        let token = match std::env::var("NETBOX_TOKEN") {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };
        let url =
            std::env::var("NETBOX_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let mut config = ClientConfig::new(url, token).with_max_retries(0);
        if std::env::var("NETBOX_INSECURE").as_deref() == Ok("1") {
            config = config.with_ssl_verification(false);
        }
        let client = Client::new(config)?;
        Ok(Some(NetboxApiClient { inner: client }))
    }

    #[derive(Clone, Debug, PartialEq)]
    struct RecordedCall {
        method: Method,
        path: String,
        body: Option<Value>,
    }

    struct FakeApiClient {
        calls: Arc<Mutex<Vec<RecordedCall>>>,
        next: Arc<Mutex<Value>>,
    }

    impl FakeApiClient {
        fn new(response: Value) -> Self {
            Self {
                calls: Arc::new(Mutex::new(Vec::new())),
                next: Arc::new(Mutex::new(response)),
            }
        }

        fn calls(&self) -> Vec<RecordedCall> {
            self.calls.lock().unwrap().clone()
        }
    }

    struct ErrorApiClient;

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

    fn output_config() -> OutputConfig {
        OutputConfig {
            format: OutputFormat::Json,
            select: None,
            columns: None,
            max_columns: 6,
            dry_run: false,
        }
    }

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
    fn load_json_rejects_invalid_json() {
        let input = JsonInput {
            json: Some("{invalid}".to_string()),
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
    fn load_json_optional_rejects_invalid_json() {
        let input = JsonInputOptional {
            json: Some("{invalid}".to_string()),
            file: None,
        };
        let result: Result<Option<Value>, _> = load_json_optional(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_graphql_query_prefers_inline() {
        let input = GraphqlInput {
            query: Some("{ devices { name } }".to_string()),
            query_file: None,
            vars: None,
        };
        let query = load_graphql_query(&input).unwrap();
        assert_eq!(query, "{ devices { name } }");
    }

    #[test]
    fn load_graphql_query_reads_file() {
        let mut path = env::temp_dir();
        path.push("netbox-cli-graphql.graphql");
        fs::write(&path, "{ devices { name } }").unwrap();

        let input = GraphqlInput {
            query: None,
            query_file: Some(path.clone()),
            vars: None,
        };
        let query = load_graphql_query(&input).unwrap();
        assert_eq!(query, "{ devices { name } }");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_graphql_vars_parses_json() {
        let input = GraphqlInput {
            query: Some("{ devices { name } }".to_string()),
            query_file: None,
            vars: Some(r#"{"limit":5}"#.to_string()),
        };
        let vars = load_graphql_vars(&input).unwrap().unwrap();
        assert_eq!(vars["limit"], 5);
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
    fn append_query_appends_when_query_present() {
        let path = "dcim/devices/?name=leaf-1";
        let query = vec!["limit=5".to_string()];
        let full = append_query(path, &query).unwrap();
        assert_eq!(full, "dcim/devices/?name=leaf-1&limit=5");
    }

    #[test]
    fn parse_query_pairs_rejects_empty_key() {
        let query = vec!["=value".to_string()];
        let result = parse_query_pairs(&query);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_api_path_strips_prefix() {
        assert_eq!(normalize_api_path("api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("/api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("/dcim/devices/"), "dcim/devices/");
    }

    #[test]
    fn resource_path_with_id_appends_trailing_slash() {
        let path = resource_path_with_id("dcim/devices/", 42);
        assert_eq!(path, "dcim/devices/42/");
    }

    #[test]
    fn select_value_handles_arrays() {
        let value = json!({
            "results": [
                {"name": "a"},
                {"name": "b"}
            ]
        });
        let selected = select_value(&value, "results.name");
        assert_eq!(selected, json!(["a", "b"]));
    }

    #[test]
    fn format_table_handles_objects() {
        let value = json!({"name": "leaf-1", "status": "active"});
        let table = format_table(&value, None, 6);
        assert!(table.contains("name"));
        assert!(table.contains("leaf-1"));
    }

    #[test]
    fn dry_run_payload_includes_path_and_body() {
        let payload = dry_run_payload(
            Method::POST,
            "dcim/devices/",
            Some(&json!({"name":"leaf-1"})),
        );
        assert_eq!(payload["method"], "POST");
        assert_eq!(payload["path"], "dcim/devices/");
        assert_eq!(payload["body"]["name"], "leaf-1");
    }

    #[test]
    fn format_netbox_error_includes_status_path_and_request_id() {
        let body = r#"{"request_id":"req-123","detail":"bad"}"#.to_string();
        let err = netbox::Error::ApiError {
            status: 400,
            message: "bad".to_string(),
            body,
        };
        let wrapped = RequestError::new(Method::POST, "dcim/devices/", Box::new(err));
        let message = wrapped.to_string();
        assert!(message.contains("POST"));
        assert!(message.contains("dcim/devices/"));
        assert!(message.contains("status 400"));
        assert!(message.contains("request_id req-123"));
        assert!(message.contains("bad"));
    }

    #[test]
    fn build_schema_path_includes_query() {
        let path = build_schema_path(Some("json"), Some("en")).unwrap();
        assert_eq!(path, "schema/?format=json&lang=en");
    }

    #[test]
    fn format_table_flattens_results() {
        let value = json!({
            "count": 2,
            "next": null,
            "previous": null,
            "results": [
                {"id": 1, "name": "alpha"},
                {"id": 2, "name": "beta"}
            ]
        });
        let table = format_table(&value, None, 6);
        assert!(table.contains("count: 2"));
        assert!(table.contains("alpha"));
        assert!(table.contains("beta"));
    }

    #[test]
    fn format_table_respects_explicit_columns() {
        let value = json!({
            "results": [
                {"id": 1, "name": "alpha", "status": "active", "extra": "ignored"},
                {"id": 2, "name": "beta", "status": "planned", "extra": "also ignored"}
            ]
        });
        let columns = vec!["name".to_string(), "status".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("name"));
        assert!(table.contains("status"));
        assert!(table.contains("alpha"));
        assert!(table.contains("active"));
        assert!(!table.contains("extra"));
        assert!(!table.contains("ignored"));
    }

    #[test]
    fn format_table_respects_explicit_columns_with_empty_results() {
        let value = json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });
        let columns = vec!["id".to_string(), "name".to_string(), "slug".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("id"));
        assert!(table.contains("name"));
        assert!(table.contains("slug"));
        assert!(!table.contains("value"));
    }

    #[test]
    fn format_table_respects_max_columns() {
        let value = json!({
            "results": [
                {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
            ]
        });
        let table = format_table(&value, None, 2);
        // Should only have 2 columns
        let header_line = table.lines().nth(1).unwrap_or("");
        let column_count = header_line
            .split('|')
            .filter(|s| !s.trim().is_empty())
            .count();
        assert_eq!(column_count, 2);
    }

    #[test]
    fn parse_columns_flag() {
        let mut args = base_args();
        args.extend(["--columns", "id,name,status", "status"]);
        let cli = parse_args(&args);
        assert_eq!(
            cli.columns,
            Some(vec![
                "id".to_string(),
                "name".to_string(),
                "status".to_string()
            ])
        );
    }

    #[test]
    fn parse_max_columns_flag() {
        let mut args = base_args();
        args.extend(["--max-columns", "10", "status"]);
        let cli = parse_args(&args);
        assert_eq!(cli.max_columns, 10);
    }

    #[test]
    fn find_resource_path_matches_known_resource() {
        let path = find_resource_path(DCIM_RESOURCES, "devices");
        assert_eq!(path, Some("dcim/devices/"));
        let missing = find_resource_path(DCIM_RESOURCES, "not-a-device");
        assert!(missing.is_none());
    }

    #[test]
    fn parse_resources_command_with_group() {
        let mut args = base_args();
        args.extend(["resources", "dcim"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::Resources { group } => {
                assert_eq!(group.as_deref(), Some("dcim"));
            }
            _ => panic!("expected resources command"),
        }
    }

    #[test]
    fn parse_dcim_list_command_with_query() {
        let mut args = base_args();
        args.extend([
            "dcim",
            "devices",
            "list",
            "--query",
            "name=leaf-1",
            "--query",
            "limit=5",
        ]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::Dcim { resource, action } => {
                assert_eq!(resource, "devices");
                match action {
                    ResourceAction::List { query } => {
                        assert_eq!(query, vec!["name=leaf-1", "limit=5"]);
                    }
                    _ => panic!("expected list action"),
                }
            }
            _ => panic!("expected dcim command"),
        }
    }

    #[test]
    fn parse_raw_command_with_json() {
        let mut args = base_args();
        args.extend([
            "raw",
            "--method",
            "POST",
            "--path",
            "api/dcim/sites/",
            "--query",
            "name=dc1",
            "--json",
            r#"{"name":"dc1"}"#,
        ]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::Raw {
                method,
                path,
                query,
                input,
            } => {
                assert_eq!(method, "POST");
                assert_eq!(path, "api/dcim/sites/");
                assert_eq!(query, vec!["name=dc1"]);
                assert!(input.json.is_some());
                assert!(input.file.is_none());
            }
            _ => panic!("expected raw command"),
        }
    }

    #[test]
    fn parse_dashboard_update_requires_json_or_file() {
        let mut args = base_args();
        args.extend(["extras-dashboard", "update"]);
        let result = Cli::try_parse_from(&args);
        assert!(result.is_err());
    }

    #[test]
    fn parse_branch_action_with_file() {
        let mut args = base_args();
        args.extend([
            "plugin-branch-action",
            "12",
            "merge",
            "--file",
            "payload.json",
        ]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::PluginBranchAction { id, action } => {
                assert_eq!(id, 12u64);
                match action {
                    BranchAction::Merge { input } => {
                        assert!(input.file.is_some());
                    }
                    _ => panic!("expected merge action"),
                }
            }
            _ => panic!("expected plugin-branch-action command"),
        }
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

    #[tokio::test]
    #[ignore]
    async fn smoke_status() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NETBOX_TOKEN not set; skipping smoke_status");
            return Ok(());
        };
        let _ = api.status().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_list_devices() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NETBOX_TOKEN not set; skipping smoke_list_devices");
            return Ok(());
        };
        handle_resource_action(
            &api,
            &output_config(),
            "dcim/devices/",
            ResourceAction::List {
                query: vec!["limit=1".to_string()],
            },
        )
        .await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_output_formats() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NETBOX_TOKEN not set; skipping smoke_output_formats");
            return Ok(());
        };
        let status = api.status().await?;
        for format in [OutputFormat::Json, OutputFormat::Yaml, OutputFormat::Table] {
            let output = OutputConfig {
                format,
                select: None,
                columns: None,
                max_columns: 6,
                dry_run: false,
            };
            let rendered = format_output(&status, &output)?;
            assert!(
                !rendered.trim().is_empty(),
                "expected output for {format:?}"
            );
        }
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_select_output() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NETBOX_TOKEN not set; skipping smoke_select_output");
            return Ok(());
        };
        let status = api.status().await?;
        let output = OutputConfig {
            format: OutputFormat::Json,
            select: Some("netbox-version".to_string()),
            columns: None,
            max_columns: 6,
            dry_run: false,
        };
        let rendered = format_output(&status, &output)?;
        assert!(!rendered.trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_users_config() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NETBOX_TOKEN not set; skipping smoke_users_config");
            return Ok(());
        };
        let _ = api.request_raw(Method::GET, "users/config/", None).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_raw_tag_roundtrip() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NETBOX_TOKEN not set; skipping smoke_raw_tag_roundtrip");
            return Ok(());
        };

        let name = format!(
            "cli-raw-tag-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        let body = json!({
            "name": name,
            "slug": name,
            "color": "9e9e9e",
        });
        let created = api
            .request_raw(Method::POST, "extras/tags/", Some(&body))
            .await?;
        let tag_id = created
            .get("id")
            .and_then(|value| value.as_i64())
            .ok_or("missing tag id")? as u64;
        let path = format!("extras/tags/{}/", tag_id);
        let _ = api.request_raw(Method::DELETE, &path, None).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_resource_crud_tag() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NETBOX_TOKEN not set; skipping smoke_resource_crud_tag");
            return Ok(());
        };

        let name = format!(
            "cli-resource-tag-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        let create = JsonInput {
            json: Some(format!(
                r#"{{"name":"{0}","slug":"{0}","color":"9e9e9e"}}"#,
                name
            )),
            file: None,
        };
        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Create { input: create },
        )
        .await?;

        let list_path = format!("extras/tags/?name={}", name);
        let list = api.request_raw(Method::GET, &list_path, None).await?;
        let tag_id = list
            .get("results")
            .and_then(|value| value.as_array())
            .and_then(|results| results.first())
            .and_then(|value| value.get("id"))
            .and_then(|value| value.as_i64())
            .ok_or("missing tag id")? as u64;

        let update = JsonInput {
            json: Some(format!(
                r#"{{"name":"{0}-updated","slug":"{0}-updated","color":"2196f3"}}"#,
                name
            )),
            file: None,
        };
        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Update {
                id: tag_id,
                input: update,
            },
        )
        .await?;

        let patch = JsonInput {
            json: Some(r#"{"description":"cli smoke test"}"#.to_string()),
            file: None,
        };
        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Patch {
                id: tag_id,
                input: patch,
            },
        )
        .await?;

        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Delete { id: tag_id },
        )
        .await?;
        Ok(())
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

    #[test]
    fn parse_ipam_availability_command() {
        let mut args = base_args();
        args.extend(["ipam-prefix-available-ips", "42", "list"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::IpamPrefixAvailableIps { id, action } => {
                assert_eq!(id, 42);
                assert!(matches!(action, AvailabilityAction::List));
            }
            _ => panic!("expected ipam-prefix-available-ips command"),
        }
    }

    #[test]
    fn parse_core_task_action_command() {
        let mut args = base_args();
        args.extend(["core-task-action", "task-123", "enqueue"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::CoreTaskAction { id, action } => {
                assert_eq!(id, "task-123");
                assert!(matches!(action, TaskAction::Enqueue));
            }
            _ => panic!("expected core-task-action command"),
        }
    }

    #[test]
    fn parse_dcim_trace_command() {
        let mut args = base_args();
        args.extend(["dcim-trace", "interface", "99"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::DcimTrace { resource } => {
                assert!(matches!(resource, TraceableResource::Interface { id: 99 }));
            }
            _ => panic!("expected dcim-trace command"),
        }
    }

    #[test]
    fn parse_extras_sync_command() {
        let mut args = base_args();
        args.extend(["extras-config-template-sync", "5"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::ExtrasConfigTemplateSync { id } => {
                assert_eq!(id, 5);
            }
            _ => panic!("expected extras-config-template-sync command"),
        }
    }

    #[test]
    fn parse_circuits_paths_command() {
        let mut args = base_args();
        args.extend(["circuits-termination-paths", "10"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::CircuitsTerminationPaths { id } => {
                assert_eq!(id, 10);
            }
            _ => panic!("expected circuits-termination-paths command"),
        }
    }

    #[test]
    fn parse_virtualization_render_config_command() {
        let mut args = base_args();
        args.extend(["virtualization-render-config", "33"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::VirtualizationRenderConfig { id } => {
                assert_eq!(id, 33);
            }
            _ => panic!("expected virtualization-render-config command"),
        }
    }

    #[test]
    fn parse_config_list_command() {
        let args = vec![
            "netbox-cli".to_string(),
            "config".to_string(),
            "list".to_string(),
        ];
        let cli = Cli::try_parse_from(&args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Config {
                action: ConfigAction::List
            }
        ));
    }

    #[test]
    fn parse_config_show_command() {
        let args = vec![
            "netbox-cli".to_string(),
            "config".to_string(),
            "show".to_string(),
        ];
        let cli = Cli::try_parse_from(&args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Config {
                action: ConfigAction::Show
            }
        ));
    }

    #[test]
    fn parse_config_validate_command() {
        let args = vec![
            "netbox-cli".to_string(),
            "config".to_string(),
            "validate".to_string(),
        ];
        let cli = Cli::try_parse_from(&args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Config {
                action: ConfigAction::Validate
            }
        ));
    }

    #[test]
    fn parse_profile_flag() {
        let args = vec![
            "netbox-cli".to_string(),
            "--profile".to_string(),
            "prod".to_string(),
            "config".to_string(),
            "show".to_string(),
        ];
        let cli = Cli::try_parse_from(&args).unwrap();
        assert_eq!(cli.profile, "prod");
    }

    #[test]
    fn default_profile_is_default() {
        let args = vec![
            "netbox-cli".to_string(),
            "config".to_string(),
            "list".to_string(),
        ];
        let cli = Cli::try_parse_from(&args).unwrap();
        assert_eq!(cli.profile, "default");
    }
}
