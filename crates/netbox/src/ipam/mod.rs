//! ipam endpoints for prefixes, addresses, vrfs, vlans, and asns.
//!
//! includes availability queries for allocating IPs, prefixes, VLANs, and ASNs
//! from their respective pools.
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
//!
//! availability queries:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! // list available IPs in a prefix
//! let available = client.ipam().available_ips_in_prefix(42).await?;
//!
//! // allocate from available pool
//! let created = client.ipam().create_available_ips_in_prefix(42, &[
//!     serde_json::json!({"description": "allocated via api"})
//! ]).await?;
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// available IP model.
pub type AvailableIp = crate::models::AvailableIp;
/// available prefix model.
pub type AvailablePrefix = crate::models::AvailablePrefix;
/// available ASN model.
pub type AvailableAsn = crate::models::AvailableAsn;
/// available VLAN model.
pub type AvailableVlan = crate::models::AvailableVlan;
/// ASN model.
pub type Asn = crate::models::Asn;
/// VLAN model.
pub type Vlan = crate::models::Vlan;

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
    /// scope type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// scope id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
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
    /// updated scope type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// updated scope id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
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

    /// list available IPs within a prefix.
    pub async fn available_ips_in_prefix(&self, id: u64) -> Result<Vec<AvailableIp>> {
        self.client
            .get(&format!("ipam/prefixes/{}/available-ips/", id))
            .await
    }

    /// create IP addresses from the available IPs within a prefix.
    pub async fn create_available_ips_in_prefix<B: Serialize>(
        &self,
        id: u64,
        body: &[B],
    ) -> Result<Vec<IpAddress>> {
        self.client
            .post(&format!("ipam/prefixes/{}/available-ips/", id), body)
            .await
    }

    /// list available child prefixes within a prefix.
    pub async fn available_prefixes_in_prefix(&self, id: u64) -> Result<Vec<AvailablePrefix>> {
        self.client
            .get(&format!("ipam/prefixes/{}/available-prefixes/", id))
            .await
    }

    /// create prefixes from the available child prefixes within a prefix.
    pub async fn create_available_prefixes_in_prefix<B: Serialize>(
        &self,
        id: u64,
        body: &[B],
    ) -> Result<Vec<Prefix>> {
        self.client
            .post(&format!("ipam/prefixes/{}/available-prefixes/", id), body)
            .await
    }

    /// list available IPs within an IP range.
    pub async fn available_ips_in_range(&self, id: u64) -> Result<Vec<AvailableIp>> {
        self.client
            .get(&format!("ipam/ip-ranges/{}/available-ips/", id))
            .await
    }

    /// create IP addresses from the available IPs within an IP range.
    pub async fn create_available_ips_in_range<B: Serialize>(
        &self,
        id: u64,
        body: &[B],
    ) -> Result<Vec<IpAddress>> {
        self.client
            .post(&format!("ipam/ip-ranges/{}/available-ips/", id), body)
            .await
    }

    /// list available VLANs within a VLAN group.
    pub async fn available_vlans_in_group(&self, id: u64) -> Result<Vec<AvailableVlan>> {
        self.client
            .get(&format!("ipam/vlan-groups/{}/available-vlans/", id))
            .await
    }

    /// create VLANs from the available VLANs within a VLAN group.
    pub async fn create_available_vlans_in_group<B: Serialize>(
        &self,
        id: u64,
        body: &[B],
    ) -> Result<Vec<Vlan>> {
        self.client
            .post(&format!("ipam/vlan-groups/{}/available-vlans/", id), body)
            .await
    }

    /// list available ASNs within an ASN range.
    pub async fn available_asns_in_range(&self, id: u64) -> Result<Vec<AvailableAsn>> {
        self.client
            .get(&format!("ipam/asn-ranges/{}/available-asns/", id))
            .await
    }

    /// create ASNs from the available ASNs within an ASN range.
    pub async fn create_available_asns_in_range<B: Serialize>(
        &self,
        id: u64,
        body: &[B],
    ) -> Result<Vec<Asn>> {
        self.client
            .post(&format!("ipam/asn-ranges/{}/available-asns/", id), body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::prelude::*;
    use serde_json::json;
    use std::collections::HashMap;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://netbox.example.com", "token");
        Client::new(config).unwrap()
    }

    fn mock_client(server: &MockServer) -> Client {
        let config = ClientConfig::new(server.base_url(), "test-token");
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
            scope_type: None,
            scope_id: None,
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

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn availability_endpoints_use_expected_paths() {
        let server = MockServer::start();
        let client = mock_client(&server);

        let mock1 = server.mock(|when, then| {
            when.method(GET)
                .path("/api/ipam/prefixes/42/available-ips/");
            then.status(200).json_body(json!([]));
        });
        let _ = client.ipam().available_ips_in_prefix(42).await;
        mock1.assert();

        let mock2 = server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/prefixes/42/available-ips/");
            then.status(201).json_body(json!([]));
        });
        let body = vec![json!({"description": "test"})];
        let _ = client
            .ipam()
            .create_available_ips_in_prefix(42, &body)
            .await;
        mock2.assert();

        let mock3 = server.mock(|when, then| {
            when.method(GET)
                .path("/api/ipam/prefixes/42/available-prefixes/");
            then.status(200).json_body(json!([]));
        });
        let _ = client.ipam().available_prefixes_in_prefix(42).await;
        mock3.assert();

        let mock4 = server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/prefixes/42/available-prefixes/");
            then.status(201).json_body(json!([]));
        });
        let body = vec![json!({"prefix_length": 26})];
        let _ = client
            .ipam()
            .create_available_prefixes_in_prefix(42, &body)
            .await;
        mock4.assert();

        let mock5 = server.mock(|when, then| {
            when.method(GET)
                .path("/api/ipam/ip-ranges/10/available-ips/");
            then.status(200).json_body(json!([]));
        });
        let _ = client.ipam().available_ips_in_range(10).await;
        mock5.assert();

        let mock6 = server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/ip-ranges/10/available-ips/");
            then.status(201).json_body(json!([]));
        });
        let body = vec![json!({})];
        let _ = client.ipam().create_available_ips_in_range(10, &body).await;
        mock6.assert();

        let mock7 = server.mock(|when, then| {
            when.method(GET)
                .path("/api/ipam/vlan-groups/5/available-vlans/");
            then.status(200).json_body(json!([]));
        });
        let _ = client.ipam().available_vlans_in_group(5).await;
        mock7.assert();

        let mock8 = server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/vlan-groups/5/available-vlans/");
            then.status(201).json_body(json!([]));
        });
        let body = vec![json!({"name": "test-vlan"})];
        let _ = client
            .ipam()
            .create_available_vlans_in_group(5, &body)
            .await;
        mock8.assert();

        let mock9 = server.mock(|when, then| {
            when.method(GET)
                .path("/api/ipam/asn-ranges/3/available-asns/");
            then.status(200).json_body(json!([]));
        });
        let _ = client.ipam().available_asns_in_range(3).await;
        mock9.assert();

        let mock10 = server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/asn-ranges/3/available-asns/");
            then.status(201).json_body(json!([]));
        });
        let body = vec![json!({})];
        let _ = client.ipam().create_available_asns_in_range(3, &body).await;
        mock10.assert();
    }
}
