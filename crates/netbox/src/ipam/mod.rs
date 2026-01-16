//! ipam endpoints for prefixes, addresses, vrfs, vlans, and asns.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let prefixes = client.ipam().prefixes().list(None).await?;
//! println!("{}", prefixes.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// request for creating a new IP address (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpAddressRequest {
    /// cIDR address string.
    pub address: String,
    /// vRF id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// role slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// assigned object type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_object_type: Option<String>,
    /// assigned object id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_object_id: Option<i32>,
    /// dNS name value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating an IP address (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIpAddressRequest {
    /// updated CIDR address string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// updated DNS name value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// request for patching fields on an ip address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchIpAddressFieldsRequest {
    /// custom field values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// tag objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
}

/// request for creating a new prefix (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrefixRequest {
    /// cIDR prefix string.
    pub prefix: String,
    /// site id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// vRF id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i32>,
    /// tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// vLAN id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
    /// status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// role id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    /// pool flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pool: Option<bool>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a prefix (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePrefixRequest {
    /// updated CIDR prefix string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// updated site id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
    /// updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// request for patching fields on a prefix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPrefixFieldsRequest {
    /// custom field values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// tag objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
}

/// iP address model.
pub type IpAddress = crate::models::IpAddress;
/// prefix model.
pub type Prefix = crate::models::Prefix;

/// resource for aggregates.
pub type AggregatesApi = Resource<crate::models::Aggregate>;
/// resource for ASN ranges.
pub type AsnRangesApi = Resource<crate::models::AsnRange>;
/// resource for ASNs.
pub type AsnsApi = Resource<crate::models::Asn>;
/// resource for FHRP group assignments.
pub type FhrpGroupAssignmentsApi = Resource<crate::models::FhrpGroupAssignment>;
/// resource for FHRP groups.
pub type FhrpGroupsApi = Resource<crate::models::FhrpGroup>;
/// resource for IP addresses.
pub type IpAddressesApi = Resource<crate::models::IpAddress>;
/// resource for IP ranges.
pub type IpRangesApi = Resource<crate::models::IpRange>;
/// resource for prefixes.
pub type PrefixesApi = Resource<crate::models::Prefix>;
/// resource for RIRs.
pub type RirsApi = Resource<crate::models::Rir>;
/// resource for roles.
pub type RolesApi = Resource<crate::models::Role>;
/// resource for route targets.
pub type RouteTargetsApi = Resource<crate::models::RouteTarget>;
/// resource for service templates.
pub type ServiceTemplatesApi = Resource<crate::models::ServiceTemplate>;
/// resource for services.
pub type ServicesApi = Resource<crate::models::Service>;
/// resource for VLAN groups.
pub type VlanGroupsApi = Resource<crate::models::VlanGroup>;
/// resource for VLAN translation policies.
pub type VlanTranslationPoliciesApi = Resource<crate::models::VlanTranslationPolicy>;
/// resource for VLAN translation rules.
pub type VlanTranslationRulesApi = Resource<crate::models::VlanTranslationRule>;
/// resource for VLANs.
pub type VlansApi = Resource<crate::models::Vlan>;
/// resource for VRFs.
pub type VrfsApi = Resource<crate::models::Vrf>;

/// api for ipam endpoints
#[derive(Clone)]
pub struct IpamApi {
    client: Client,
}

