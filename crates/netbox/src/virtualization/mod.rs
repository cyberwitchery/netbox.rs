//! Virtualization API endpoints.

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request for creating a cluster (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClusterRequest {
    /// Cluster name.
    pub name: String,
    /// Cluster type ID.
    pub r#type: i32,
    /// Cluster group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a cluster (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateClusterRequest {
    /// Updated cluster name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated cluster type ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Updated cluster group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Updated scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Updated scope object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a virtual machine (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualMachineRequest {
    /// Virtual machine name.
    pub name: String,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// Cluster ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
    /// Device ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,
    /// Serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Role ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Platform ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<i32>,
    /// Primary IPv4 address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<i32>,
    /// Primary IPv6 address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<i32>,
    /// vCPU count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<f64>,
    /// Memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    /// Disk in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Config template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_template: Option<i32>,
    /// Local config context data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Value>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a virtual machine (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualMachineRequest {
    /// Updated virtual machine name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// Updated cluster ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
    /// Updated device ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,
    /// Updated serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Updated role ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Updated platform ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<i32>,
    /// Updated primary IPv4 address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<i32>,
    /// Updated primary IPv6 address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<i32>,
    /// Updated vCPU count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<f64>,
    /// Updated memory in MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    /// Updated disk in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated config template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_template: Option<i32>,
    /// Updated local config context data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Value>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a VM interface (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVmInterfaceRequest {
    /// Virtual machine ID.
    pub virtual_machine: i32,
    /// Interface name.
    pub name: String,
    /// Enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Parent interface ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// Bridge interface ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<i32>,
    /// MTU value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    /// Primary MAC address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_mac_address: Option<i32>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Mode slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Untagged VLAN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<i32>,
    /// Tagged VLAN IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    /// Q-in-Q SVLAN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qinq_svlan: Option<i32>,
    /// VLAN translation policy ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan_translation_policy: Option<i32>,
    /// VRF ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a VM interface (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVmInterfaceRequest {
    /// Updated virtual machine ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<i32>,
    /// Updated interface name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Updated parent interface ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// Updated bridge interface ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<i32>,
    /// Updated MTU value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    /// Updated primary MAC address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_mac_address: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated mode slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Updated untagged VLAN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<i32>,
    /// Updated tagged VLAN IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    /// Updated Q-in-Q SVLAN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qinq_svlan: Option<i32>,
    /// Updated VLAN translation policy ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan_translation_policy: Option<i32>,
    /// Updated VRF ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a virtual disk (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualDiskRequest {
    /// Virtual machine ID.
    pub virtual_machine: i32,
    /// Disk name.
    pub name: String,
    /// Disk size in GB.
    pub size: i32,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a virtual disk (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualDiskRequest {
    /// Updated virtual machine ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<i32>,
    /// Updated disk name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated disk size in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated tag IDs.
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

/// Cluster group model.
pub type ClusterGroup = crate::models::ClusterGroup;
/// Cluster type model.
pub type ClusterType = crate::models::ClusterType;
/// Cluster model.
pub type Cluster = crate::models::Cluster;
/// VM interface model.
pub type VmInterface = crate::models::VmInterface;
/// Virtual disk model.
pub type VirtualDisk = crate::models::VirtualDisk;
/// Virtual machine model with config context.
pub type VirtualMachine = crate::models::VirtualMachineWithConfigContext;

/// Resource for cluster groups.
pub type ClusterGroupsApi = Resource<crate::models::ClusterGroup>;
/// Resource for cluster types.
pub type ClusterTypesApi = Resource<crate::models::ClusterType>;
/// Resource for clusters.
pub type ClustersApi = Resource<crate::models::Cluster>;
/// Resource for VM interfaces.
pub type VmInterfacesApi = Resource<crate::models::VmInterface>;
/// Resource for virtual disks.
pub type VirtualDisksApi = Resource<crate::models::VirtualDisk>;
/// Resource for virtual machines.
pub type VirtualMachinesApi = Resource<crate::models::VirtualMachineWithConfigContext>;

/// API for virtualization endpoints.
#[derive(Clone)]
pub struct VirtualizationApi {
    client: Client,
}

impl VirtualizationApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the cluster groups resource.
    pub fn cluster_groups(&self) -> ClusterGroupsApi {
        Resource::new(self.client.clone(), "virtualization/cluster-groups/")
    }

    /// Returns the cluster types resource.
    pub fn cluster_types(&self) -> ClusterTypesApi {
        Resource::new(self.client.clone(), "virtualization/cluster-types/")
    }

    /// Returns the clusters resource.
    pub fn clusters(&self) -> ClustersApi {
        Resource::new(self.client.clone(), "virtualization/clusters/")
    }

    /// Returns the VM interfaces resource.
    pub fn interfaces(&self) -> VmInterfacesApi {
        Resource::new(self.client.clone(), "virtualization/interfaces/")
    }

    /// Returns the virtual disks resource.
    pub fn virtual_disks(&self) -> VirtualDisksApi {
        Resource::new(self.client.clone(), "virtualization/virtual-disks/")
    }

    /// Returns the virtual machines resource.
    pub fn virtual_machines(&self) -> VirtualMachinesApi {
        Resource::new(self.client.clone(), "virtualization/virtual-machines/")
    }
}
