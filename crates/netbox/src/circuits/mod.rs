//! Circuits API endpoints.

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};

/// Request for creating a provider (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderRequest {
    /// Provider name.
    pub name: String,
    /// Provider slug.
    pub slug: String,
    /// Provider account IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<i32>>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// ASN IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asns: Option<Vec<i32>>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a provider (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderRequest {
    /// Updated provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated provider slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated provider account IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<i32>>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated ASN IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asns: Option<Vec<i32>>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a provider network (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderNetworkRequest {
    /// Provider ID.
    pub provider: i32,
    /// Provider network name.
    pub name: String,
    /// Service ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
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

/// Request for updating a provider network (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderNetworkRequest {
    /// Updated provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<i32>,
    /// Updated provider network name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated service ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
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

/// Request for creating a circuit type (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCircuitTypeRequest {
    /// Circuit type name.
    pub name: String,
    /// Circuit type slug.
    pub slug: String,
    /// Color hex value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a circuit type (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCircuitTypeRequest {
    /// Updated circuit type name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated circuit type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated color hex value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a circuit (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCircuitRequest {
    /// Circuit ID (CID).
    pub cid: String,
    /// Provider ID.
    pub provider: i32,
    /// Provider account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_account: Option<i32>,
    /// Circuit type ID.
    pub r#type: i32,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Install date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,
    /// Termination date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    /// Commit rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_rate: Option<i32>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Distance value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    /// Distance unit slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a circuit (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCircuitRequest {
    /// Updated circuit ID (CID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    /// Updated provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<i32>,
    /// Updated provider account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_account: Option<i32>,
    /// Updated circuit type ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Updated install date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,
    /// Updated termination date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    /// Updated commit rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_rate: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated distance value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    /// Updated distance unit slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a virtual circuit (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualCircuitRequest {
    /// Virtual circuit ID (CID).
    pub cid: String,
    /// Provider network ID.
    pub provider_network: i32,
    /// Provider account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_account: Option<i32>,
    /// Virtual circuit type ID.
    pub r#type: i32,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
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

/// Request for updating a virtual circuit (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualCircuitRequest {
    /// Updated virtual circuit ID (CID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    /// Updated provider network ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_network: Option<i32>,
    /// Updated provider account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_account: Option<i32>,
    /// Updated virtual circuit type ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
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

/// Request for creating a provider account (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderAccountRequest {
    /// Provider ID.
    pub provider: i32,
    /// Account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Account identifier.
    pub account: String,
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

/// Request for updating a provider account (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderAccountRequest {
    /// Updated provider ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<i32>,
    /// Updated account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated account identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
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

