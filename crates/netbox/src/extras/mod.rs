//! Extras API endpoints.

use crate::error::Result;
use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Request for creating a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    /// Tag name.
    pub name: String,
    /// Tag slug.
    pub slug: String,
    /// Tag color in hex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Tag description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Object types (content type strings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
}

/// Request for updating a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    /// Updated tag name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated tag slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Updated tag color in hex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Updated tag description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated tag weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Updated object types (content type strings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
}

/// Request for creating a custom field choice set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomFieldChoiceSetRequest {
    /// Choice set name.
    pub name: String,
    /// Choice set description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Base choices identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_choices: Option<String>,
    /// Extra choices payload.
    pub extra_choices: Vec<Vec<Value>>,
    /// Order choices alphabetically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_alphabetically: Option<bool>,
}

/// Request for updating a custom field choice set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomFieldChoiceSetRequest {
    /// Updated choice set name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated choice set description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated base choices identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_choices: Option<String>,
    /// Updated extra choices payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_choices: Option<Vec<Vec<Value>>>,
    /// Updated ordering flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_alphabetically: Option<bool>,
}

/// Request for creating a custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomFieldRequest {
    /// Object types (content type strings).
    pub object_types: Vec<String>,
    /// Field type identifier.
    pub r#type: String,
    /// Internal field name.
    pub name: String,
    /// Related object content type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_type: Option<String>,
    /// Field label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Field group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Field description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Unique flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    /// Search weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_weight: Option<i32>,
    /// Filter logic identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_logic: Option<String>,
    /// UI visibility identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_visible: Option<String>,
    /// UI editable identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_editable: Option<String>,
    /// Cloneable flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cloneable: Option<bool>,
    /// Default JSON value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    /// Related object filter JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_filter: Option<Value>,
    /// Field weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Validation minimum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_minimum: Option<f64>,
    /// Validation maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_maximum: Option<f64>,
    /// Validation regex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_regex: Option<String>,
    /// Choice set ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_set: Option<i32>,
    /// Field comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Request for updating a custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomFieldRequest {
    /// Updated object types (content type strings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
    /// Updated field type identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Updated internal field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated related object content type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_type: Option<String>,
    /// Updated field label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Updated field group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Updated field description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated required flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Updated unique flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    /// Updated search weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_weight: Option<i32>,
    /// Updated filter logic identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_logic: Option<String>,
    /// Updated UI visibility identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_visible: Option<String>,
    /// Updated UI editable identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_editable: Option<String>,
    /// Updated cloneable flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cloneable: Option<bool>,
    /// Updated default JSON value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    /// Updated related object filter JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_filter: Option<Value>,
    /// Updated field weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Updated validation minimum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_minimum: Option<f64>,
    /// Updated validation maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_maximum: Option<f64>,
    /// Updated validation regex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_regex: Option<String>,
    /// Updated choice set ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_set: Option<i32>,
    /// Updated comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Request for creating a config context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConfigContextRequest {
    /// Config context name.
    pub name: String,
    /// Weight value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Profile ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<i32>,
    /// Description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Active flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Region IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<i32>>,
    /// Site group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_groups: Option<Vec<i32>>,
    /// Site IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<i32>>,
    /// Location IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<i32>>,
    /// Device type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<Vec<i32>>,
    /// Role IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<i32>>,
    /// Platform IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<i32>>,
    /// Cluster type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<i32>>,
    /// Cluster group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_groups: Option<Vec<i32>>,
    /// Cluster IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<i32>>,
    /// Tenant group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_groups: Option<Vec<i32>>,
    /// Tenant IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<i32>>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Data source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// Data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Request for updating a config context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigContextRequest {
    /// Updated config context name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated weight value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Updated profile ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<i32>,
    /// Updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated active flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Updated region IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<i32>>,
    /// Updated site group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_groups: Option<Vec<i32>>,
    /// Updated site IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<i32>>,
    /// Updated location IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<i32>>,
    /// Updated device type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<Vec<i32>>,
    /// Updated role IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<i32>>,
    /// Updated platform IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<i32>>,
    /// Updated cluster type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<i32>>,
    /// Updated cluster group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_groups: Option<Vec<i32>>,
    /// Updated cluster IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<i32>>,
    /// Updated tenant group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_groups: Option<Vec<i32>>,
    /// Updated tenant IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<i32>>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Updated data source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// Updated data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Request for creating a config context profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConfigContextProfileRequest {
    /// Profile name.
    pub name: String,
    /// Profile description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Profile schema payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Data source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
}

