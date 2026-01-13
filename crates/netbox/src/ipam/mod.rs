//! IPAM (IP Address Management) API endpoints

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};

/// Request for creating a new IP address (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpAddressRequest {
    /// CIDR address string.
    pub address: String,
    /// VRF ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Role slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Assigned object type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_object_type: Option<String>,
    /// Assigned object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_object_id: Option<i32>,
    /// DNS name value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an IP address (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIpAddressRequest {
    /// Updated CIDR address string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated DNS name value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request for creating a new prefix (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrefixRequest {
    /// CIDR prefix string.
    pub prefix: String,
    /// Site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// VRF ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// VLAN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Role ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// Pool flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pool: Option<bool>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a prefix (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePrefixRequest {
    /// Updated CIDR prefix string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Updated site ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// IP address model.
pub type IpAddress = crate::models::IpAddress;
/// Prefix model.
pub type Prefix = crate::models::Prefix;

/// Resource for aggregates.
pub type AggregatesApi = Resource<crate::models::Aggregate>;
/// Resource for ASN ranges.
pub type AsnRangesApi = Resource<crate::models::AsnRange>;
/// Resource for ASNs.
pub type AsnsApi = Resource<crate::models::Asn>;
/// Resource for FHRP group assignments.
pub type FhrpGroupAssignmentsApi = Resource<crate::models::FhrpGroupAssignment>;
/// Resource for FHRP groups.
pub type FhrpGroupsApi = Resource<crate::models::FhrpGroup>;
/// Resource for IP addresses.
pub type IpAddressesApi = Resource<crate::models::IpAddress>;
/// Resource for IP ranges.
pub type IpRangesApi = Resource<crate::models::IpRange>;
/// Resource for prefixes.
pub type PrefixesApi = Resource<crate::models::Prefix>;
/// Resource for RIRs.
pub type RirsApi = Resource<crate::models::Rir>;
/// Resource for roles.
pub type RolesApi = Resource<crate::models::Role>;
/// Resource for route targets.
pub type RouteTargetsApi = Resource<crate::models::RouteTarget>;
/// Resource for service templates.
pub type ServiceTemplatesApi = Resource<crate::models::ServiceTemplate>;
/// Resource for services.
pub type ServicesApi = Resource<crate::models::Service>;
/// Resource for VLAN groups.
pub type VlanGroupsApi = Resource<crate::models::VlanGroup>;
/// Resource for VLAN translation policies.
pub type VlanTranslationPoliciesApi = Resource<crate::models::VlanTranslationPolicy>;
/// Resource for VLAN translation rules.
pub type VlanTranslationRulesApi = Resource<crate::models::VlanTranslationRule>;
/// Resource for VLANs.
pub type VlansApi = Resource<crate::models::Vlan>;
/// Resource for VRFs.
pub type VrfsApi = Resource<crate::models::Vrf>;

/// API for IPAM endpoints
#[derive(Clone)]
pub struct IpamApi {
    client: Client,
}

