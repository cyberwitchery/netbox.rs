#![doc = include_str!("../../../docs/cli.md")]

use clap::{Args, Parser, Subcommand};
use netbox::circuits::{
    CreateCircuitGroupAssignmentRequest, CreateCircuitRequest, CreateCircuitTerminationRequest,
    CreateCircuitTypeRequest, CreateProviderAccountRequest, CreateProviderNetworkRequest,
    CreateProviderRequest, CreateVirtualCircuitRequest, CreateVirtualCircuitTerminationRequest,
    UpdateCircuitGroupAssignmentRequest, UpdateCircuitRequest, UpdateCircuitTerminationRequest,
    UpdateCircuitTypeRequest, UpdateProviderAccountRequest, UpdateProviderNetworkRequest,
    UpdateProviderRequest, UpdateVirtualCircuitRequest, UpdateVirtualCircuitTerminationRequest,
};
use netbox::plugins::{CommitRequest, PatchedWritableBranchRequest, WritableBranchRequest};
use netbox::users::TokenProvisionRequest;
use netbox::virtualization::{
    CreateClusterRequest, CreateVirtualDiskRequest, CreateVirtualMachineRequest,
    CreateVmInterfaceRequest, UpdateClusterRequest, UpdateVirtualDiskRequest,
    UpdateVirtualMachineRequest, UpdateVmInterfaceRequest,
};
use netbox::vpn::{
    CreateIkePolicyRequest, CreateIkeProposalRequest, CreateIpSecPolicyRequest,
    CreateIpSecProfileRequest, CreateIpSecProposalRequest, CreateL2VpnRequest,
    CreateL2VpnTerminationRequest, CreateTunnelGroupRequest, CreateTunnelRequest,
    CreateTunnelTerminationRequest, UpdateIkePolicyRequest, UpdateIkeProposalRequest,
    UpdateIpSecPolicyRequest, UpdateIpSecProfileRequest, UpdateIpSecProposalRequest,
    UpdateL2VpnRequest, UpdateL2VpnTerminationRequest, UpdateTunnelGroupRequest,
    UpdateTunnelRequest, UpdateTunnelTerminationRequest,
};
use netbox::{Client, ClientConfig};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::{to_string_pretty, Value};
use std::fs;
use std::path::PathBuf;

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
    /// List devices
    ListDevices,
    /// List prefixes
    ListPrefixes,
    /// List IP addresses
    ListIpAddresses,
    /// List circuits
    ListCircuits,
    /// List circuit terminations
    ListCircuitTerminations,
    /// List virtual machines
    ListVirtualMachines,
    /// List VM interfaces
    ListVmInterfaces,
    /// List VPN tunnels
    ListTunnels,
    /// List L2VPNs
    ListL2Vpns,
    /// List IPsec profiles
    ListIpSecProfiles,
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
    /// Make a raw API request (covers all endpoints)
    Raw {
        /// HTTP method (GET, POST, PATCH, PUT, DELETE)
        #[arg(long)]
        method: String,
        /// API path, e.g. "api/dcim/devices/"
        #[arg(long)]
        path: String,
        /// Query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
        #[command(flatten)]
        input: JsonInputOptional,
    },
    /// Create a provider
    CreateProvider {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a provider
    UpdateProvider {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a provider network
    CreateProviderNetwork {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a provider network
    UpdateProviderNetwork {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a provider account
    CreateProviderAccount {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a provider account
    UpdateProviderAccount {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a circuit type
    CreateCircuitType {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a circuit type
    UpdateCircuitType {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a circuit
    CreateCircuit {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a circuit
    UpdateCircuit {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a circuit termination
    CreateCircuitTermination {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a circuit termination
    UpdateCircuitTermination {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a circuit group assignment
    CreateCircuitGroupAssignment {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a circuit group assignment
    UpdateCircuitGroupAssignment {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a virtual circuit
    CreateVirtualCircuit {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a virtual circuit
    UpdateVirtualCircuit {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a virtual circuit termination
    CreateVirtualCircuitTermination {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a virtual circuit termination
    UpdateVirtualCircuitTermination {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a cluster
    CreateCluster {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a cluster
    UpdateCluster {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a virtual machine
    CreateVirtualMachine {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a virtual machine
    UpdateVirtualMachine {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a VM interface
    CreateVmInterface {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a VM interface
    UpdateVmInterface {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a virtual disk
    CreateVirtualDisk {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a virtual disk
    UpdateVirtualDisk {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a tunnel group
    CreateTunnelGroup {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a tunnel group
    UpdateTunnelGroup {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a tunnel
    CreateTunnel {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a tunnel
    UpdateTunnel {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create a tunnel termination
    CreateTunnelTermination {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a tunnel termination
    UpdateTunnelTermination {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create an L2VPN
    CreateL2Vpn {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update an L2VPN
    UpdateL2Vpn {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create an L2VPN termination
    CreateL2VpnTermination {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update an L2VPN termination
    UpdateL2VpnTermination {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create an IKE proposal
    CreateIkeProposal {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update an IKE proposal
    UpdateIkeProposal {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create an IKE policy
    CreateIkePolicy {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update an IKE policy
    UpdateIkePolicy {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create an IPsec proposal
    CreateIpSecProposal {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update an IPsec proposal
    UpdateIpSecProposal {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create an IPsec policy
    CreateIpSecPolicy {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update an IPsec policy
    UpdateIpSecPolicy {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Create an IPsec profile
    CreateIpSecProfile {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update an IPsec profile
    UpdateIpSecProfile {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// List branches (branching plugin)
    ListBranches,
    /// Get a branch (branching plugin)
    GetBranch { id: i32 },
    /// Create a branch (branching plugin)
    CreateBranch {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a branch (branching plugin)
    UpdateBranch {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Delete a branch (branching plugin)
    DeleteBranch { id: i32 },
    /// Merge a branch (branching plugin)
    MergeBranch {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Revert a branch (branching plugin)
    RevertBranch {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Sync a branch (branching plugin)
    SyncBranch {
        id: i32,
        #[command(flatten)]
        input: JsonInput,
    },
    /// List branch events (branching plugin)
    ListBranchEvents,
    /// Get a branch event (branching plugin)
    GetBranchEvent { id: i32 },
    /// List changes (branching plugin)
    ListChanges,
    /// Get a change (branching plugin)
    GetChange { id: i32 },
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
        Commands::ListDevices => {
            println!("Listing devices...");
            let devices = client.dcim().devices().list(None).await?;
            println!("{}", to_string_pretty(&devices)?);
        }
        Commands::ListPrefixes => {
            println!("Listing prefixes...");
            let prefixes = client.ipam().prefixes().list(None).await?;
            println!("{}", to_string_pretty(&prefixes)?);
        }
        Commands::ListIpAddresses => {
            println!("Listing IP addresses...");
            let ips = client.ipam().ip_addresses().list(None).await?;
            println!("{}", to_string_pretty(&ips)?);
        }
        Commands::ListCircuits => {
            println!("Listing circuits...");
            let circuits = client.circuits().circuits().list(None).await?;
            println!("{}", to_string_pretty(&circuits)?);
        }
        Commands::ListCircuitTerminations => {
            println!("Listing circuit terminations...");
            let terminations = client.circuits().circuit_terminations().list(None).await?;
            println!("{}", to_string_pretty(&terminations)?);
        }
        Commands::ListVirtualMachines => {
            println!("Listing virtual machines...");
            let vms = client
                .virtualization()
                .virtual_machines()
                .list(None)
                .await?;
            println!("{}", to_string_pretty(&vms)?);
        }
        Commands::ListVmInterfaces => {
            println!("Listing VM interfaces...");
            let interfaces = client.virtualization().interfaces().list(None).await?;
            println!("{}", to_string_pretty(&interfaces)?);
        }
        Commands::ListTunnels => {
            println!("Listing VPN tunnels...");
            let tunnels = client.vpn().tunnels().list(None).await?;
            println!("{}", to_string_pretty(&tunnels)?);
        }
        Commands::ListL2Vpns => {
            println!("Listing L2VPNs...");
            let l2vpns = client.vpn().l2vpns().list(None).await?;
            println!("{}", to_string_pretty(&l2vpns)?);
        }
        Commands::ListIpSecProfiles => {
            println!("Listing IPsec profiles...");
            let profiles = client.vpn().ipsec_profiles().list(None).await?;
            println!("{}", to_string_pretty(&profiles)?);
        }
        Commands::Status => {
            let status = client.status().status().await?;
            println!("{}", to_string_pretty(&status)?);
        }
        Commands::Schema { format, lang } => {
            let schema = client
                .schema()
                .schema(format.as_deref(), lang.as_deref())
                .await?;
            println!("{}", to_string_pretty(&schema)?);
        }
        Commands::ConnectedDevice {
            peer_device,
            peer_interface,
        } => {
            let devices = client
                .dcim()
                .connected_device(&peer_device, &peer_interface)
                .await?;
            println!("{}", to_string_pretty(&devices)?);
        }
        Commands::ProvisionToken { input } => {
            let request: TokenProvisionRequest = load_json(&input)?;
            let token = client.users().provision_token(&request).await?;
            println!("{}", to_string_pretty(&token)?);
        }
        Commands::Raw {
            method,
            path,
            query,
            input,
        } => {
            let method = Method::from_bytes(method.as_bytes())?;
            let body: Option<Value> = load_json_optional(&input)?;
            let full_path = append_query(&path, &query)?;
            let response = client
                .request_raw(method, &full_path, body.as_ref())
                .await?;
            println!("{}", to_string_pretty(&response)?);
        }
        Commands::CreateProvider { input } => {
            let request: CreateProviderRequest = load_json(&input)?;
            let provider = client.circuits().providers().create(&request).await?;
            println!("{}", to_string_pretty(&provider)?);
        }
        Commands::UpdateProvider { id, input } => {
            let request: UpdateProviderRequest = load_json(&input)?;
            let provider = client.circuits().providers().patch(id, &request).await?;
            println!("{}", to_string_pretty(&provider)?);
        }
        Commands::CreateProviderNetwork { input } => {
            let request: CreateProviderNetworkRequest = load_json(&input)?;
            let network = client
                .circuits()
                .provider_networks()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&network)?);
        }
        Commands::UpdateProviderNetwork { id, input } => {
            let request: UpdateProviderNetworkRequest = load_json(&input)?;
            let network = client
                .circuits()
                .provider_networks()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&network)?);
        }
        Commands::CreateProviderAccount { input } => {
            let request: CreateProviderAccountRequest = load_json(&input)?;
            let account = client
                .circuits()
                .provider_accounts()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&account)?);
        }
        Commands::UpdateProviderAccount { id, input } => {
            let request: UpdateProviderAccountRequest = load_json(&input)?;
            let account = client
                .circuits()
                .provider_accounts()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&account)?);
        }
        Commands::CreateCircuitType { input } => {
            let request: CreateCircuitTypeRequest = load_json(&input)?;
            let circuit_type = client.circuits().circuit_types().create(&request).await?;
            println!("{}", to_string_pretty(&circuit_type)?);
        }
        Commands::UpdateCircuitType { id, input } => {
            let request: UpdateCircuitTypeRequest = load_json(&input)?;
            let circuit_type = client
                .circuits()
                .circuit_types()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&circuit_type)?);
        }
        Commands::CreateCircuit { input } => {
            let request: CreateCircuitRequest = load_json(&input)?;
            let circuit = client.circuits().circuits().create(&request).await?;
            println!("{}", to_string_pretty(&circuit)?);
        }
        Commands::UpdateCircuit { id, input } => {
            let request: UpdateCircuitRequest = load_json(&input)?;
            let circuit = client.circuits().circuits().patch(id, &request).await?;
            println!("{}", to_string_pretty(&circuit)?);
        }
        Commands::CreateCircuitTermination { input } => {
            let request: CreateCircuitTerminationRequest = load_json(&input)?;
            let termination = client
                .circuits()
                .circuit_terminations()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::UpdateCircuitTermination { id, input } => {
            let request: UpdateCircuitTerminationRequest = load_json(&input)?;
            let termination = client
                .circuits()
                .circuit_terminations()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::CreateCircuitGroupAssignment { input } => {
            let request: CreateCircuitGroupAssignmentRequest = load_json(&input)?;
            let assignment = client
                .circuits()
                .circuit_group_assignments()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&assignment)?);
        }
        Commands::UpdateCircuitGroupAssignment { id, input } => {
            let request: UpdateCircuitGroupAssignmentRequest = load_json(&input)?;
            let assignment = client
                .circuits()
                .circuit_group_assignments()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&assignment)?);
        }
        Commands::CreateVirtualCircuit { input } => {
            let request: CreateVirtualCircuitRequest = load_json(&input)?;
            let vc = client
                .circuits()
                .virtual_circuits()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&vc)?);
        }
        Commands::UpdateVirtualCircuit { id, input } => {
            let request: UpdateVirtualCircuitRequest = load_json(&input)?;
            let vc = client
                .circuits()
                .virtual_circuits()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&vc)?);
        }
        Commands::CreateVirtualCircuitTermination { input } => {
            let request: CreateVirtualCircuitTerminationRequest = load_json(&input)?;
            let termination = client
                .circuits()
                .virtual_circuit_terminations()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::UpdateVirtualCircuitTermination { id, input } => {
            let request: UpdateVirtualCircuitTerminationRequest = load_json(&input)?;
            let termination = client
                .circuits()
                .virtual_circuit_terminations()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::CreateCluster { input } => {
            let request: CreateClusterRequest = load_json(&input)?;
            let cluster = client.virtualization().clusters().create(&request).await?;
            println!("{}", to_string_pretty(&cluster)?);
        }
        Commands::UpdateCluster { id, input } => {
            let request: UpdateClusterRequest = load_json(&input)?;
            let cluster = client
                .virtualization()
                .clusters()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&cluster)?);
        }
        Commands::CreateVirtualMachine { input } => {
            let request: CreateVirtualMachineRequest = load_json(&input)?;
            let vm = client
                .virtualization()
                .virtual_machines()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&vm)?);
        }
        Commands::UpdateVirtualMachine { id, input } => {
            let request: UpdateVirtualMachineRequest = load_json(&input)?;
            let vm = client
                .virtualization()
                .virtual_machines()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&vm)?);
        }
        Commands::CreateVmInterface { input } => {
            let request: CreateVmInterfaceRequest = load_json(&input)?;
            let iface = client
                .virtualization()
                .interfaces()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&iface)?);
        }
        Commands::UpdateVmInterface { id, input } => {
            let request: UpdateVmInterfaceRequest = load_json(&input)?;
            let iface = client
                .virtualization()
                .interfaces()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&iface)?);
        }
        Commands::CreateVirtualDisk { input } => {
            let request: CreateVirtualDiskRequest = load_json(&input)?;
            let disk = client
                .virtualization()
                .virtual_disks()
                .create(&request)
                .await?;
            println!("{}", to_string_pretty(&disk)?);
        }
        Commands::UpdateVirtualDisk { id, input } => {
            let request: UpdateVirtualDiskRequest = load_json(&input)?;
            let disk = client
                .virtualization()
                .virtual_disks()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&disk)?);
        }
        Commands::CreateTunnelGroup { input } => {
            let request: CreateTunnelGroupRequest = load_json(&input)?;
            let group = client.vpn().tunnel_groups().create(&request).await?;
            println!("{}", to_string_pretty(&group)?);
        }
        Commands::UpdateTunnelGroup { id, input } => {
            let request: UpdateTunnelGroupRequest = load_json(&input)?;
            let group = client.vpn().tunnel_groups().patch(id, &request).await?;
            println!("{}", to_string_pretty(&group)?);
        }
        Commands::CreateTunnel { input } => {
            let request: CreateTunnelRequest = load_json(&input)?;
            let tunnel = client.vpn().tunnels().create(&request).await?;
            println!("{}", to_string_pretty(&tunnel)?);
        }
        Commands::UpdateTunnel { id, input } => {
            let request: UpdateTunnelRequest = load_json(&input)?;
            let tunnel = client.vpn().tunnels().patch(id, &request).await?;
            println!("{}", to_string_pretty(&tunnel)?);
        }
        Commands::CreateTunnelTermination { input } => {
            let request: CreateTunnelTerminationRequest = load_json(&input)?;
            let termination = client.vpn().tunnel_terminations().create(&request).await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::UpdateTunnelTermination { id, input } => {
            let request: UpdateTunnelTerminationRequest = load_json(&input)?;
            let termination = client
                .vpn()
                .tunnel_terminations()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::CreateL2Vpn { input } => {
            let request: CreateL2VpnRequest = load_json(&input)?;
            let l2vpn = client.vpn().l2vpns().create(&request).await?;
            println!("{}", to_string_pretty(&l2vpn)?);
        }
        Commands::UpdateL2Vpn { id, input } => {
            let request: UpdateL2VpnRequest = load_json(&input)?;
            let l2vpn = client.vpn().l2vpns().patch(id, &request).await?;
            println!("{}", to_string_pretty(&l2vpn)?);
        }
        Commands::CreateL2VpnTermination { input } => {
            let request: CreateL2VpnTerminationRequest = load_json(&input)?;
            let termination = client.vpn().l2vpn_terminations().create(&request).await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::UpdateL2VpnTermination { id, input } => {
            let request: UpdateL2VpnTerminationRequest = load_json(&input)?;
            let termination = client
                .vpn()
                .l2vpn_terminations()
                .patch(id, &request)
                .await?;
            println!("{}", to_string_pretty(&termination)?);
        }
        Commands::CreateIkeProposal { input } => {
            let request: CreateIkeProposalRequest = load_json(&input)?;
            let proposal = client.vpn().ike_proposals().create(&request).await?;
            println!("{}", to_string_pretty(&proposal)?);
        }
        Commands::UpdateIkeProposal { id, input } => {
            let request: UpdateIkeProposalRequest = load_json(&input)?;
            let proposal = client.vpn().ike_proposals().patch(id, &request).await?;
            println!("{}", to_string_pretty(&proposal)?);
        }
        Commands::CreateIkePolicy { input } => {
            let request: CreateIkePolicyRequest = load_json(&input)?;
            let policy = client.vpn().ike_policies().create(&request).await?;
            println!("{}", to_string_pretty(&policy)?);
        }
        Commands::UpdateIkePolicy { id, input } => {
            let request: UpdateIkePolicyRequest = load_json(&input)?;
            let policy = client.vpn().ike_policies().patch(id, &request).await?;
            println!("{}", to_string_pretty(&policy)?);
        }
        Commands::CreateIpSecProposal { input } => {
            let request: CreateIpSecProposalRequest = load_json(&input)?;
            let proposal = client.vpn().ipsec_proposals().create(&request).await?;
            println!("{}", to_string_pretty(&proposal)?);
        }
        Commands::UpdateIpSecProposal { id, input } => {
            let request: UpdateIpSecProposalRequest = load_json(&input)?;
            let proposal = client.vpn().ipsec_proposals().patch(id, &request).await?;
            println!("{}", to_string_pretty(&proposal)?);
        }
        Commands::CreateIpSecPolicy { input } => {
            let request: CreateIpSecPolicyRequest = load_json(&input)?;
            let policy = client.vpn().ipsec_policies().create(&request).await?;
            println!("{}", to_string_pretty(&policy)?);
        }
        Commands::UpdateIpSecPolicy { id, input } => {
            let request: UpdateIpSecPolicyRequest = load_json(&input)?;
            let policy = client.vpn().ipsec_policies().patch(id, &request).await?;
            println!("{}", to_string_pretty(&policy)?);
        }
        Commands::CreateIpSecProfile { input } => {
            let request: CreateIpSecProfileRequest = load_json(&input)?;
            let profile = client.vpn().ipsec_profiles().create(&request).await?;
            println!("{}", to_string_pretty(&profile)?);
        }
        Commands::UpdateIpSecProfile { id, input } => {
            let request: UpdateIpSecProfileRequest = load_json(&input)?;
            let profile = client.vpn().ipsec_profiles().patch(id, &request).await?;
            println!("{}", to_string_pretty(&profile)?);
        }
        Commands::ListBranches => {
            let branches = client.plugins().branches().list(None).await?;
            println!("{}", to_string_pretty(&branches)?);
        }
        Commands::GetBranch { id } => {
            let branch = client.plugins().branches().get(id).await?;
            println!("{}", to_string_pretty(&branch)?);
        }
        Commands::CreateBranch { input } => {
            let request: WritableBranchRequest = load_json(&input)?;
            let branch = client.plugins().branches().create(&request).await?;
            println!("{}", to_string_pretty(&branch)?);
        }
        Commands::UpdateBranch { id, input } => {
            let request: PatchedWritableBranchRequest = load_json(&input)?;
            let branch = client.plugins().branches().patch(id, &request).await?;
            println!("{}", to_string_pretty(&branch)?);
        }
        Commands::DeleteBranch { id } => {
            client.plugins().branches().delete(id).await?;
            println!("Deleted branch {}", id);
        }
        Commands::MergeBranch { id, input } => {
            let request: CommitRequest = load_json(&input)?;
            let job = client.plugins().merge_branch(id, &request).await?;
            println!("{}", to_string_pretty(&job)?);
        }
        Commands::RevertBranch { id, input } => {
            let request: CommitRequest = load_json(&input)?;
            let job = client.plugins().revert_branch(id, &request).await?;
            println!("{}", to_string_pretty(&job)?);
        }
        Commands::SyncBranch { id, input } => {
            let request: CommitRequest = load_json(&input)?;
            let job = client.plugins().sync_branch(id, &request).await?;
            println!("{}", to_string_pretty(&job)?);
        }
        Commands::ListBranchEvents => {
            let events = client.plugins().branch_events().list(None).await?;
            println!("{}", to_string_pretty(&events)?);
        }
        Commands::GetBranchEvent { id } => {
            let event = client.plugins().branch_events().get(id).await?;
            println!("{}", to_string_pretty(&event)?);
        }
        Commands::ListChanges => {
            let changes = client.plugins().changes().list(None).await?;
            println!("{}", to_string_pretty(&changes)?);
        }
        Commands::GetChange { id } => {
            let change = client.plugins().changes().get(id).await?;
            println!("{}", to_string_pretty(&change)?);
        }
    }

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
        let value: CreateProviderRequest = load_json(&input).unwrap();
        assert_eq!(value.name, "carrier");
        assert_eq!(value.slug, "carrier");
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
        let value: CreateProviderRequest = load_json(&input).unwrap();
        assert_eq!(value.name, "carrier");
        assert_eq!(value.slug, "carrier");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_json_requires_input() {
        let input = JsonInput {
            json: None,
            file: None,
        };
        let result: Result<CreateProviderRequest, _> = load_json(&input);
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
        let path = "api/dcim/devices/";
        let query = vec!["name=leaf 1".to_string(), "limit=5".to_string()];
        let full = append_query(path, &query).unwrap();
        assert_eq!(full, "api/dcim/devices/?name=leaf+1&limit=5");
    }

    #[test]
    fn append_query_rejects_missing_value() {
        let path = "api/dcim/devices/";
        let query = vec!["name".to_string()];
        let result = append_query(path, &query);
        assert!(result.is_err());
    }
}