/// Request for updating a config context profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigContextProfileRequest {
    /// Updated profile name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated profile description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated profile schema payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Updated data source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
}

/// Request for creating a config template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConfigTemplateRequest {
    /// Template name.
    pub name: String,
    /// Template code (Jinja2).
    pub template_code: String,
    /// Template description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Jinja environment params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_params: Option<Value>,
    /// MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// File extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// Attachment flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_attachment: Option<bool>,
    /// Data source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a config template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigTemplateRequest {
    /// Updated template name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated template code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// Updated template description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated Jinja environment params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_params: Option<Value>,
    /// Updated MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Updated file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Updated file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// Updated attachment flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_attachment: Option<bool>,
    /// Updated data source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for creating a webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    /// Webhook name.
    pub name: String,
    /// Payload URL.
    pub payload_url: String,
    /// Webhook description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// HTTP method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// HTTP content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_content_type: Option<String>,
    /// Additional headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_headers: Option<String>,
    /// Body template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// Secret string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// SSL verification flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_verification: Option<bool>,
    /// CA file path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_file_path: Option<String>,
    /// Custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// Tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Request for updating a webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWebhookRequest {
    /// Updated webhook name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated payload URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_url: Option<String>,
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated HTTP method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// Updated HTTP content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_content_type: Option<String>,
    /// Updated additional headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_headers: Option<String>,
    /// Updated body template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// Updated secret string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Updated SSL verification flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_verification: Option<bool>,
    /// Updated CA file path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_file_path: Option<String>,
    /// Updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// Updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// Bookmark model.
pub type Bookmark = crate::models::Bookmark;
/// Config context profile model.
pub type ConfigContextProfile = crate::models::ConfigContextProfile;
/// Config context model.
pub type ConfigContext = crate::models::ConfigContext;
/// Config template model.
pub type ConfigTemplate = crate::models::ConfigTemplate;
/// Custom field choice set model.
pub type CustomFieldChoiceSet = crate::models::CustomFieldChoiceSet;
/// Custom field model.
pub type CustomField = crate::models::CustomField;
/// Custom link model.
pub type CustomLink = crate::models::CustomLink;
/// Dashboard model.
pub type Dashboard = crate::models::Dashboard;
/// Dashboard request model.
pub type DashboardRequest = crate::models::DashboardRequest;
/// Patched dashboard request model.
pub type PatchedDashboardRequest = crate::models::PatchedDashboardRequest;
/// Event rule model.
pub type EventRule = crate::models::EventRule;
/// Export template model.
pub type ExportTemplate = crate::models::ExportTemplate;
/// Image attachment model.
pub type ImageAttachment = crate::models::ImageAttachment;
/// Journal entry model.
pub type JournalEntry = crate::models::JournalEntry;
/// Notification group model.
pub type NotificationGroup = crate::models::NotificationGroup;
/// Notification model.
pub type Notification = crate::models::Notification;
/// Object type model.
pub type ObjectType = crate::models::ObjectType;
/// Saved filter model.
pub type SavedFilter = crate::models::SavedFilter;
/// Script model.
pub type Script = crate::models::Script;
/// Subscription model.
pub type Subscription = crate::models::Subscription;
/// Table config model.
pub type TableConfig = crate::models::TableConfig;
/// Tagged item model.
pub type TaggedItem = crate::models::TaggedItem;
/// Tag model.
pub type Tag = crate::models::Tag;
/// Webhook model.
pub type Webhook = crate::models::Webhook;

