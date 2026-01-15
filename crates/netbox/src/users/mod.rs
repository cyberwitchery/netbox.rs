//! users, groups, tokens, and permissions.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let users = client.users().users().list(None).await?;
//! println!("{}", users.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// request for creating a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    /// group name.
    pub name: String,
    /// group description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// permission IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<i32>>,
}

/// request for updating a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    /// updated group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated group description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated permission IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<i32>>,
}

/// request for creating a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    /// user id.
    pub user: i32,
    /// expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// last used timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
    /// token key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// write enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    /// token description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// request for updating a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTokenRequest {
    /// updated user id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    /// updated expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// updated last used timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
    /// updated token key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// updated write enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    /// updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// request for creating a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// username.
    pub username: String,
    /// password.
    pub password: String,
    /// first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// staff flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    /// active flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// joined timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<String>,
    /// last login timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    /// group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// permission IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<i32>>,
}

/// request for updating a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    /// updated username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// updated password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// updated first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// updated last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// updated email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// updated staff flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_staff: Option<bool>,
    /// updated active flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// updated joined timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<String>,
    /// updated last login timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    /// updated group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// updated permission IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<i32>>,
}

/// request for creating an object permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateObjectPermissionRequest {
    /// permission name.
    pub name: String,
    /// object types (content type strings).
    pub object_types: Vec<String>,
    /// actions list.
    pub actions: Vec<String>,
    /// permission description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// constraints json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Value>,
    /// group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// user IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i32>>,
}

/// request for updating an object permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateObjectPermissionRequest {
    /// updated permission name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated object types (content type strings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
    /// updated actions list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated enabled flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// updated constraints json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Value>,
    /// updated group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// updated user IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i32>>,
}

/// user config model.
pub type UserConfig = HashMap<String, Value>;
/// group model.
pub type Group = crate::models::Group;
/// object permission model.
pub type ObjectPermission = crate::models::ObjectPermission;
/// token model.
pub type Token = crate::models::Token;
/// token provision model.
pub type TokenProvision = crate::models::TokenProvision;
/// token provision request model.
pub type TokenProvisionRequest = crate::models::TokenProvisionRequest;
/// user model.
pub type User = crate::models::User;

/// resource for groups.
pub type GroupsApi = Resource<crate::models::Group>;
/// resource for permissions.
pub type PermissionsApi = Resource<crate::models::ObjectPermission>;
/// resource for tokens.
pub type TokensApi = Resource<crate::models::Token>;
/// resource for users.
pub type UsersResource = Resource<crate::models::User>;

/// api for user endpoints.
#[derive(Clone)]
pub struct UsersApi {
    client: Client,
}

