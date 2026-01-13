//! wireless endpoints for lans, groups, and links.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let lans = client.wireless().wireless_lans().list(None).await?;
//! println!("{}", lans.count);
//! # Ok(())
//! # }
//! ```

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// request for creating a wireless LAN group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWirelessLanGroupRequest {
    /// group name.
    pub name: String,
    /// group slug.
    pub slug: String,
    /// parent group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// request for updating a wireless LAN group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWirelessLanGroupRequest {
    /// updated group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// updated parent group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// request for creating a wireless LAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWirelessLanRequest {
    /// sSID string.
    pub ssid: String,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// vLAN id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
    /// scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// scope id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// request for updating a wireless LAN.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWirelessLanRequest {
    /// updated SSID string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// updated VLAN id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
    /// updated scope type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// updated scope id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i32>,
    /// updated tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// updated auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// updated auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// updated auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// request for creating a wireless link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWirelessLinkRequest {
    /// interface A id.
    pub interface_a: i32,
    /// interface B id.
    pub interface_b: i32,
    /// sSID string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// link distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    /// distance unit slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// request for updating a wireless link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWirelessLinkRequest {
    /// updated interface A id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_a: Option<i32>,
    /// updated interface B id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_b: Option<i32>,
    /// updated SSID string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// updated tenant id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// updated auth type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// updated auth cipher slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<String>,
    /// updated auth PSK string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    /// updated link distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    /// updated distance unit slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
    /// updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
}

/// wireless LAN group model.
pub type WirelessLanGroup = crate::models::WirelessLanGroup;
/// wireless LAN model.
pub type WirelessLan = crate::models::WirelessLan;
/// wireless link model.
pub type WirelessLink = crate::models::WirelessLink;

/// resource for wireless LAN groups.
pub type WirelessLanGroupsApi = Resource<crate::models::WirelessLanGroup>;
/// resource for wireless LANs.
pub type WirelessLansApi = Resource<crate::models::WirelessLan>;
/// resource for wireless links.
pub type WirelessLinksApi = Resource<crate::models::WirelessLink>;

/// api for wireless endpoints.
#[derive(Clone)]
pub struct WirelessApi {
    client: Client,
}

impl WirelessApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the wireless LAN groups resource.
    pub fn wireless_lan_groups(&self) -> WirelessLanGroupsApi {
        Resource::new(self.client.clone(), "wireless/wireless-lan-groups/")
    }

    /// returns the wireless LANs resource.
    pub fn wireless_lans(&self) -> WirelessLansApi {
        Resource::new(self.client.clone(), "wireless/wireless-lans/")
    }

    /// returns the wireless links resource.
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
