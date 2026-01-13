//! VPN API endpoints.

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};

/// Request for creating a tunnel group (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTunnelGroupRequest {
    /// Tunnel group name.
    pub name: String,
    /// Tunnel group slug.
    pub slug: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a tunnel group (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTunnelGroupRequest {
    /// Updated tunnel group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated tunnel group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a tunnel (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTunnelRequest {
    /// Tunnel name.
    pub name: String,
    /// Status slug.
    pub status: String,
    /// Encapsulation slug.
    pub encapsulation: String,
    /// Tunnel group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// IPsec profile ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec_profile: Option<i32>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Tunnel ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<i64>,
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

/// Request for updating a tunnel (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTunnelRequest {
    /// Updated tunnel name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated encapsulation slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encapsulation: Option<String>,
    /// Updated tunnel group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
    /// Updated IPsec profile ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec_profile: Option<i32>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Updated tunnel ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<i64>,
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

/// Request for creating an L2VPN (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateL2VpnRequest {
    /// L2VPN identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<i64>,
    /// L2VPN name.
    pub name: String,
    /// L2VPN slug.
    pub slug: String,
    /// L2VPN type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Import target IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_targets: Option<Vec<i32>>,
    /// Export target IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_targets: Option<Vec<i32>>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an L2VPN (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateL2VpnRequest {
    /// Updated L2VPN identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<i64>,
    /// Updated L2VPN name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated L2VPN slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated L2VPN type slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Updated status slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Updated import target IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_targets: Option<Vec<i32>>,
    /// Updated export target IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_targets: Option<Vec<i32>>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<i32>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating an L2VPN termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateL2VpnTerminationRequest {
    /// L2VPN ID.
    pub l2vpn: i32,
    /// Assigned object type string.
    pub assigned_object_type: String,
    /// Assigned object ID.
    pub assigned_object_id: i64,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an L2VPN termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateL2VpnTerminationRequest {
    /// Updated L2VPN ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l2vpn: Option<i32>,
    /// Updated assigned object type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_object_type: Option<String>,
    /// Updated assigned object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_object_id: Option<i64>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating an IKE proposal (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIkeProposalRequest {
    /// Proposal name.
    pub name: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Authentication method slug.
    pub authentication_method: String,
    /// Encryption algorithm slug.
    pub encryption_algorithm: String,
    /// Authentication algorithm slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_algorithm: Option<String>,
    /// Group slug.
    pub group: String,
    /// SA lifetime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa_lifetime: Option<i32>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an IKE proposal (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIkeProposalRequest {
    /// Updated proposal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated authentication method slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    /// Updated encryption algorithm slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// Updated authentication algorithm slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_algorithm: Option<String>,
    /// Updated group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Updated SA lifetime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa_lifetime: Option<i32>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating an IKE policy (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIkePolicyRequest {
    /// Policy name.
    pub name: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IKE version string.
    pub version: String,
    /// Mode slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Proposal IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<i32>>,
    /// Preshared key string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preshared_key: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an IKE policy (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIkePolicyRequest {
    /// Updated policy name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated IKE version string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Updated mode slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Updated proposal IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<i32>>,
    /// Updated preshared key string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preshared_key: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating an IPsec proposal (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpSecProposalRequest {
    /// Proposal name.
    pub name: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Encryption algorithm slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// Authentication algorithm slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_algorithm: Option<String>,
    /// SA lifetime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa_lifetime_seconds: Option<i32>,
    /// SA lifetime in kilobytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa_lifetime_data: Option<i32>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an IPsec proposal (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIpSecProposalRequest {
    /// Updated proposal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated encryption algorithm slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// Updated authentication algorithm slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_algorithm: Option<String>,
    /// Updated SA lifetime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa_lifetime_seconds: Option<i32>,
    /// Updated SA lifetime in kilobytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa_lifetime_data: Option<i32>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating an IPsec policy (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpSecPolicyRequest {
    /// Policy name.
    pub name: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Proposal IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<i32>>,
    /// PFS group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfs_group: Option<String>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an IPsec policy (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIpSecPolicyRequest {
    /// Updated policy name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated proposal IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<i32>>,
    /// Updated PFS group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfs_group: Option<String>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating an IPsec profile (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpSecProfileRequest {
    /// Profile name.
    pub name: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Mode slug.
    pub mode: String,
    /// IKE policy ID.
    pub ike_policy: i32,
    /// IPsec policy ID.
    pub ipsec_policy: i32,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating an IPsec profile (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIpSecProfileRequest {
    /// Updated profile name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated mode slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Updated IKE policy ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ike_policy: Option<i32>,
    /// Updated IPsec policy ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec_policy: Option<i32>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a tunnel termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTunnelTerminationRequest {
    /// Tunnel ID.
    pub tunnel: i32,
    /// Role slug.
    pub role: String,
    /// Termination type string.
    pub termination_type: String,
    /// Termination object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_id: Option<i64>,
    /// Outside IP address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_ip: Option<i32>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a tunnel termination (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTunnelTerminationRequest {
    /// Updated tunnel ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<i32>,
    /// Updated role slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Updated termination type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_type: Option<String>,
    /// Updated termination object ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_id: Option<i64>,
    /// Updated outside IP address ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_ip: Option<i32>,
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

    fn assert_optional_i64(value: &Value, key: &str, field: &Option<i64>) {
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
    fn serialize_tunnel_group_requests() {
        let create = CreateTunnelGroupRequest {
            name: "core".to_string(),
            slug: "core".to_string(),
            description: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "core");
        assert_eq!(value["slug"], "core");
        assert_missing(&value, "description");

        let update = UpdateTunnelGroupRequest {
            name: None,
            slug: Some("core-new".to_string()),
            description: Some("Updated".to_string()),
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["slug"], "core-new");
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_tunnel_requests() {
        let create = CreateTunnelRequest {
            name: "t1".to_string(),
            status: "active".to_string(),
            encapsulation: "gre".to_string(),
            group: Some(2),
            ipsec_profile: None,
            tenant: None,
            tunnel_id: None,
            description: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "t1");
        assert_eq!(value["status"], "active");
        assert_eq!(value["encapsulation"], "gre");
        assert_eq!(value["group"], 2);
        assert_missing(&value, "ipsec_profile");

        let update = UpdateTunnelRequest {
            name: None,
            status: Some("disabled".to_string()),
            encapsulation: None,
            group: None,
            ipsec_profile: None,
            tenant: None,
            tunnel_id: None,
            description: Some("Updated".to_string()),
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["status"], "disabled");
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_tunnel_termination_requests() {
        let create = CreateTunnelTerminationRequest {
            tunnel: 1,
            role: "peer".to_string(),
            termination_type: "dcim.interface".to_string(),
            termination_id: Some(5),
            outside_ip: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["tunnel"], 1);
        assert_eq!(value["role"], "peer");
        assert_eq!(value["termination_type"], "dcim.interface");
        assert_eq!(value["termination_id"], 5);
        assert_missing(&value, "outside_ip");

        let update = UpdateTunnelTerminationRequest {
            tunnel: None,
            role: None,
            termination_type: None,
            termination_id: None,
            outside_ip: Some(10),
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["outside_ip"], 10);
        assert_missing(&value, "tunnel");
    }

    #[test]
    fn serialize_l2vpn_requests() {
        let create = CreateL2VpnRequest {
            identifier: None,
            name: "l2vpn".to_string(),
            slug: "l2vpn".to_string(),
            r#type: Some("vxlan".to_string()),
            status: Some("active".to_string()),
            import_targets: None,
            export_targets: None,
            description: None,
            comments: None,
            tenant: None,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "l2vpn");
        assert_eq!(value["slug"], "l2vpn");
        assert_eq!(value["type"], "vxlan");
        assert_eq!(value["status"], "active");
        assert_missing(&value, "identifier");

        let update = UpdateL2VpnRequest {
            identifier: Some(100),
            name: None,
            slug: None,
            r#type: None,
            status: None,
            import_targets: None,
            export_targets: None,
            description: Some("Updated".to_string()),
            comments: None,
            tenant: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["identifier"], 100);
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_l2vpn_termination_requests() {
        let create = CreateL2VpnTerminationRequest {
            l2vpn: 1,
            assigned_object_type: "dcim.interface".to_string(),
            assigned_object_id: 2,
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["l2vpn"], 1);
        assert_eq!(value["assigned_object_type"], "dcim.interface");
        assert_eq!(value["assigned_object_id"], 2);
        assert_missing(&value, "tags");

        let update = UpdateL2VpnTerminationRequest {
            l2vpn: None,
            assigned_object_type: None,
            assigned_object_id: Some(3),
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["assigned_object_id"], 3);
        assert_missing(&value, "l2vpn");
    }

    #[test]
    fn serialize_ike_requests() {
        let proposal = CreateIkeProposalRequest {
            name: "ike-proposal".to_string(),
            description: None,
            authentication_method: "preshared-keys".to_string(),
            encryption_algorithm: "aes-256-gcm".to_string(),
            authentication_algorithm: Some("hmac-sha256".to_string()),
            group: "14".to_string(),
            sa_lifetime: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&proposal).unwrap();
        assert_eq!(value["authentication_method"], "preshared-keys");
        assert_eq!(value["encryption_algorithm"], "aes-256-gcm");
        assert_eq!(value["group"], "14");
        assert_missing(&value, "sa_lifetime");

        let policy = CreateIkePolicyRequest {
            name: "ike-policy".to_string(),
            description: None,
            version: "2".to_string(),
            mode: None,
            proposals: Some(vec![1]),
            preshared_key: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&policy).unwrap();
        assert_eq!(value["version"], "2");
        assert_eq!(value["proposals"], serde_json::json!([1]));
        assert_missing(&value, "mode");

        let update = UpdateIkeProposalRequest {
            name: None,
            description: Some("Updated".to_string()),
            authentication_method: None,
            encryption_algorithm: None,
            authentication_algorithm: None,
            group: None,
            sa_lifetime: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_ipsec_requests() {
        let proposal = CreateIpSecProposalRequest {
            name: "ipsec-proposal".to_string(),
            description: None,
            encryption_algorithm: Some("aes-128-gcm".to_string()),
            authentication_algorithm: None,
            sa_lifetime_seconds: Some(3600),
            sa_lifetime_data: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&proposal).unwrap();
        assert_eq!(value["encryption_algorithm"], "aes-128-gcm");
        assert_eq!(value["sa_lifetime_seconds"], 3600);
        assert_missing(&value, "authentication_algorithm");

        let policy = CreateIpSecPolicyRequest {
            name: "ipsec-policy".to_string(),
            description: None,
            proposals: Some(vec![2]),
            pfs_group: Some("14".to_string()),
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&policy).unwrap();
        assert_eq!(value["proposals"], serde_json::json!([2]));
        assert_eq!(value["pfs_group"], "14");
        assert_missing(&value, "description");

        let profile = CreateIpSecProfileRequest {
            name: "ipsec-profile".to_string(),
            description: None,
            mode: "esp".to_string(),
            ike_policy: 1,
            ipsec_policy: 2,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&profile).unwrap();
        assert_eq!(value["mode"], "esp");
        assert_eq!(value["ike_policy"], 1);
        assert_eq!(value["ipsec_policy"], 2);
        assert_missing(&value, "description");

        let update = UpdateIpSecProfileRequest {
            name: None,
            description: Some("Updated".to_string()),
            mode: None,
            ike_policy: None,
            ipsec_policy: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    proptest! {
        #[test]
        fn prop_tunnel_optional_fields(
            group in proptest::option::of(0i32..1000),
            ipsec_profile in proptest::option::of(0i32..1000),
        ) {
            let request = CreateTunnelRequest {
                name: "t1".to_string(),
                status: "active".to_string(),
                encapsulation: "gre".to_string(),
                group,
                ipsec_profile,
                tenant: None,
                tunnel_id: None,
                description: None,
                comments: None,
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_i32(&value, "group", &group);
            assert_optional_i32(&value, "ipsec_profile", &ipsec_profile);
        }
    }

    proptest! {
        #[test]
        fn prop_l2vpn_optional_fields(
            identifier in proptest::option::of(0i64..10000),
            r#type in proptest::option::of("[a-z0-9-]{1,12}"),
        ) {
            let request = CreateL2VpnRequest {
                identifier,
                name: "l2vpn".to_string(),
                slug: "l2vpn".to_string(),
                r#type: r#type.clone(),
                status: None,
                import_targets: None,
                export_targets: None,
                description: None,
                comments: None,
                tenant: None,
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_i64(&value, "identifier", &identifier);
            assert_optional_string(&value, "type", &r#type);
        }
    }

    proptest! {
        #[test]
        fn prop_ipsec_proposal_optional_fields(
            encryption in proptest::option::of("[a-z0-9-]{1,16}"),
            auth in proptest::option::of("[a-z0-9-]{1,16}"),
        ) {
            let request = CreateIpSecProposalRequest {
                name: "ipsec".to_string(),
                description: None,
                encryption_algorithm: encryption.clone(),
                authentication_algorithm: auth.clone(),
                sa_lifetime_seconds: None,
                sa_lifetime_data: None,
                comments: None,
                tags: None,
            };
            let value = serde_json::to_value(&request).unwrap();
            assert_optional_string(&value, "encryption_algorithm", &encryption);
            assert_optional_string(&value, "authentication_algorithm", &auth);
        }
    }
}

/// IKE policy model.
pub type IkePolicy = crate::models::IkePolicy;
/// IKE proposal model.
pub type IkeProposal = crate::models::IkeProposal;
/// IPsec policy model.
pub type IpSecPolicy = crate::models::IpSecPolicy;
/// IPsec profile model.
pub type IpSecProfile = crate::models::IpSecProfile;
/// IPsec proposal model.
pub type IpSecProposal = crate::models::IpSecProposal;
/// L2VPN model.
pub type L2Vpn = crate::models::L2Vpn;
/// L2VPN termination model.
pub type L2VpnTermination = crate::models::L2VpnTermination;
/// Tunnel group model.
pub type TunnelGroup = crate::models::TunnelGroup;
/// Tunnel termination model.
pub type TunnelTermination = crate::models::TunnelTermination;
/// Tunnel model.
pub type Tunnel = crate::models::Tunnel;

/// Resource for IKE policies.
pub type IkePoliciesApi = Resource<crate::models::IkePolicy>;
/// Resource for IKE proposals.
pub type IkeProposalsApi = Resource<crate::models::IkeProposal>;
/// Resource for IPsec policies.
pub type IpSecPoliciesApi = Resource<crate::models::IpSecPolicy>;
/// Resource for IPsec profiles.
pub type IpSecProfilesApi = Resource<crate::models::IpSecProfile>;
/// Resource for IPsec proposals.
pub type IpSecProposalsApi = Resource<crate::models::IpSecProposal>;
/// Resource for L2VPN terminations.
pub type L2VpnTerminationsApi = Resource<crate::models::L2VpnTermination>;
/// Resource for L2VPNs.
pub type L2VpnsApi = Resource<crate::models::L2Vpn>;
/// Resource for tunnel groups.
pub type TunnelGroupsApi = Resource<crate::models::TunnelGroup>;
/// Resource for tunnel terminations.
pub type TunnelTerminationsApi = Resource<crate::models::TunnelTermination>;
/// Resource for tunnels.
pub type TunnelsApi = Resource<crate::models::Tunnel>;

/// API for VPN endpoints.
#[derive(Clone)]
pub struct VpnApi {
    client: Client,
}

impl VpnApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the IKE policies resource.
    pub fn ike_policies(&self) -> IkePoliciesApi {
        Resource::new(self.client.clone(), "vpn/ike-policies/")
    }

    /// Returns the IKE proposals resource.
    pub fn ike_proposals(&self) -> IkeProposalsApi {
        Resource::new(self.client.clone(), "vpn/ike-proposals/")
    }

    /// Returns the IPsec policies resource.
    pub fn ipsec_policies(&self) -> IpSecPoliciesApi {
        Resource::new(self.client.clone(), "vpn/ipsec-policies/")
    }

    /// Returns the IPsec profiles resource.
    pub fn ipsec_profiles(&self) -> IpSecProfilesApi {
        Resource::new(self.client.clone(), "vpn/ipsec-profiles/")
    }

    /// Returns the IPsec proposals resource.
    pub fn ipsec_proposals(&self) -> IpSecProposalsApi {
        Resource::new(self.client.clone(), "vpn/ipsec-proposals/")
    }

    /// Returns the L2VPN terminations resource.
    pub fn l2vpn_terminations(&self) -> L2VpnTerminationsApi {
        Resource::new(self.client.clone(), "vpn/l2vpn-terminations/")
    }

    /// Returns the L2VPNs resource.
    pub fn l2vpns(&self) -> L2VpnsApi {
        Resource::new(self.client.clone(), "vpn/l2vpns/")
    }

    /// Returns the tunnel groups resource.
    pub fn tunnel_groups(&self) -> TunnelGroupsApi {
        Resource::new(self.client.clone(), "vpn/tunnel-groups/")
    }

    /// Returns the tunnel terminations resource.
    pub fn tunnel_terminations(&self) -> TunnelTerminationsApi {
        Resource::new(self.client.clone(), "vpn/tunnel-terminations/")
    }

    /// Returns the tunnels resource.
    pub fn tunnels(&self) -> TunnelsApi {
        Resource::new(self.client.clone(), "vpn/tunnels/")
    }
}
