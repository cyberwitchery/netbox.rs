//! virtualization endpoints for clusters, vms, and interfaces.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let vms = client.virtualization().virtual_machines().list(None).await?;
//! println!("{}", vms.count);
//! # Ok(())
//! # }
//! ```

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// request for creating a cluster (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClusterRequest {
    /// cluster name.
    pub name: String,
    /// cluster type id.
    pub r#type: i32,
    /// cluster group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// scope object id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a cluster (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateClusterRequest {
    /// updated cluster name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated cluster type id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// updated cluster group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// updated tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// updated scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// updated scope object id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for creating a virtual machine (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualMachineRequest {
    /// virtual machine name.
    pub name: String,
    /// status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// site id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// cluster id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
    /// device id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,
    /// serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// role id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// platform id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<i32>,
    /// primary IPv4 address id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<i32>,
    /// primary IPv6 address id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<i32>,
    /// vCPU count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<f64>,
    /// memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    /// disk in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// config template id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_template: Option<i32>,
    /// local config context data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Value>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a virtual machine (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualMachineRequest {
    /// updated virtual machine name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// updated site id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// updated cluster id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
    /// updated device id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,
    /// updated serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// updated role id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// updated tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// updated platform id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<i32>,
    /// updated primary IPv4 address id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<i32>,
    /// updated primary IPv6 address id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<i32>,
    /// updated vCPU count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<f64>,
    /// updated memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    /// updated disk in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// updated config template id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_template: Option<i32>,
    /// updated local config context data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Value>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for creating a VM interface (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVmInterfaceRequest {
    /// virtual machine id.
    pub virtual_machine: i32,
    /// interface name.
    pub name: String,
    /// enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// parent interface id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// bridge interface id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<i32>,
    /// mTU value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    /// primary MAC address id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_mac_address: Option<i32>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// mode slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// untagged VLAN id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<i32>,
    /// tagged VLAN IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    /// q-in-Q SVLAN id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qinq_svlan: Option<i32>,
    /// vLAN translation policy id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan_translation_policy: Option<i32>,
    /// vRF id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a VM interface (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVmInterfaceRequest {
    /// updated virtual machine id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<i32>,
    /// updated interface name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// updated parent interface id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// updated bridge interface id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<i32>,
    /// updated MTU value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    /// updated primary MAC address id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_mac_address: Option<i32>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated mode slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// updated untagged VLAN id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<i32>,
    /// updated tagged VLAN IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    /// updated Q-in-Q SVLAN id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qinq_svlan: Option<i32>,
    /// updated VLAN translation policy id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan_translation_policy: Option<i32>,
    /// updated VRF id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for creating a virtual disk (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualDiskRequest {
    /// virtual machine id.
    pub virtual_machine: i32,
    /// disk name.
    pub name: String,
    /// disk size in GB.
    pub size: i32,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a virtual disk (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualDiskRequest {
    /// updated virtual machine id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<i32>,
    /// updated disk name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated disk size in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde_json::Value;

    fn assert_missing(value: &Value, key: &str) {
        assert!(value.get(key).is_none(), "expected {} to be omitted", key);
    }

    fn assert_optional_i32(value: &Value, key: &str, field: &Option<i32>) {
        match field {
            Some(v) => assert_eq!(value[key], serde_json::json!(*v)),
            None => assert_missing(value, key),
        }
    }

    fn assert_optional_string(value: &Value, key: &str, field: &Option<String>) {
        match field {
            Some(v) => assert_eq!(value[key], serde_json::json!(v)),
            None => assert_missing(value, key),
        }
    }

    #[test]
    fn serialize_cluster_requests() {
        let create = CreateClusterRequest {
            name: "cluster".to_string(),
            r#type: 1,
            group: None,
            status: Some("active".to_string()),
            tenant: None,
            scope_type: None,
            scope_id: None,
            description: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "cluster");
        assert_eq!(value["type"], 1);
        assert_eq!(value["status"], "active");
        assert_missing(&value, "group");

        let update = UpdateClusterRequest {
            name: None,
            r#type: None,
            group: Some(2),
            status: None,
            tenant: None,
            scope_type: None,
            scope_id: None,
            description: Some("Updated".to_string()),
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["group"], 2);
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_virtual_machine_requests() {
        let create = CreateVirtualMachineRequest {
            name: "vm".to_string(),
            status: Some("active".to_string()),
            site: None,
            cluster: Some(1),
            device: None,
            serial: None,
            role: None,
            tenant: None,
            platform: None,
            primary_ip4: None,
            primary_ip6: None,
            vcpus: Some(2.0),
            memory: Some(2048),
            disk: Some(40),
            description: None,
            comments: None,
            config_template: None,
            local_context_data: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "vm");
        assert_eq!(value["status"], "active");
        assert_eq!(value["cluster"], 1);
        assert_eq!(value["vcpus"], 2.0);
        assert_eq!(value["memory"], 2048);
        assert_eq!(value["disk"], 40);
        assert_missing(&value, "site");

        let update = UpdateVirtualMachineRequest {
            name: None,
            status: None,
            site: None,
            cluster: None,
            device: None,
            serial: None,
            role: None,
            tenant: None,
            platform: None,
            primary_ip4: None,
            primary_ip6: None,
            vcpus: None,
            memory: None,
            disk: None,
            description: Some("Updated".to_string()),
            comments: None,
            config_template: None,
            local_context_data: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_vm_interface_requests() {
        let create = CreateVmInterfaceRequest {
            virtual_machine: 1,
            name: "eth0".to_string(),
            enabled: Some(true),
            parent: None,
            bridge: None,
            mtu: None,
            primary_mac_address: None,
            description: None,
            mode: None,
            untagged_vlan: None,
            tagged_vlans: None,
            qinq_svlan: None,
            vlan_translation_policy: None,
            vrf: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["virtual_machine"], 1);
        assert_eq!(value["name"], "eth0");
        assert_eq!(value["enabled"], true);
        assert_missing(&value, "parent");

        let update = UpdateVmInterfaceRequest {
            virtual_machine: None,
            name: None,
            enabled: None,
            parent: None,
            bridge: None,
            mtu: Some(1500),
            primary_mac_address: None,
            description: Some("Updated".to_string()),
            mode: None,
            untagged_vlan: None,
            tagged_vlans: None,
            qinq_svlan: None,
            vlan_translation_policy: None,
            vrf: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["mtu"], 1500);
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_virtual_disk_requests() {
        let create = CreateVirtualDiskRequest {
            virtual_machine: 1,
            name: "root".to_string(),
            size: 100,
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["virtual_machine"], 1);
        assert_eq!(value["name"], "root");
        assert_eq!(value["size"], 100);
        assert_missing(&value, "description");

        let update = UpdateVirtualDiskRequest {
            virtual_machine: None,
            name: None,
            size: Some(200),
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["size"], 200);
        assert_missing(&value, "name");
    }

    proptest! {
        #[test]
        fn prop_virtual_machine_optional_fields(
            status in proptest::option::of("[a-z0-9-]{1,12}"),
            memory in proptest::option::of(0i32..65536),
        ) {
            let request = CreateVirtualMachineRequest {
                name: "vm".to_string(),
                status: status.clone(),
                site: None,
                cluster: None,
                device: None,
                serial: None,
                role: None,
                tenant: None,
                platform: None,
                primary_ip4: None,
                primary_ip6: None,
                vcpus: None,
                memory,
                disk: None,
                description: None,
                comments: None,
                config_template: None,
                local_context_data: None,
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_string(&value, "status", &status);
            assert_optional_i32(&value, "memory", &memory);
        }
    }

    proptest! {
        #[test]
        fn prop_vm_interface_optional_fields(
            mtu in proptest::option::of(0i32..9000),
            mode in proptest::option::of("[a-z0-9-]{1,12}"),
        ) {
            let request = CreateVmInterfaceRequest {
                virtual_machine: 1,
                name: "eth0".to_string(),
                enabled: None,
                parent: None,
                bridge: None,
                mtu,
                primary_mac_address: None,
                description: None,
                mode: mode.clone(),
                untagged_vlan: None,
                tagged_vlans: None,
                qinq_svlan: None,
                vlan_translation_policy: None,
                vrf: None,
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_i32(&value, "mtu", &mtu);
            assert_optional_string(&value, "mode", &mode);
        }
    }

    proptest! {
        #[test]
        fn prop_virtual_disk_optional_fields(
            description in proptest::option::of("[a-z0-9-]{0,16}"),
        ) {
            let request = CreateVirtualDiskRequest {
                virtual_machine: 1,
                name: "root".to_string(),
                size: 10,
                description: description.clone(),
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_string(&value, "description", &description);
        }
    }
}

/// cluster group model.
pub type ClusterGroup = crate::models::ClusterGroup;
/// cluster type model.
pub type ClusterType = crate::models::ClusterType;
/// cluster model.
pub type Cluster = crate::models::Cluster;
/// vM interface model.
pub type VmInterface = crate::models::VmInterface;
/// virtual disk model.
pub type VirtualDisk = crate::models::VirtualDisk;
/// virtual machine model with config context.
pub type VirtualMachine = crate::models::VirtualMachineWithConfigContext;

/// resource for cluster groups.
pub type ClusterGroupsApi = Resource<crate::models::ClusterGroup>;
/// resource for cluster types.
pub type ClusterTypesApi = Resource<crate::models::ClusterType>;
/// resource for clusters.
pub type ClustersApi = Resource<crate::models::Cluster>;
/// resource for VM interfaces.
pub type VmInterfacesApi = Resource<crate::models::VmInterface>;
/// resource for virtual disks.
pub type VirtualDisksApi = Resource<crate::models::VirtualDisk>;
/// resource for virtual machines.
pub type VirtualMachinesApi = Resource<crate::models::VirtualMachineWithConfigContext>;

/// api for virtualization endpoints.
#[derive(Clone)]
pub struct VirtualizationApi {
    client: Client,
}

impl VirtualizationApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the cluster groups resource.
    pub fn cluster_groups(&self) -> ClusterGroupsApi {
        Resource::new(self.client.clone(), "virtualization/cluster-groups/")
    }

    /// returns the cluster types resource.
    pub fn cluster_types(&self) -> ClusterTypesApi {
        Resource::new(self.client.clone(), "virtualization/cluster-types/")
    }

    /// returns the clusters resource.
    pub fn clusters(&self) -> ClustersApi {
        Resource::new(self.client.clone(), "virtualization/clusters/")
    }

    /// returns the VM interfaces resource.
    pub fn interfaces(&self) -> VmInterfacesApi {
        Resource::new(self.client.clone(), "virtualization/interfaces/")
    }

    /// returns the virtual disks resource.
    pub fn virtual_disks(&self) -> VirtualDisksApi {
        Resource::new(self.client.clone(), "virtualization/virtual-disks/")
    }

    /// returns the virtual machines resource.
    pub fn virtual_machines(&self) -> VirtualMachinesApi {
        Resource::new(self.client.clone(), "virtualization/virtual-machines/")
    }
}