impl IpamApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the aggregates resource.
    pub fn aggregates(&self) -> AggregatesApi {
        Resource::new(self.client.clone(), "ipam/aggregates/")
    }

    /// returns the ASN ranges resource.
    pub fn asn_ranges(&self) -> AsnRangesApi {
        Resource::new(self.client.clone(), "ipam/asn-ranges/")
    }

    /// returns the ASNs resource.
    pub fn asns(&self) -> AsnsApi {
        Resource::new(self.client.clone(), "ipam/asns/")
    }

    /// returns the FHRP group assignments resource.
    pub fn fhrp_group_assignments(&self) -> FhrpGroupAssignmentsApi {
        Resource::new(self.client.clone(), "ipam/fhrp-group-assignments/")
    }

    /// returns the FHRP groups resource.
    pub fn fhrp_groups(&self) -> FhrpGroupsApi {
        Resource::new(self.client.clone(), "ipam/fhrp-groups/")
    }

    /// returns the IP addresses resource.
    pub fn ip_addresses(&self) -> IpAddressesApi {
        Resource::new(self.client.clone(), "ipam/ip-addresses/")
    }

    /// returns the IP ranges resource.
    pub fn ip_ranges(&self) -> IpRangesApi {
        Resource::new(self.client.clone(), "ipam/ip-ranges/")
    }

    /// returns the prefixes resource.
    pub fn prefixes(&self) -> PrefixesApi {
        Resource::new(self.client.clone(), "ipam/prefixes/")
    }

    /// returns the RIRs resource.
    pub fn rirs(&self) -> RirsApi {
        Resource::new(self.client.clone(), "ipam/rirs/")
    }

    /// returns the roles resource.
    pub fn roles(&self) -> RolesApi {
        Resource::new(self.client.clone(), "ipam/roles/")
    }

    /// returns the route targets resource.
    pub fn route_targets(&self) -> RouteTargetsApi {
        Resource::new(self.client.clone(), "ipam/route-targets/")
    }

    /// returns the service templates resource.
    pub fn service_templates(&self) -> ServiceTemplatesApi {
        Resource::new(self.client.clone(), "ipam/service-templates/")
    }

    /// returns the services resource.
    pub fn services(&self) -> ServicesApi {
        Resource::new(self.client.clone(), "ipam/services/")
    }

    /// returns the VLAN groups resource.
    pub fn vlan_groups(&self) -> VlanGroupsApi {
        Resource::new(self.client.clone(), "ipam/vlan-groups/")
    }

    /// returns the VLAN translation policies resource.
    pub fn vlan_translation_policies(&self) -> VlanTranslationPoliciesApi {
        Resource::new(self.client.clone(), "ipam/vlan-translation-policies/")
    }

    /// returns the VLAN translation rules resource.
    pub fn vlan_translation_rules(&self) -> VlanTranslationRulesApi {
        Resource::new(self.client.clone(), "ipam/vlan-translation-rules/")
    }

    /// returns the VLANs resource.
    pub fn vlans(&self) -> VlansApi {
        Resource::new(self.client.clone(), "ipam/vlans/")
    }

    /// returns the VRFs resource.
    pub fn vrfs(&self) -> VrfsApi {
        Resource::new(self.client.clone(), "ipam/vrfs/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use serde_json::json;
    use std::collections::HashMap;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://netbox.example.com", "token");
        Client::new(config).unwrap()
    }

    fn assert_path<T>(resource: Resource<T>, expected: &str)
    where
        T: serde::de::DeserializeOwned,
    {
        let paginator = resource.paginate(None).unwrap();
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn ipam_accessors_return_expected_paths() {
        let api = IpamApi::new(test_client());

        assert_path(api.aggregates(), "ipam/aggregates/");
        assert_path(api.asn_ranges(), "ipam/asn-ranges/");
        assert_path(api.asns(), "ipam/asns/");
        assert_path(api.fhrp_group_assignments(), "ipam/fhrp-group-assignments/");
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
        assert_path(api.vlan_translation_rules(), "ipam/vlan-translation-rules/");
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

    #[test]
    fn serialize_projection_patch_requests() {
        let mut fields = HashMap::new();
        fields.insert("owner".to_string(), json!("netops"));
        let tags = vec![crate::models::NestedTag::new(
            "Core".to_string(),
            "core".to_string(),
        )];

        let prefix = PatchPrefixFieldsRequest {
            custom_fields: Some(fields.clone()),
            tags: Some(tags.clone()),
        };
        let value = serde_json::to_value(&prefix).unwrap();
        assert_eq!(value["custom_fields"]["owner"], "netops");
        assert_eq!(value["tags"][0]["slug"], "core");

        let ip = PatchIpAddressFieldsRequest {
            custom_fields: Some(fields),
            tags: Some(tags),
        };
        let value = serde_json::to_value(&ip).unwrap();
        assert_eq!(value["custom_fields"]["owner"], "netops");
        assert_eq!(value["tags"][0]["name"], "Core");
    }
}
