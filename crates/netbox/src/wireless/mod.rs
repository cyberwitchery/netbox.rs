//! Wireless API endpoints.

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Request for creating a wireless LAN group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWirelessLanGroupRequest {
    /// Group name.
    pub name: String,
    /// Group slug.
    pub slug: String,
    /// Parent group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// Custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Request for updating a wireless LAN group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWirelessLanGroupRequest {
    /// Updated group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated parent group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// Updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Request for creating a wireless LAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWirelessLanRequest {
    /// SSID string.
    pub ssid: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// VLAN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
    /// Scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// Auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// Auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// Custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// Request for updating a wireless LAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWirelessLanRequest {
    /// Updated SSID string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated VLAN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
    /// Updated scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Updated scope ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Updated auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// Updated auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// Updated auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// Updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// Request for creating a wireless link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWirelessLinkRequest {
    /// Interface A ID.
    pub interface_a: i32,
    /// Interface B ID.
    pub interface_b: i32,
    /// SSID string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// Auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// Auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// Link distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    /// Distance unit slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// Custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// Request for updating a wireless link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWirelessLinkRequest {
    /// Updated interface A ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_a: Option<i32>,
    /// Updated interface B ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_b: Option<i32>,
    /// Updated SSID string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Updated auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// Updated auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// Updated auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// Updated link distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    /// Updated distance unit slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// Updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// Wireless LAN group model.
pub type WirelessLanGroup = crate::models::WirelessLanGroup;
/// Wireless LAN model.
pub type WirelessLan = crate::models::WirelessLan;
/// Wireless link model.
pub type WirelessLink = crate::models::WirelessLink;

/// Resource for wireless LAN groups.
pub type WirelessLanGroupsApi = Resource<crate::models::WirelessLanGroup>;
/// Resource for wireless LANs.
pub type WirelessLansApi = Resource<crate::models::WirelessLan>;
/// Resource for wireless links.
pub type WirelessLinksApi = Resource<crate::models::WirelessLink>;

/// API for wireless endpoints.
#[derive(Clone)]
pub struct WirelessApi {
    client: Client,
}

impl WirelessApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the wireless LAN groups resource.
    pub fn wireless_lan_groups(&self) -> WirelessLanGroupsApi {
        Resource::new(self.client.clone(), "wireless/wireless-lan-groups/")
    }

    /// Returns the wireless LANs resource.
    pub fn wireless_lans(&self) -> WirelessLansApi {
        Resource::new(self.client.clone(), "wireless/wireless-lans/")
    }

    /// Returns the wireless links resource.
    pub fn wireless_links(&self) -> WirelessLinksApi {
        Resource::new(self.client.clone(), "wireless/wireless-links/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use serde_json::json;

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

    fn assert_missing(value: &serde_json::Value, key: &str) {
        assert!(value.get(key).is_none(), "expected {} to be omitted", key);
    }

    #[test]
    fn wireless_accessors_return_expected_paths() {
        let api = WirelessApi::new(test_client());

        assert_path(api.wireless_lan_groups(), "wireless/wireless-lan-groups/");
        assert_path(api.wireless_lans(), "wireless/wireless-lans/");
        assert_path(api.wireless_links(), "wireless/wireless-links/");
    }

    #[test]
    fn serialize_wireless_lan_group_requests() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert("owner".to_string(), json!("netops"));
        let create = CreateWirelessLanGroupRequest {
            name: "campus".to_string(),
            slug: "campus".to_string(),
            parent: None,
            description: Some("Campus groups".to_string()),
            tags: Some(vec![1]),
            custom_fields: Some(custom_fields),
            comments: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "campus");
        assert_eq!(value["slug"], "campus");
        assert_eq!(value["description"], "Campus groups");
        assert_eq!(value["tags"], json!([1]));
        assert_eq!(value["custom_fields"]["owner"], "netops");
        assert_missing(&value, "parent");

        let update = UpdateWirelessLanGroupRequest {
            name: None,
            slug: None,
            parent: Some(2),
            description: None,
            tags: None,
            custom_fields: None,
            comments: Some("Updated".to_string()),
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["parent"], 2);
        assert_eq!(value["comments"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_wireless_lan_requests() {
        let create = CreateWirelessLanRequest {
            ssid: "Guest".to_string(),
            description: None,
            group: Some(1),
            status: Some("active".to_string()),
            vlan: Some(100),
            scope_type: None,
            scope_id: None,
            tenant: None,
            auth_type: Some("wpa-personal".to_string()),
            auth_cipher: Some("aes".to_string()),
            auth_psk: None,
            comments: None,
            tags: Some(vec![2]),
            custom_fields: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["ssid"], "Guest");
        assert_eq!(value["group"], 1);
        assert_eq!(value["status"], "active");
        assert_eq!(value["vlan"], 100);
        assert_eq!(value["auth_type"], "wpa-personal");
        assert_eq!(value["auth_cipher"], "aes");
        assert_eq!(value["tags"], json!([2]));

        let update = UpdateWirelessLanRequest {
            ssid: None,
            description: Some("Updated".to_string()),
            group: None,
            status: None,
            vlan: None,
            scope_type: Some("dcim.site".to_string()),
            scope_id: Some(5),
            tenant: None,
            auth_type: None,
            auth_cipher: None,
            auth_psk: None,
            comments: None,
            tags: None,
            custom_fields: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_eq!(value["scope_type"], "dcim.site");
        assert_eq!(value["scope_id"], 5);
        assert_missing(&value, "ssid");
    }

    #[test]
    fn serialize_wireless_link_requests() {
        let create = CreateWirelessLinkRequest {
            interface_a: 1,
            interface_b: 2,
            ssid: Some("Backhaul".to_string()),
            status: Some("connected".to_string()),
            tenant: None,
            auth_type: None,
            auth_cipher: None,
            auth_psk: None,
            distance: Some(0.5),
            distance_unit: Some("km".to_string()),
            description: None,
            comments: None,
            tags: Some(vec![7]),
            custom_fields: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["interface_a"], 1);
        assert_eq!(value["interface_b"], 2);
        assert_eq!(value["ssid"], "Backhaul");
        assert_eq!(value["status"], "connected");
        assert_eq!(value["distance"], 0.5);
        assert_eq!(value["distance_unit"], "km");
        assert_eq!(value["tags"], json!([7]));

        let update = UpdateWirelessLinkRequest {
            interface_a: None,
            interface_b: None,
            ssid: None,
            status: Some("planned".to_string()),
            tenant: Some(3),
            auth_type: None,
            auth_cipher: None,
            auth_psk: None,
            distance: None,
            distance_unit: None,
            description: Some("Planned link".to_string()),
            comments: None,
            tags: None,
            custom_fields: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["status"], "planned");
        assert_eq!(value["tenant"], 3);
        assert_eq!(value["description"], "Planned link");
        assert_missing(&value, "interface_a");
    }
}
