#![doc = include_str!("../docs/cli.md")]

mod cli;
mod config;

use clap::{Args, Parser, Subcommand, ValueEnum};
use cli::{
    CIRCUITS_RESOURCES, CORE_RESOURCES, DCIM_RESOURCES, EXTRAS_RESOURCES, IPAM_RESOURCES,
    PLUGINS_RESOURCES, TENANCY_RESOURCES, USERS_RESOURCES, VIRTUALIZATION_RESOURCES, VPN_RESOURCES,
    WIRELESS_RESOURCES, append_query, build_schema_path, handle_availability_action,
    handle_branch_action, handle_config_command, handle_dashboard_action, handle_get_action,
    handle_named_lookup, handle_resource_group, handle_sync_action, handle_task_action,
    handle_trace_action, load_graphql_query, load_graphql_vars, load_json, load_json_optional,
    normalize_api_path, print_dry_run, print_output, print_resources, request_raw_with_context,
    wrap_request_error,
};
use config::{Profile, load_config};
use netbox::{Client, ClientConfig};
use reqwest::Method;
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

#[async_trait::async_trait]
pub(crate) trait ApiClient {
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
pub(crate) enum OutputFormat {
    Json,
    Yaml,
    Table,
}

#[derive(Clone, Debug)]
pub(crate) struct OutputConfig {
    pub(crate) format: OutputFormat,
    pub(crate) select: Option<String>,
    pub(crate) columns: Option<Vec<String>>,
    pub(crate) max_columns: usize,
    pub(crate) dry_run: bool,
}

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

    /// config profile to use (default: "default")
    #[arg(short, long, default_value = "default")]
    profile: String,

    /// output format (json, yaml, table)
    #[arg(long, value_enum)]
    output: Option<OutputFormat>,

    /// select a field from the response (dot path)
    #[arg(long)]
    select: Option<String>,

    /// columns to show in table output (comma-separated)
    #[arg(long, value_delimiter = ',')]
    columns: Option<Vec<String>>,

    /// maximum columns in table output (default: 6)
    #[arg(long, default_value = "6")]
    max_columns: usize,