/// Resource for bookmarks.
pub type BookmarksApi = Resource<crate::models::Bookmark>;
/// Resource for config context profiles.
pub type ConfigContextProfilesApi = Resource<crate::models::ConfigContextProfile>;
/// Resource for config contexts.
pub type ConfigContextsApi = Resource<crate::models::ConfigContext>;
/// Resource for config templates.
pub type ConfigTemplatesApi = Resource<crate::models::ConfigTemplate>;
/// Resource for custom field choice sets.
pub type CustomFieldChoiceSetsApi = Resource<crate::models::CustomFieldChoiceSet>;
/// Resource for custom fields.
pub type CustomFieldsApi = Resource<crate::models::CustomField>;
/// Resource for custom links.
pub type CustomLinksApi = Resource<crate::models::CustomLink>;
/// Resource for event rules.
pub type EventRulesApi = Resource<crate::models::EventRule>;
/// Resource for export templates.
pub type ExportTemplatesApi = Resource<crate::models::ExportTemplate>;
/// Resource for image attachments.
pub type ImageAttachmentsApi = Resource<crate::models::ImageAttachment>;
/// Resource for journal entries.
pub type JournalEntriesApi = Resource<crate::models::JournalEntry>;
/// Resource for notification groups.
pub type NotificationGroupsApi = Resource<crate::models::NotificationGroup>;
/// Resource for notifications.
pub type NotificationsApi = Resource<crate::models::Notification>;
/// Resource for object types.
pub type ObjectTypesApi = Resource<crate::models::ObjectType>;
/// Resource for saved filters.
pub type SavedFiltersApi = Resource<crate::models::SavedFilter>;
/// Resource for scripts.
pub type ScriptsApi = Resource<crate::models::Script>;
/// Resource for subscriptions.
pub type SubscriptionsApi = Resource<crate::models::Subscription>;
/// Resource for table configs.
pub type TableConfigsApi = Resource<crate::models::TableConfig>;
/// Resource for tagged objects.
pub type TaggedObjectsApi = Resource<crate::models::TaggedItem>;
/// Resource for tags.
pub type TagsApi = Resource<crate::models::Tag>;
/// Resource for webhooks.
pub type WebhooksApi = Resource<crate::models::Webhook>;

/// API for extras endpoints.
#[derive(Clone)]
pub struct ExtrasApi {
    client: Client,
}