impl IpamApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the aggregates resource.
    pub fn aggregates(&self) -> AggregatesApi {
        Resource::new(self.client.clone(), "ipam/aggregates/")
    }

    /// Returns the ASN ranges resource.
    pub fn asn_ranges(&self) -> AsnRangesApi {
        Resource::new(self.client.clone(), "ipam/asn-ranges/")
    }

    /// Returns the ASNs resource.
    pub fn asns(&self) -> AsnsApi {
        Resource::new(self.client.clone(), "ipam/asns/")
    }

    /// Returns the FHRP group assignments resource.
    pub fn fhrp_group_assignments(&self) -> FhrpGroupAssignmentsApi {
        Resource::new(self.client.clone(), "ipam/fhrp-group-assignments/")
    }

    /// Returns the FHRP groups resource.
    pub fn fhrp_groups(&self) -> FhrpGroupsApi {
        Resource::new(self.client.clone(), "ipam/fhrp-groups/")
    }

    /// Returns the IP addresses resource.
    pub fn ip_addresses(&self) -> IpAddressesApi {
        Resource::new(self.client.clone(), "ipam/ip-addresses/")
    }

    /// Returns the IP ranges resource.
    pub fn ip_ranges(&self) -> IpRangesApi {
        Resource::new(self.client.clone(), "ipam/ip-ranges/")
    }

    /// Returns the prefixes resource.
    pub fn prefixes(&self) -> PrefixesApi {
        Resource::new(self.client.clone(), "ipam/prefixes/")
    }

    /// Returns the RIRs resource.
    pub fn rirs(&self) -> RirsApi {
        Resource::new(self.client.clone(), "ipam/rirs/")
    }

    /// Returns the roles resource.
    pub fn roles(&self) -> RolesApi {
        Resource::new(self.client.clone(), "ipam/roles/")
    }

    /// Returns the route targets resource.
    pub fn route_targets(&self) -> RouteTargetsApi {
        Resource::new(self.client.clone(), "ipam/route-targets/")
    }

    /// Returns the service templates resource.
    pub fn service_templates(&self) -> ServiceTemplatesApi {
        Resource::new(self.client.clone(), "ipam/service-templates/")
    }

    /// Returns the services resource.
    pub fn services(&self) -> ServicesApi {
        Resource::new(self.client.clone(), "ipam/services/")
    }

    /// Returns the VLAN groups resource.
    pub fn vlan_groups(&self) -> VlanGroupsApi {
        Resource::new(self.client.clone(), "ipam/vlan-groups/")
    }

    /// Returns the VLAN translation policies resource.
    pub fn vlan_translation_policies(&self) -> VlanTranslationPoliciesApi {
        Resource::new(self.client.clone(), "ipam/vlan-translation-policies/")
    }

    /// Returns the VLAN translation rules resource.
    pub fn vlan_translation_rules(&self) -> VlanTranslationRulesApi {
        Resource::new(self.client.clone(), "ipam/vlan-translation-rules/")
    }

    /// Returns the VLANs resource.
    pub fn vlans(&self) -> VlansApi {
        Resource::new(self.client.clone(), "ipam/vlans/")
    }

    /// Returns the VRFs resource.
    pub fn vrfs(&self) -> VrfsApi {
        Resource::new(self.client.clone(), "ipam/vrfs/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://netbox.example.com", "token");
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
    fn ipam_accessors_return_expected_paths() {
        let api = IpamApi::new(test_client());

        assert_path(api.aggregates(), "ipam/aggregates/");
        assert_path(api.asn_ranges(), "ipam/asn-ranges/");
        assert_path(api.asns(), "ipam/asns/");
        assert_path(
            api.fhrp_group_assignments(),
            "ipam/fhrp-group-assignments/",
        );
        assert_path(api.fhrp_groups(), "ipam/fhrp-groups/");
        assert_path(api.ip_addresses(), "ipam/ip-addresses/");
        assert_path(api.ip_ranges(), "ipam/ip-ranges/");
        assert_path(api.prefixes(), "ipam/prefixes/");
        assert_path(api.rirs(), "ipam/rirs/");
        assert_path(api.roles(), "ipam/roles/");
        assert_path(api.route_targets(), "ipam/route-targets/");
        assert_path(api.service_templates(), "ipam/service-templates/");
        assert_path(api.services(), "ipam/services/");
        assert_path(api.vlan_groups(), "ipam/vlan-groups/");
        assert_path(
            api.vlan_translation_policies(),
            "ipam/vlan-translation-policies/",
        );
        assert_path(
            api.vlan_translation_rules(),
            "ipam/vlan-translation-rules/",
        );
        assert_path(api.vlans(), "ipam/vlans/");
        assert_path(api.vrfs(), "ipam/vrfs/");
    }

    #[test]
    fn serialize_ipam_requests() {
        let ip = CreateIpAddressRequest {
            address: "10.0.0.1/24".to_string(),
            vrf: None,
            tenant: None,
            status: Some("active".to_string()),
            role: None,
            assigned_object_type: None,
            assigned_object_id: None,
            dns_name: None,
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&ip).unwrap();
        assert_eq!(value["address"], "10.0.0.1/24");
        assert_eq!(value["status"], "active");

        let prefix = CreatePrefixRequest {
            prefix: "192.168.0.0/24".to_string(),
            site: None,
            vrf: None,
            tenant: None,
            vlan: None,
            status: None,
            role: None,
            is_pool: Some(true),
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&prefix).unwrap();
        assert_eq!(value["prefix"], "192.168.0.0/24");
        assert_eq!(value["is_pool"], true);
    }
}