    /// print the request and skip write operations
    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum ConfigAction {
    /// show the resolved configuration for a profile
    Show,
    /// list all available profiles
    List,
    /// validate a profile configuration
    Validate,
    /// show the config file path
    Path,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// manage configuration profiles
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// list resources by group (or all resources)
    Resources {
        /// resource group name (dcim, ipam, circuits, tenancy, extras, core, users, virtualization, vpn, wireless, plugins)
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
    /// circuits resources (providers, circuits, ...)
    Circuits {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// tenancy resources (tenants, contacts, ...)
    Tenancy {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// extras resources (tags, scripts, custom fields, ...)
    Extras {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// core resources (jobs, object changes, ...)
    Core {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// users resources (users, groups, tokens, ...)
    Users {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// virtualization resources (clusters, vms, ...)
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
    /// wireless resources (lans, links, ...)
    Wireless {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// plugin resources (branching data)
    Plugins {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// extras dashboard operations
    ExtrasDashboard {
        #[command(subcommand)]
        action: DashboardAction,
    },
    /// core background queue summaries
    CoreBackgroundQueues {
        #[command(subcommand)]
        action: NamedLookupAction,
    },
    /// core background worker summaries
    CoreBackgroundWorkers {
        #[command(subcommand)]
        action: NamedLookupAction,
    },
    /// fetch current user config
    UsersConfig,
    /// fetch NetBox status
    Status,
    /// fetch OpenAPI schema
    Schema {
        /// schema format (json, yaml)
        #[arg(long)]
        format: Option<String>,
        /// schema language
        #[arg(long)]
        lang: Option<String>,
    },
    /// run a read-only graphql query
    Graphql {
        #[command(flatten)]
        input: GraphqlInput,
    },
    /// find a device connected to a peer device/interface
    ConnectedDevice {
        /// peer device name
        #[arg(long)]
        peer_device: String,
        /// peer interface name
        #[arg(long)]
        peer_interface: String,
    },
    /// provision a token with username/password
    ProvisionToken {
        #[command(flatten)]
        input: JsonInput,
    },
    /// branch actions (branching plugin)
    PluginBranchAction {
        id: u64,
        #[command(subcommand)]
        action: BranchAction,
    },
    /// list or create available IPs in a prefix
    IpamPrefixAvailableIps {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// list or create available prefixes in a prefix
    IpamPrefixAvailablePrefixes {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// list or create available IPs in an IP range
    IpamRangeAvailableIps {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// list or create available VLANs in a VLAN group
    IpamVlanGroupAvailableVlans {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// list or create available ASNs in an ASN range
    IpamAsnRangeAvailableAsns {
        id: u64,
        #[command(subcommand)]
        action: AvailabilityAction,
    },
    /// background task actions
    CoreTaskAction {
        id: String,
        #[command(subcommand)]
        action: TaskAction,
    },
    /// sync a data source
    CoreDataSourceSync { id: u64 },
    /// sync a config context
    ExtrasConfigContextSync { id: u64 },
    /// sync a config context profile
    ExtrasConfigContextProfileSync { id: u64 },
    /// sync a config template
    ExtrasConfigTemplateSync { id: u64 },
    /// render a config template
    ExtrasConfigTemplateRender { id: u64 },
    /// sync an export template
    ExtrasExportTemplateSync { id: u64 },
    /// get custom field choices
    ExtrasCustomFieldChoices { id: u64 },
    /// get circuit termination paths
    CircuitsTerminationPaths { id: u64 },
    /// get virtual circuit termination paths
    CircuitsVirtualTerminationPaths { id: u64 },
    /// trace DCIM resources (interfaces, ports, feeds)
    DcimTrace {
        #[command(subcommand)]
        resource: TraceableResource,
    },
    /// render VM config
    VirtualizationRenderConfig { id: u64 },
    /// make a raw API request (covers all endpoints)
    Raw {
        /// HTTP method (GET, POST, PATCH, PUT, DELETE)
        #[arg(long)]
        method: String,
        /// API path, e.g. "dcim/devices/"
        #[arg(long)]
        path: String,
        /// query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
        #[command(flatten)]
        input: JsonInputOptional,
    },
}

#[derive(Subcommand)]
pub(crate) enum ResourceAction {
    /// list resources
    List {
        /// query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
    },
    /// get a resource by id
    Get { id: u64 },
    /// create a resource
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    /// update a resource (PUT)
    Update {
        id: u64,
        #[command(flatten)]
        input: JsonInput,
    },
    /// patch a resource
    Patch {
        id: u64,
        #[command(flatten)]
        input: JsonInput,
    },
    /// delete a resource
    Delete { id: u64 },
}

#[derive(Subcommand)]
pub(crate) enum DashboardAction {
    /// fetch the dashboard config
    Get,
    /// update the dashboard config (PUT)
    Update {
        #[command(flatten)]
        input: JsonInput,
    },
    /// patch the dashboard config
    Patch {
        #[command(flatten)]
        input: JsonInput,
    },
    /// delete the dashboard config
    Delete,
}

#[derive(Subcommand)]
pub(crate) enum NamedLookupAction {
    /// list summaries
    List,
    /// get a summary by name
    Get { name: String },
}

#[derive(Subcommand)]
pub(crate) enum BranchAction {
    /// merge a branch
    Merge {
        #[command(flatten)]
        input: JsonInput,
    },
    /// revert a branch
    Revert {
        #[command(flatten)]
        input: JsonInput,
    },
    /// sync a branch
    Sync {
        #[command(flatten)]
        input: JsonInput,
    },
}

#[derive(Subcommand)]
pub(crate) enum AvailabilityAction {
    /// list available resources
    List,
    /// create resources from available pool
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
}

#[derive(Subcommand)]
pub(crate) enum TaskAction {
    /// enqueue a background task
    Enqueue,
    /// stop a background task
    Stop,
    /// requeue a background task
    Requeue,
    /// delete a background task
    Delete,
}

#[derive(Subcommand)]
pub(crate) enum TraceableResource {
    /// trace an interface
    Interface { id: u64 },
    /// trace a console port
    ConsolePort { id: u64 },
    /// trace a console server port
    ConsoleServerPort { id: u64 },
    /// trace a power port
    PowerPort { id: u64 },
    /// trace a power outlet
    PowerOutlet { id: u64 },
    /// trace a power feed
    PowerFeed { id: u64 },
}

#[derive(Args, Debug)]
pub(crate) struct JsonInput {
    /// JSON payload string
    #[arg(long, required_unless_present = "file")]
    pub(crate) json: Option<String>,
    /// JSON payload file path
    #[arg(long, required_unless_present = "json")]
    pub(crate) file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub(crate) struct JsonInputOptional {
    /// JSON payload string
    #[arg(long)]
    pub(crate) json: Option<String>,
    /// JSON payload file path
    #[arg(long)]
    pub(crate) file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub(crate) struct GraphqlInput {
    /// GraphQL query string
    #[arg(long, required_unless_present = "query_file")]
    pub(crate) query: Option<String>,
    /// GraphQL query file path
    #[arg(long, required_unless_present = "query")]
    pub(crate) query_file: Option<PathBuf>,
    /// JSON variables payload
    #[arg(long)]
    pub(crate) vars: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // load config file
    let config_file = load_config().ok().flatten();

    // handle config commands first (no API access needed)
    if let Commands::Config { action } = &cli.command {
        return handle_config_command(action, &cli.profile, config_file.as_ref());
    }

    // resolve profile from config file
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

    // resolve URL and token
    let url = profile
        .url
        .clone()
        .ok_or("url not specified (use --url, NETBOX_URL, or config file)")?;
    let token = profile.resolve_token()?.ok_or(
        "token not specified (use --token, NETBOX_TOKEN, token_env, or token_command in config)",
    )?;

    // build client config
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

    // resolve output format
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

#[cfg(test)]
mod tests {
    use super::*;
    use cli::test_util::*;
    use serde_json::json;
    use std::error::Error;

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
        cli::handle_resource_action(
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
            let rendered = cli::format_output(&status, &output)?;
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
        let rendered = cli::format_output(&status, &output)?;
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
        cli::handle_resource_action(
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
        cli::handle_resource_action(
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
        cli::handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Patch {
                id: tag_id,
                input: patch,
            },
        )
        .await?;

        cli::handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Delete { id: tag_id },
        )
        .await?;
        Ok(())
    }
}