/// Request for creating a circuit termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCircuitTerminationRequest {
    /// Circuit ID.
    pub circuit: i32,
    /// Termination side (A/Z).
    pub term_side: String,
    /// Termination object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_type: Option<String>,
    /// Termination object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_id: Option<i32>,
    /// Port speed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_speed: Option<i32>,
    /// Upstream speed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_speed: Option<i32>,
    /// Cross-connect ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xconnect_id: Option<String>,
    /// Patch panel info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pp_info: Option<String>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Mark as connected flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a circuit termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCircuitTerminationRequest {
    /// Updated circuit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circuit: Option<i32>,
    /// Updated termination side (A/Z).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_side: Option<String>,
    /// Updated termination object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_type: Option<String>,
    /// Updated termination object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_id: Option<i32>,
    /// Updated port speed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_speed: Option<i32>,
    /// Updated upstream speed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_speed: Option<i32>,
    /// Updated cross-connect ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xconnect_id: Option<String>,
    /// Updated patch panel info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pp_info: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated mark as connected flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a virtual circuit termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualCircuitTerminationRequest {
    /// Virtual circuit ID.
    pub virtual_circuit: i32,
    /// Role slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Interface ID.
    pub interface: i32,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a virtual circuit termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualCircuitTerminationRequest {
    /// Updated virtual circuit ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_circuit: Option<i32>,
    /// Updated role slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Updated interface ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a circuit group assignment (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCircuitGroupAssignmentRequest {
    /// Circuit group ID.
    pub group: i32,
    /// Member type string.
    pub member_type: String,
    /// Member object ID.
    pub member_id: i64,
    /// Priority slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a circuit group assignment (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCircuitGroupAssignmentRequest {
    /// Updated circuit group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// Updated member type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// Updated member object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<i64>,
    /// Updated priority slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
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
    fn serialize_provider_requests() {
        let create = CreateProviderRequest {
            name: "carrier".to_string(),
            slug: "carrier".to_string(),
            accounts: None,
            description: None,
            comments: None,
            asns: Some(vec![65000]),
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "carrier");
        assert_eq!(value["slug"], "carrier");
        assert_eq!(value["asns"], serde_json::json!([65000]));
        assert_missing(&value, "accounts");
        assert_missing(&value, "tags");

        let update = UpdateProviderRequest {
            name: None,
            slug: Some("carrier-new".to_string()),
            accounts: None,
            description: Some("Updated".to_string()),
            comments: None,
            asns: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["slug"], "carrier-new");
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
        assert_missing(&value, "asns");
    }

    #[test]
    fn serialize_provider_network_requests() {
        let create = CreateProviderNetworkRequest {
            provider: 1,
            name: "core".to_string(),
            service_id: None,
            description: None,
            comments: Some("Notes".to_string()),
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["provider"], 1);
        assert_eq!(value["name"], "core");
        assert_eq!(value["comments"], "Notes");
        assert_missing(&value, "service_id");

        let update = UpdateProviderNetworkRequest {
            provider: None,
            name: Some("core-new".to_string()),
            service_id: Some("svc-1".to_string()),
            description: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["name"], "core-new");
        assert_eq!(value["service_id"], "svc-1");
        assert_missing(&value, "provider");
    }

    #[test]
    fn serialize_provider_account_requests() {
        let create = CreateProviderAccountRequest {
            provider: 10,
            name: None,
            account: "ACC-1".to_string(),
            description: None,
            comments: None,
            tags: Some(vec![1, 2]),
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["provider"], 10);
        assert_eq!(value["account"], "ACC-1");
        assert_eq!(value["tags"], serde_json::json!([1, 2]));
        assert_missing(&value, "name");

        let update = UpdateProviderAccountRequest {
            provider: None,
            name: Some("Acct".to_string()),
            account: None,
            description: Some("Desc".to_string()),
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["name"], "Acct");
        assert_eq!(value["description"], "Desc");
        assert_missing(&value, "provider");
    }

    #[test]
    fn serialize_circuit_type_requests() {
        let create = CreateCircuitTypeRequest {
            name: "mpls".to_string(),
            slug: "mpls".to_string(),
            color: None,
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "mpls");
        assert_eq!(value["slug"], "mpls");
        assert_missing(&value, "color");

        let update = UpdateCircuitTypeRequest {
            name: None,
            slug: Some("mpls-new".to_string()),
            color: Some("ff00ff".to_string()),
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["slug"], "mpls-new");
        assert_eq!(value["color"], "ff00ff");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_circuit_requests() {
        let create = CreateCircuitRequest {
            cid: "C-1".to_string(),
            provider: 1,
            provider_account: None,
            r#type: 2,
            status: Some("active".to_string()),
            tenant: None,
            install_date: None,
            termination_date: None,
            commit_rate: Some(1000),
            description: None,
            distance: None,
            distance_unit: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["cid"], "C-1");
        assert_eq!(value["provider"], 1);
        assert_eq!(value["type"], 2);
        assert_eq!(value["status"], "active");
        assert_eq!(value["commit_rate"], 1000);
        assert_missing(&value, "provider_account");
        assert_missing(&value, "distance_unit");

        let update = UpdateCircuitRequest {
            cid: None,
            provider: None,
            provider_account: Some(5),
            r#type: None,
            status: Some("offline".to_string()),
            tenant: None,
            install_date: None,
            termination_date: None,
            commit_rate: None,
            description: Some("Updated".to_string()),
            distance: None,
            distance_unit: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["provider_account"], 5);
        assert_eq!(value["status"], "offline");
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "cid");
        assert_missing(&value, "provider");
    }

    #[test]
    fn serialize_circuit_termination_requests() {
        let create = CreateCircuitTerminationRequest {
            circuit: 3,
            term_side: "A".to_string(),
            termination_type: Some("dcim.interface".to_string()),
            termination_id: Some(9),
            port_speed: None,
            upstream_speed: None,
            xconnect_id: None,
            pp_info: None,
            description: None,
            mark_connected: Some(true),
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["circuit"], 3);
        assert_eq!(value["term_side"], "A");
        assert_eq!(value["termination_type"], "dcim.interface");
        assert_eq!(value["termination_id"], 9);
        assert_eq!(value["mark_connected"], true);
        assert_missing(&value, "port_speed");

        let update = UpdateCircuitTerminationRequest {
            circuit: None,
            term_side: None,
            termination_type: None,
            termination_id: None,
            port_speed: Some(100),
            upstream_speed: None,
            xconnect_id: None,
            pp_info: None,
            description: Some("Term".to_string()),
            mark_connected: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["port_speed"], 100);
        assert_eq!(value["description"], "Term");
        assert_missing(&value, "circuit");
    }

    #[test]
    fn serialize_circuit_group_assignment_requests() {
        let create = CreateCircuitGroupAssignmentRequest {
            group: 1,
            member_type: "circuits.circuit".to_string(),
            member_id: 42,
            priority: Some("primary".to_string()),
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["group"], 1);
        assert_eq!(value["member_type"], "circuits.circuit");
        assert_eq!(value["member_id"], 42);
        assert_eq!(value["priority"], "primary");
        assert_missing(&value, "tags");

        let update = UpdateCircuitGroupAssignmentRequest {
            group: None,
            member_type: None,
            member_id: None,
            priority: Some("secondary".to_string()),
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["priority"], "secondary");
        assert_missing(&value, "group");
    }

    #[test]
    fn serialize_virtual_circuit_requests() {
        let create = CreateVirtualCircuitRequest {
            cid: "VC-1".to_string(),
            provider_network: 1,
            provider_account: None,
            r#type: 2,
            status: Some("active".to_string()),
            tenant: None,
            description: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["cid"], "VC-1");
        assert_eq!(value["provider_network"], 1);
        assert_eq!(value["type"], 2);
        assert_eq!(value["status"], "active");
        assert_missing(&value, "provider_account");

        let update = UpdateVirtualCircuitRequest {
            cid: None,
            provider_network: None,
            provider_account: Some(2),
            r#type: None,
            status: None,
            tenant: None,
            description: Some("Updated".to_string()),
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["provider_account"], 2);
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "cid");
    }

    #[test]
    fn serialize_virtual_circuit_termination_requests() {
        let create = CreateVirtualCircuitTerminationRequest {
            virtual_circuit: 1,
            role: Some("peer".to_string()),
            interface: 2,
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["virtual_circuit"], 1);
        assert_eq!(value["role"], "peer");
        assert_eq!(value["interface"], 2);
        assert_missing(&value, "description");

        let update = UpdateVirtualCircuitTerminationRequest {
            virtual_circuit: None,
            role: None,
            interface: Some(3),
            description: Some("Term".to_string()),
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["interface"], 3);
        assert_eq!(value["description"], "Term");
        assert_missing(&value, "virtual_circuit");
    }

    proptest! {
        #[test]
        fn prop_create_circuit_optional_fields(
            status in proptest::option::of("[a-z0-9-]{1,16}"),
            provider_account in proptest::option::of(0i32..1000),
        ) {
            let request = CreateCircuitRequest {
                cid: "C-1".to_string(),
                provider: 1,
                provider_account,
                r#type: 2,
                status: status.clone(),
                tenant: None,
                install_date: None,
                termination_date: None,
                commit_rate: None,
                description: None,
                distance: None,
                distance_unit: None,
                comments: None,
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_string(&value, "status", &status);
            assert_optional_i32(&value, "provider_account", &provider_account);
        }
    }

    proptest! {
        #[test]
        fn prop_update_circuit_termination_optional_fields(
            description in proptest::option::of("[a-z0-9-]{0,16}"),
            mark_connected in proptest::option::of(any::<bool>()),
        ) {
            let request = UpdateCircuitTerminationRequest {
                circuit: None,
                term_side: None,
                termination_type: None,
                termination_id: None,
                port_speed: None,
                upstream_speed: None,
                xconnect_id: None,
                pp_info: None,
                description: description.clone(),
                mark_connected,
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_string(&value, "description", &description);
            match mark_connected {
                Some(flag) => assert_eq!(value["mark_connected"], flag),
                None => assert_missing(&value, "mark_connected"),
            }
        }
    }

    proptest! {
        #[test]
        fn prop_virtual_circuit_termination_optional_fields(
            role in proptest::option::of("[a-z0-9-]{1,8}"),
            description in proptest::option::of("[a-z0-9-]{0,16}"),
        ) {
            let request = CreateVirtualCircuitTerminationRequest {
                virtual_circuit: 1,
                role: role.clone(),
                interface: 2,
                description: description.clone(),
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_string(&value, "role", &role);
            assert_optional_string(&value, "description", &description);
        }
    }
}

/// Circuit model.
pub type Circuit = crate::models::Circuit;
/// Circuit termination model.
pub type CircuitTermination = crate::models::CircuitTermination;
/// Circuit type model.
pub type CircuitType = crate::models::CircuitType;
/// Circuit group model.
pub type CircuitGroup = crate::models::CircuitGroup;
/// Circuit group assignment model.
pub type CircuitGroupAssignment = crate::models::CircuitGroupAssignment;
/// Provider model.
pub type Provider = crate::models::Provider;
/// Provider network model.
pub type ProviderNetwork = crate::models::ProviderNetwork;
/// Provider account model.
pub type ProviderAccount = crate::models::ProviderAccount;
/// Virtual circuit model.
pub type VirtualCircuit = crate::models::VirtualCircuit;
/// Virtual circuit termination model.
pub type VirtualCircuitTermination = crate::models::VirtualCircuitTermination;
/// Virtual circuit type model.
pub type VirtualCircuitType = crate::models::VirtualCircuitType;

/// Resource for circuit group assignments.
pub type CircuitGroupAssignmentsApi = Resource<crate::models::CircuitGroupAssignment>;
/// Resource for circuit groups.
pub type CircuitGroupsApi = Resource<crate::models::CircuitGroup>;
/// Resource for circuit terminations.
pub type CircuitTerminationsApi = Resource<crate::models::CircuitTermination>;
/// Resource for circuit types.
pub type CircuitTypesApi = Resource<crate::models::CircuitType>;
/// Resource for circuits.
pub type CircuitsResource = Resource<crate::models::Circuit>;
/// Resource for provider accounts.
pub type ProviderAccountsApi = Resource<crate::models::ProviderAccount>;
/// Resource for provider networks.
pub type ProviderNetworksApi = Resource<crate::models::ProviderNetwork>;
/// Resource for providers.
pub type ProvidersApi = Resource<crate::models::Provider>;
/// Resource for virtual circuit terminations.
pub type VirtualCircuitTerminationsApi = Resource<crate::models::VirtualCircuitTermination>;
/// Resource for virtual circuit types.
pub type VirtualCircuitTypesApi = Resource<crate::models::VirtualCircuitType>;
/// Resource for virtual circuits.
pub type VirtualCircuitsApi = Resource<crate::models::VirtualCircuit>;

/// API for circuits endpoints.
#[derive(Clone)]
pub struct CircuitsApi {
    client: Client,
}

impl CircuitsApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the circuit group assignments resource.
    pub fn circuit_group_assignments(&self) -> CircuitGroupAssignmentsApi {
        Resource::new(self.client.clone(), "circuits/circuit-group-assignments/")
    }

    /// Returns the circuit groups resource.
    pub fn circuit_groups(&self) -> CircuitGroupsApi {
        Resource::new(self.client.clone(), "circuits/circuit-groups/")
    }

    /// Returns the circuit terminations resource.
    pub fn circuit_terminations(&self) -> CircuitTerminationsApi {
        Resource::new(self.client.clone(), "circuits/circuit-terminations/")
    }

    /// Returns the circuit types resource.
    pub fn circuit_types(&self) -> CircuitTypesApi {
        Resource::new(self.client.clone(), "circuits/circuit-types/")
    }

    /// Returns the circuits resource.
    pub fn circuits(&self) -> CircuitsResource {
        Resource::new(self.client.clone(), "circuits/circuits/")
    }

    /// Returns the provider accounts resource.
    pub fn provider_accounts(&self) -> ProviderAccountsApi {
        Resource::new(self.client.clone(), "circuits/provider-accounts/")
    }

    /// Returns the provider networks resource.
    pub fn provider_networks(&self) -> ProviderNetworksApi {
        Resource::new(self.client.clone(), "circuits/provider-networks/")
    }

    /// Returns the providers resource.
    pub fn providers(&self) -> ProvidersApi {
        Resource::new(self.client.clone(), "circuits/providers/")
    }

    /// Returns the virtual circuit terminations resource.
    pub fn virtual_circuit_terminations(&self) -> VirtualCircuitTerminationsApi {
        Resource::new(self.client.clone(), "circuits/virtual-circuit-terminations/")
    }

    /// Returns the virtual circuit types resource.
    pub fn virtual_circuit_types(&self) -> VirtualCircuitTypesApi {
        Resource::new(self.client.clone(), "circuits/virtual-circuit-types/")
    }

    /// Returns the virtual circuits resource.
    pub fn virtual_circuits(&self) -> VirtualCircuitsApi {
        Resource::new(self.client.clone(), "circuits/virtual-circuits/")
    }
}
