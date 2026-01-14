//! tenancy endpoints for tenants, groups, and contacts.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let tenants = client.tenancy().tenants().list(None).await?;
//! println!("{}", tenants.count);
//! # Ok(())
//! # }
//! ```

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};

/// request for creating a tenant (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    /// tenant name.
    pub name: String,
    /// tenant slug.
    pub slug: String,
    /// tenant group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
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

/// request for updating a tenant (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    /// updated tenant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated tenant slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// updated tenant group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
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

/// request for creating a tenant group (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantGroupRequest {
    /// tenant group name.
    pub name: String,
    /// tenant group slug.
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
}

/// request for updating a tenant group (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTenantGroupRequest {
    /// updated tenant group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated tenant group slug.
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
}

/// request for creating a contact (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRequest {
    /// contact group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// contact name.
    pub name: String,
    /// contact title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// external link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
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

/// request for updating a contact (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    /// updated contact group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// updated contact name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated contact title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// updated phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// updated email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// updated street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// updated external link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
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

/// request for creating a contact group (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactGroupRequest {
    /// contact group name.
    pub name: String,
    /// contact group slug.
    pub slug: String,
    /// parent group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
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

/// request for updating a contact group (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactGroupRequest {
    /// updated contact group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated contact group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// updated parent group id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
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

/// request for creating a contact role (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRoleRequest {
    /// contact role name.
    pub name: String,
    /// contact role slug.
    pub slug: String,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a contact role (id-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRoleRequest {
    /// updated contact role name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated contact role slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// tenant model.
pub type Tenant = crate::models::Tenant;
/// tenant group model.
pub type TenantGroup = crate::models::TenantGroup;
/// contact model.
pub type Contact = crate::models::Contact;
/// contact group model.
pub type ContactGroup = crate::models::ContactGroup;
/// contact role model.
pub type ContactRole = crate::models::ContactRole;
/// contact assignment model.
pub type ContactAssignment = crate::models::ContactAssignment;

/// resource for contact assignments.
pub type ContactAssignmentsApi = Resource<crate::models::ContactAssignment>;
/// resource for contact groups.
pub type ContactGroupsApi = Resource<crate::models::ContactGroup>;
/// resource for contact roles.
pub type ContactRolesApi = Resource<crate::models::ContactRole>;
/// resource for contacts.
pub type ContactsApi = Resource<crate::models::Contact>;
/// resource for tenant groups.
pub type TenantGroupsApi = Resource<crate::models::TenantGroup>;
/// resource for tenants.
pub type TenantsApi = Resource<crate::models::Tenant>;

/// api for tenancy endpoints.
#[derive(Clone)]
pub struct TenancyApi {
    client: Client,
}

impl TenancyApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the contact assignments resource.
    pub fn contact_assignments(&self) -> ContactAssignmentsApi {
        Resource::new(self.client.clone(), "tenancy/contact-assignments/")
    }

    /// returns the contact groups resource.
    pub fn contact_groups(&self) -> ContactGroupsApi {
        Resource::new(self.client.clone(), "tenancy/contact-groups/")
    }

    /// returns the contact roles resource.
    pub fn contact_roles(&self) -> ContactRolesApi {
        Resource::new(self.client.clone(), "tenancy/contact-roles/")
    }

    /// returns the contacts resource.
    pub fn contacts(&self) -> ContactsApi {
        Resource::new(self.client.clone(), "tenancy/contacts/")
    }

    /// returns the tenant groups resource.
    pub fn tenant_groups(&self) -> TenantGroupsApi {
        Resource::new(self.client.clone(), "tenancy/tenant-groups/")
    }

    /// returns the tenants resource.
    pub fn tenants(&self) -> TenantsApi {
        Resource::new(self.client.clone(), "tenancy/tenants/")
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
        let paginator = resource.paginate(None).unwrap();
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn tenancy_accessors_return_expected_paths() {
        let api = TenancyApi::new(test_client());

        assert_path(api.contact_assignments(), "tenancy/contact-assignments/");
        assert_path(api.contact_groups(), "tenancy/contact-groups/");
        assert_path(api.contact_roles(), "tenancy/contact-roles/");
        assert_path(api.contacts(), "tenancy/contacts/");
        assert_path(api.tenant_groups(), "tenancy/tenant-groups/");
        assert_path(api.tenants(), "tenancy/tenants/");
    }

    #[test]
    fn serialize_tenancy_requests() {
        let tenant = CreateTenantRequest {
            name: "acme".to_string(),
            slug: "acme".to_string(),
            group: Some(1),
            description: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&tenant).unwrap();
        assert_eq!(value["name"], "acme");
        assert_eq!(value["slug"], "acme");
        assert_eq!(value["group"], 1);

        let contact = CreateContactRequest {
            groups: Some(vec![1, 2]),
            name: "Alice".to_string(),
            title: None,
            phone: None,
            email: None,
            address: None,
            link: None,
            description: None,
            comments: None,
            tags: None,
        };
        let value = serde_json::to_value(&contact).unwrap();
        assert_eq!(value["name"], "Alice");
        assert_eq!(value["groups"], serde_json::json!([1, 2]));
    }
}
