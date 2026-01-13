//! Tenancy API endpoints.

use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};

/// Request for creating a tenant (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    /// Tenant name.
    pub name: String,
    /// Tenant slug.
    pub slug: String,
    /// Tenant group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
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

/// Request for updating a tenant (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    /// Updated tenant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated tenant slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated tenant group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i32>,
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

/// Request for creating a tenant group (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantGroupRequest {
    /// Tenant group name.
    pub name: String,
    /// Tenant group slug.
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
}

/// Request for updating a tenant group (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTenantGroupRequest {
    /// Updated tenant group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated tenant group slug.
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
}

/// Request for creating a contact (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRequest {
    /// Contact group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// Contact name.
    pub name: String,
    /// Contact title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// External link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
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

/// Request for updating a contact (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    /// Updated contact group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<i32>>,
    /// Updated contact name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated contact title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Updated phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Updated email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Updated street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Updated external link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
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

/// Request for creating a contact group (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactGroupRequest {
    /// Contact group name.
    pub name: String,
    /// Contact group slug.
    pub slug: String,
    /// Parent group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
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

/// Request for updating a contact group (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactGroupRequest {
    /// Updated contact group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated contact group slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated parent group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i32>,
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

/// Request for creating a contact role (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRoleRequest {
    /// Contact role name.
    pub name: String,
    /// Contact role slug.
    pub slug: String,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a contact role (ID-based references).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateContactRoleRequest {
    /// Updated contact role name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated contact role slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Tenant model.
pub type Tenant = crate::models::Tenant;
/// Tenant group model.
pub type TenantGroup = crate::models::TenantGroup;
/// Contact model.
pub type Contact = crate::models::Contact;
/// Contact group model.
pub type ContactGroup = crate::models::ContactGroup;
/// Contact role model.
pub type ContactRole = crate::models::ContactRole;
/// Contact assignment model.
pub type ContactAssignment = crate::models::ContactAssignment;

/// Resource for contact assignments.
pub type ContactAssignmentsApi = Resource<crate::models::ContactAssignment>;
/// Resource for contact groups.
pub type ContactGroupsApi = Resource<crate::models::ContactGroup>;
/// Resource for contact roles.
pub type ContactRolesApi = Resource<crate::models::ContactRole>;
/// Resource for contacts.
pub type ContactsApi = Resource<crate::models::Contact>;
/// Resource for tenant groups.
pub type TenantGroupsApi = Resource<crate::models::TenantGroup>;
/// Resource for tenants.
pub type TenantsApi = Resource<crate::models::Tenant>;

/// API for tenancy endpoints.
#[derive(Clone)]
pub struct TenancyApi {
    client: Client,
}

impl TenancyApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the contact assignments resource.
    pub fn contact_assignments(&self) -> ContactAssignmentsApi {
        Resource::new(self.client.clone(), "tenancy/contact-assignments/")
    }

    /// Returns the contact groups resource.
    pub fn contact_groups(&self) -> ContactGroupsApi {
        Resource::new(self.client.clone(), "tenancy/contact-groups/")
    }

    /// Returns the contact roles resource.
    pub fn contact_roles(&self) -> ContactRolesApi {
        Resource::new(self.client.clone(), "tenancy/contact-roles/")
    }

    /// Returns the contacts resource.
    pub fn contacts(&self) -> ContactsApi {
        Resource::new(self.client.clone(), "tenancy/contacts/")
    }

    /// Returns the tenant groups resource.
    pub fn tenant_groups(&self) -> TenantGroupsApi {
        Resource::new(self.client.clone(), "tenancy/tenant-groups/")
    }

    /// Returns the tenants resource.
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
        let paginator = resource.paginate(None);
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn tenancy_accessors_return_expected_paths() {
        let api = TenancyApi::new(test_client());

        assert_path(
            api.contact_assignments(),
            "tenancy/contact-assignments/",
        );
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