impl ExtrasApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the bookmarks resource.
    pub fn bookmarks(&self) -> BookmarksApi {
        Resource::new(self.client.clone(), "extras/bookmarks/")
    }

    /// Returns the config context profiles resource.
    pub fn config_context_profiles(&self) -> ConfigContextProfilesApi {
        Resource::new(self.client.clone(), "extras/config-context-profiles/")
    }

    /// Returns the config contexts resource.
    pub fn config_contexts(&self) -> ConfigContextsApi {
        Resource::new(self.client.clone(), "extras/config-contexts/")
    }

    /// Returns the config templates resource.
    pub fn config_templates(&self) -> ConfigTemplatesApi {
        Resource::new(self.client.clone(), "extras/config-templates/")
    }

    /// Returns the custom field choice sets resource.
    pub fn custom_field_choice_sets(&self) -> CustomFieldChoiceSetsApi {
        Resource::new(self.client.clone(), "extras/custom-field-choice-sets/")
    }

    /// Returns the custom fields resource.
    pub fn custom_fields(&self) -> CustomFieldsApi {
        Resource::new(self.client.clone(), "extras/custom-fields/")
    }

    /// Returns the custom links resource.
    pub fn custom_links(&self) -> CustomLinksApi {
        Resource::new(self.client.clone(), "extras/custom-links/")
    }

    /// Fetch the current dashboard configuration.
    pub async fn dashboard(&self) -> Result<Dashboard> {
        self.client.get("extras/dashboard/").await
    }

    /// Update the dashboard configuration (full update).
    pub async fn update_dashboard(&self, body: &DashboardRequest) -> Result<Dashboard> {
        self.client.put("extras/dashboard/", body).await
    }

    /// Update the dashboard configuration (partial update).
    pub async fn patch_dashboard(&self, body: &PatchedDashboardRequest) -> Result<Dashboard> {
        self.client.patch("extras/dashboard/", body).await
    }

    /// Delete the current dashboard configuration.
    pub async fn delete_dashboard(&self) -> Result<()> {
        self.client.delete("extras/dashboard/").await
    }

    /// Returns the event rules resource.
    pub fn event_rules(&self) -> EventRulesApi {
        Resource::new(self.client.clone(), "extras/event-rules/")
    }

    /// Returns the export templates resource.
    pub fn export_templates(&self) -> ExportTemplatesApi {
        Resource::new(self.client.clone(), "extras/export-templates/")
    }

    /// Returns the image attachments resource.
    pub fn image_attachments(&self) -> ImageAttachmentsApi {
        Resource::new(self.client.clone(), "extras/image-attachments/")
    }

    /// Returns the journal entries resource.
    pub fn journal_entries(&self) -> JournalEntriesApi {
        Resource::new(self.client.clone(), "extras/journal-entries/")
    }

    /// Returns the notification groups resource.
    pub fn notification_groups(&self) -> NotificationGroupsApi {
        Resource::new(self.client.clone(), "extras/notification-groups/")
    }

    /// Returns the notifications resource.
    pub fn notifications(&self) -> NotificationsApi {
        Resource::new(self.client.clone(), "extras/notifications/")
    }

    /// Returns the object types resource.
    pub fn object_types(&self) -> ObjectTypesApi {
        Resource::new(self.client.clone(), "extras/object-types/")
    }

    /// Returns the saved filters resource.
    pub fn saved_filters(&self) -> SavedFiltersApi {
        Resource::new(self.client.clone(), "extras/saved-filters/")
    }

    /// Returns the scripts resource.
    pub fn scripts(&self) -> ScriptsApi {
        Resource::new(self.client.clone(), "extras/scripts/")
    }

    /// Returns the subscriptions resource.
    pub fn subscriptions(&self) -> SubscriptionsApi {
        Resource::new(self.client.clone(), "extras/subscriptions/")
    }

    /// Returns the table configs resource.
    pub fn table_configs(&self) -> TableConfigsApi {
        Resource::new(self.client.clone(), "extras/table-configs/")
    }

    /// Returns the tagged objects resource.
    pub fn tagged_objects(&self) -> TaggedObjectsApi {
        Resource::new(self.client.clone(), "extras/tagged-objects/")
    }

    /// Returns the tags resource.
    pub fn tags(&self) -> TagsApi {
        Resource::new(self.client.clone(), "extras/tags/")
    }

    /// Returns the webhooks resource.
    pub fn webhooks(&self) -> WebhooksApi {
        Resource::new(self.client.clone(), "extras/webhooks/")
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
    fn extras_accessors_return_expected_paths() {
        let api = ExtrasApi::new(test_client());

        assert_path(api.bookmarks(), "extras/bookmarks/");
        assert_path(api.config_context_profiles(), "extras/config-context-profiles/");
        assert_path(api.config_contexts(), "extras/config-contexts/");
        assert_path(api.config_templates(), "extras/config-templates/");
        assert_path(api.custom_field_choice_sets(), "extras/custom-field-choice-sets/");
        assert_path(api.custom_fields(), "extras/custom-fields/");
        assert_path(api.custom_links(), "extras/custom-links/");
        assert_path(api.event_rules(), "extras/event-rules/");
        assert_path(api.export_templates(), "extras/export-templates/");
        assert_path(api.image_attachments(), "extras/image-attachments/");
        assert_path(api.journal_entries(), "extras/journal-entries/");
        assert_path(api.notification_groups(), "extras/notification-groups/");
        assert_path(api.notifications(), "extras/notifications/");
        assert_path(api.object_types(), "extras/object-types/");
        assert_path(api.saved_filters(), "extras/saved-filters/");
        assert_path(api.scripts(), "extras/scripts/");
        assert_path(api.subscriptions(), "extras/subscriptions/");
        assert_path(api.table_configs(), "extras/table-configs/");
        assert_path(api.tagged_objects(), "extras/tagged-objects/");
        assert_path(api.tags(), "extras/tags/");
        assert_path(api.webhooks(), "extras/webhooks/");
    }

    #[test]
    fn serialize_tag_requests() {
        let create = CreateTagRequest {
            name: "critical".to_string(),
            slug: "critical".to_string(),
            color: Some("ff0000".to_string()),
            description: None,
            weight: None,
            object_types: Some(vec!["dcim.device".to_string()]),
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "critical");
        assert_eq!(value["slug"], "critical");
        assert_eq!(value["color"], "ff0000");
        assert_eq!(value["object_types"], json!(["dcim.device"]));
        assert_missing(&value, "description");

        let update = UpdateTagRequest {
            name: None,
            slug: Some("critical-updated".to_string()),
            color: None,
            description: Some("Updated".to_string()),
            weight: Some(10),
            object_types: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["slug"], "critical-updated");
        assert_eq!(value["description"], "Updated");
        assert_eq!(value["weight"], 10);
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_custom_field_choice_set_requests() {
        let create = CreateCustomFieldChoiceSetRequest {
            name: "regions".to_string(),
            description: None,
            base_choices: Some("ISO_3166".to_string()),
            extra_choices: vec![vec![json!("NA"), json!("North America")]],
            order_alphabetically: Some(true),
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "regions");
        assert_eq!(value["base_choices"], "ISO_3166");
        assert_eq!(value["order_alphabetically"], true);
        assert_missing(&value, "description");

        let update = UpdateCustomFieldChoiceSetRequest {
            name: None,
            description: Some("Updated".to_string()),
            base_choices: None,
            extra_choices: None,
            order_alphabetically: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_custom_field_requests() {
        let create = CreateCustomFieldRequest {
            object_types: vec!["dcim.device".to_string()],
            r#type: "text".to_string(),
            name: "asset_code".to_string(),
            related_object_type: None,
            label: Some("Asset Code".to_string()),
            group_name: None,
            description: None,
            required: Some(true),
            unique: None,
            search_weight: None,
            filter_logic: None,
            ui_visible: None,
            ui_editable: None,
            is_cloneable: None,
            default: None,
            related_object_filter: None,
            weight: None,
            validation_minimum: None,
            validation_maximum: None,
            validation_regex: None,
            choice_set: Some(3),
            comments: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["object_types"], json!(["dcim.device"]));
        assert_eq!(value["type"], "text");
        assert_eq!(value["name"], "asset_code");
        assert_eq!(value["label"], "Asset Code");
        assert_eq!(value["required"], true);
        assert_eq!(value["choice_set"], 3);
        assert_missing(&value, "group_name");

        let update = UpdateCustomFieldRequest {
            object_types: None,
            r#type: None,
            name: None,
            related_object_type: None,
            label: None,
            group_name: Some("Ops".to_string()),
            description: Some("Updated".to_string()),
            required: None,
            unique: None,
            search_weight: None,
            filter_logic: Some("exact".to_string()),
            ui_visible: None,
            ui_editable: None,
            is_cloneable: None,
            default: None,
            related_object_filter: None,
            weight: None,
            validation_minimum: None,
            validation_maximum: None,
            validation_regex: None,
            choice_set: None,
            comments: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["group_name"], "Ops");
        assert_eq!(value["description"], "Updated");
        assert_eq!(value["filter_logic"], "exact");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_config_context_requests() {
        let create = CreateConfigContextRequest {
            name: "base".to_string(),
            weight: None,
            profile: None,
            description: None,
            is_active: Some(true),
            regions: Some(vec![1, 2]),
            site_groups: None,
            sites: None,
            locations: None,
            device_types: None,
            roles: None,
            platforms: None,
            cluster_types: None,
            cluster_groups: None,
            clusters: None,
            tenant_groups: None,
            tenants: None,
            tags: Some(vec!["codex-smoke".to_string()]),
            data_source: Some(5),
            data: Some(json!({"env": "prod"})),
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "base");
        assert_eq!(value["is_active"], true);
        assert_eq!(value["regions"], json!([1, 2]));
        assert_eq!(value["tags"], json!(["codex-smoke"]));
        assert_eq!(value["data_source"], 5);
        assert_eq!(value["data"]["env"], "prod");
        assert_missing(&value, "profile");

        let update = UpdateConfigContextRequest {
            name: None,
            weight: Some(100),
            profile: Some(2),
            description: Some("Updated".to_string()),
            is_active: None,
            regions: None,
            site_groups: None,
            sites: None,
            locations: None,
            device_types: None,
            roles: None,
            platforms: None,
            cluster_types: None,
            cluster_groups: None,
            clusters: None,
            tenant_groups: None,
            tenants: None,
            tags: None,
            data_source: None,
            data: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["weight"], 100);
        assert_eq!(value["profile"], 2);
        assert_eq!(value["description"], "Updated");
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_config_context_profile_requests() {
        let create = CreateConfigContextProfileRequest {
            name: "defaults".to_string(),
            description: None,
            schema: Some(json!({"type": "object"})),
            tags: Some(vec!["codex-smoke".to_string(), "ops".to_string()]),
            comments: None,
            data_source: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "defaults");
        assert_eq!(value["schema"]["type"], "object");
        assert_eq!(value["tags"], json!(["codex-smoke", "ops"]));
        assert_missing(&value, "description");

        let update = UpdateConfigContextProfileRequest {
            name: None,
            description: Some("Updated".to_string()),
            schema: None,
            tags: None,
            comments: None,
            data_source: Some(7),
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_eq!(value["data_source"], 7);
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_config_template_requests() {
        let create = CreateConfigTemplateRequest {
            name: "basic".to_string(),
            template_code: "hostname {{ device.name }}".to_string(),
            description: None,
            environment_params: Some(json!({"trim_blocks": true})),
            mime_type: None,
            file_name: None,
            file_extension: None,
            as_attachment: Some(true),
            data_source: Some(4),
            tags: None,
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "basic");
        assert_eq!(value["template_code"], "hostname {{ device.name }}");
        assert_eq!(value["environment_params"]["trim_blocks"], true);
        assert_eq!(value["as_attachment"], true);
        assert_eq!(value["data_source"], 4);
        assert_missing(&value, "mime_type");

        let update = UpdateConfigTemplateRequest {
            name: None,
            template_code: None,
            description: Some("Updated".to_string()),
            environment_params: None,
            mime_type: None,
            file_name: None,
            file_extension: Some("cfg".to_string()),
            as_attachment: None,
            data_source: None,
            tags: Some(vec![3]),
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_eq!(value["file_extension"], "cfg");
        assert_eq!(value["tags"], json!([3]));
        assert_missing(&value, "name");
    }

    #[test]
    fn serialize_webhook_requests() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert("service".to_string(), json!("billing"));
        let create = CreateWebhookRequest {
            name: "audit".to_string(),
            payload_url: "https://hooks.example.com/audit".to_string(),
            description: None,
            http_method: Some("POST".to_string()),
            http_content_type: Some("application/json".to_string()),
            additional_headers: None,
            body_template: None,
            secret: None,
            ssl_verification: Some(true),
            ca_file_path: None,
            custom_fields: Some(custom_fields),
            tags: Some(vec![1]),
        };
        let value = serde_json::to_value(&create).unwrap();
        assert_eq!(value["name"], "audit");
        assert_eq!(value["payload_url"], "https://hooks.example.com/audit");
        assert_eq!(value["http_method"], "POST");
        assert_eq!(value["http_content_type"], "application/json");
        assert_eq!(value["ssl_verification"], true);
        assert_eq!(value["custom_fields"]["service"], "billing");
        assert_eq!(value["tags"], json!([1]));

        let update = UpdateWebhookRequest {
            name: None,
            payload_url: None,
            description: Some("Updated".to_string()),
            http_method: None,
            http_content_type: None,
            additional_headers: None,
            body_template: None,
            secret: None,
            ssl_verification: Some(false),
            ca_file_path: Some("/etc/ssl/ca.pem".to_string()),
            custom_fields: None,
            tags: None,
        };
        let value = serde_json::to_value(&update).unwrap();
        assert_eq!(value["description"], "Updated");
        assert_eq!(value["ssl_verification"], false);
        assert_eq!(value["ca_file_path"], "/etc/ssl/ca.pem");
        assert_missing(&value, "name");
    }
}