impl UsersApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// fetch the current user's config.
    pub async fn config(&self) -> Result<UserConfig> {
        self.client.get("users/config/").await
    }

    /// returns the groups resource.
    pub fn groups(&self) -> GroupsApi {
        Resource::new(self.client.clone(), "users/groups/")
    }

    /// returns the permissions resource.
    pub fn permissions(&self) -> PermissionsApi {
        Resource::new(self.client.clone(), "users/permissions/")
    }

    /// returns the tokens resource.
    pub fn tokens(&self) -> TokensApi {
        Resource::new(self.client.clone(), "users/tokens/")
    }

    /// provision a token using username/password credentials.
    pub async fn provision_token(&self, body: &TokenProvisionRequest) -> Result<TokenProvision> {
        self.client.post("users/tokens/provision/", body).await
    }

    /// returns the users resource.
    pub fn users(&self) -> UsersResource {
        Resource::new(self.client.clone(), "users/users/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::POST, MockServer};
    use serde_json::json;

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

    fn assert_missing(value: &serde_json::Value, key: &str) {
        assert!(value.get(key).is_none(), "expected {} to be omitted", key);
    }

    #[test]
    fn users_accessors_return_expected_paths() {
        let api = UsersApi::new(test_client());

        assert_path(api.groups(), "users/groups/");
        assert_path(api.permissions(), "users/permissions/");
        assert_path(api.tokens(), "users/tokens/");
        assert_path(api.users(), "users/users/");
    }

    #[test]
    fn serialize_group_requests() {
        let create = CreateGroupRequest {
            name: "operators".to_string(),
            description: None,
            permissions: Some(vec![1, 2]),
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "operators");
        assert_eq!(value["permissions"], json!([1, 2]));
        assert_missing(&value, "description");

        let update = UpdateGroupRequest {
            name: None,
            description: Some("Updated".to_string()),
            permissions: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_token_requests() {
        let create = CreateTokenRequest {
            user: 5,
            expires: Some("2030-01-01T00:00:00Z".to_string()),
            last_used: None,
            key: None,
            write_enabled: Some(true),
            description: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["user"], 5);
        assert_eq!(value["expires"], "2030-01-01T00:00:00Z");
        assert_eq!(value["write_enabled"], true);
        assert_missing(&value, "description");

        let update = UpdateTokenRequest {
            user: None,
            expires: None,
            last_used: Some("2024-01-01T00:00:00Z".to_string()),
            key: Some("abcd".to_string()),
            write_enabled: None,
            description: Some("Updated".to_string()),
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["last_used"], "2024-01-01T00:00:00Z");
        assert_eq!(value["key"], "abcd");
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "user");
    }

    #[test]
    fn serialize_user_requests() {
        let create = CreateUserRequest {
            username: "alice".to_string(),
            password: "secret".to_string(),
            first_name: Some("Alice".to_string()),
            last_name: None,
            email: Some("alice@example.com".to_string()),
            is_staff: Some(true),
            is_active: Some(true),
            date_joined: None,
            last_login: None,
            groups: Some(vec![1]),
            permissions: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["username"], "alice");
        assert_eq!(value["password"], "secret");
        assert_eq!(value["first_name"], "Alice");
        assert_eq!(value["email"], "alice@example.com");
        assert_eq!(value["is_staff"], true);
        assert_eq!(value["groups"], json!([1]));
        assert_missing(&value, "last_name");

        let update = UpdateUserRequest {
            username: None,
            password: None,
            first_name: None,
            last_name: Some("Smith".to_string()),
            email: None,
            is_staff: None,
            is_active: Some(false),
            date_joined: None,
            last_login: None,
            groups: None,
            permissions: Some(vec![9]),
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["last_name"], "Smith");
        assert_eq!(value["is_active"], false);
        assert_eq!(value["permissions"], json!([9]));
        assert_missing(&value, "username");
    }

    #[test]
    fn serialize_object_permission_requests() {
        let create = CreateObjectPermissionRequest {
            name: "dcim_view".to_string(),
            object_types: vec!["dcim.device".to_string()],
            actions: vec!["view".to_string()],
            description: None,
            enabled: Some(true),
            constraints: Some(json!({"status": "active"})),
            groups: None,
            users: Some(vec![2]),
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "dcim_view");
        assert_eq!(value["object_types"], json!(["dcim.device"]));
        assert_eq!(value["actions"], json!(["view"]));
        assert_eq!(value["enabled"], true);
        assert_eq!(value["constraints"]["status"], "active");
        assert_eq!(value["users"], json!([2]));

        let update = UpdateObjectPermissionRequest {
            name: None,
            object_types: None,
            actions: None,
            description: Some("Updated".to_string()),
            enabled: None,
            constraints: None,
            groups: Some(vec![1]),
            users: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_eq!(value["groups"], json!([1]));
        assert_missing(&value, "name");
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn provision_token_uses_expected_path() {
        let server = MockServer::start();
        let base_url = server.base_url();
        let config = ClientConfig::new(&base_url, "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = UsersApi::new(client);

        let response = json!({
            "id": 1,
            "url": "http://example.com/api/users/tokens/1/",
            "display_url": "http://example.com/users/tokens/1/",
            "display": "token",
            "user": {
                "id": 1,
                "url": "http://example.com/api/users/users/1/",
                "display": "admin",
                "username": "admin"
            },
            "created": "2024-01-01T00:00:00Z",
            "expires": null,
            "last_used": "2024-01-01T00:00:00Z",
            "key": "token",
            "write_enabled": true,
            "description": "provisioned"
        });

        server.mock(|when, then| {
            when.method(POST).path("/api/users/tokens/provision/");
            then.status(201).json_body(response);
        });

        let request = TokenProvisionRequest {
            expires: None,
            write_enabled: Some(true),
            description: Some("provisioned".to_string()),
            username: "admin".to_string(),
            password: "secret".to_string(),
        };

        let token = api.provision_token(&request).await.unwrap();
        assert_eq!(token.key.as_deref(), Some("token"));
    }
}
