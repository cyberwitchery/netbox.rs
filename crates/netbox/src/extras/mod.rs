//! extras endpoints for tags, webhooks, scripts, and custom fields.
//!
//! basic usage:
//! ```no_run
//! # use netbox::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://netbox.example.com", "token"))?;
//! let tags = client.extras().tags().list(None).await?;
//! println!("{}", tags.count);
//! # Ok(())
//! # }
//! ```

use crate::error::Result;
use crate::resource::Resource;
use crate::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// request for creating a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    /// tag name.
    pub name: String,
    /// tag slug.
    pub slug: String,
    /// tag color in hex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// tag description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// tag weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// object types (content type strings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
}

/// request for updating a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    /// updated tag name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated tag slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// updated tag color in hex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// updated tag description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated tag weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// updated object types (content type strings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
}

/// request for creating a custom field choice set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomFieldChoiceSetRequest {
    /// choice set name.
    pub name: String,
    /// choice set description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// base choices identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_choices: Option<String>,
    /// extra choices payload.
    pub extra_choices: Vec<Vec<Value>>,
    /// order choices alphabetically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_alphabetically: Option<bool>,
}

/// request for updating a custom field choice set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomFieldChoiceSetRequest {
    /// updated choice set name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated choice set description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated base choices identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_choices: Option<String>,
    /// updated extra choices payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_choices: Option<Vec<Vec<Value>>>,
    /// updated ordering flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_alphabetically: Option<bool>,
}

/// request for creating a custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomFieldRequest {
    /// object types (content type strings).
    pub object_types: Vec<String>,
    /// field type identifier.
    pub r#type: String,
    /// internal field name.
    pub name: String,
    /// related object content type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_type: Option<String>,
    /// field label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// field group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// field description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// required flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// unique flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    /// search weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_weight: Option<i32>,
    /// filter logic identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_logic: Option<String>,
    /// uI visibility identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_visible: Option<String>,
    /// uI editable identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_editable: Option<String>,
    /// cloneable flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cloneable: Option<bool>,
    /// default json value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    /// related object filter json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_filter: Option<Value>,
    /// field weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// validation minimum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_minimum: Option<f64>,
    /// validation maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_maximum: Option<f64>,
    /// validation regex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_regex: Option<String>,
    /// choice set id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_set: Option<i32>,
    /// field comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// request for updating a custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomFieldRequest {
    /// updated object types (content type strings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_types: Option<Vec<String>>,
    /// updated field type identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// updated internal field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated related object content type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_type: Option<String>,
    /// updated field label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// updated field group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// updated field description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated required flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// updated unique flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    /// updated search weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_weight: Option<i32>,
    /// updated filter logic identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_logic: Option<String>,
    /// updated UI visibility identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_visible: Option<String>,
    /// updated UI editable identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_editable: Option<String>,
    /// updated cloneable flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cloneable: Option<bool>,
    /// updated default json value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    /// updated related object filter json.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_object_filter: Option<Value>,
    /// updated field weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// updated validation minimum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_minimum: Option<f64>,
    /// updated validation maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_maximum: Option<f64>,
    /// updated validation regex.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_regex: Option<String>,
    /// updated choice set id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_set: Option<i32>,
    /// updated comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// request for creating a config context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConfigContextRequest {
    /// config context name.
    pub name: String,
    /// weight value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// profile id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<i32>,
    /// description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// active flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// region IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<i32>>,
    /// site group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_groups: Option<Vec<i32>>,
    /// site IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<i32>>,
    /// location IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<i32>>,
    /// device type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<Vec<i32>>,
    /// role IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<i32>>,
    /// platform IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<i32>>,
    /// cluster type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<i32>>,
    /// cluster group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_groups: Option<Vec<i32>>,
    /// cluster IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<i32>>,
    /// tenant group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_groups: Option<Vec<i32>>,
    /// tenant IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<i32>>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// data source id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// request for updating a config context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigContextRequest {
    /// updated config context name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated weight value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// updated profile id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<i32>,
    /// updated description text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated active flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// updated region IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<i32>>,
    /// updated site group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_groups: Option<Vec<i32>>,
    /// updated site IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<i32>>,
    /// updated location IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<i32>>,
    /// updated device type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<Vec<i32>>,
    /// updated role IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<i32>>,
    /// updated platform IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<i32>>,
    /// updated cluster type IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<i32>>,
    /// updated cluster group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_groups: Option<Vec<i32>>,
    /// updated cluster IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<i32>>,
    /// updated tenant group IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_groups: Option<Vec<i32>>,
    /// updated tenant IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<i32>>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// updated data source id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// updated data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// request for creating a config context profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConfigContextProfileRequest {
    /// profile name.
    pub name: String,
    /// profile description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// profile schema payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// data source id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
}

/// request for updating a config context profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigContextProfileRequest {
    /// updated profile name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated profile description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated profile schema payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// updated comments text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// updated data source id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
}

/// request for creating a config template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConfigTemplateRequest {
    /// template name.
    pub name: String,
    /// template code (Jinja2).
    pub template_code: String,
    /// template description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// jinja environment params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_params: Option<Value>,
    /// mIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// attachment flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_attachment: Option<bool>,
    /// data source id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a config template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfigTemplateRequest {
    /// updated template name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated template code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// updated template description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated Jinja environment params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_params: Option<Value>,
    /// updated MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// updated file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// updated file extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// updated attachment flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_attachment: Option<bool>,
    /// updated data source id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<i32>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for creating a webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    /// webhook name.
    pub name: String,
    /// payload url.
    pub payload_url: String,
    /// webhook description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// http method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// http content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_content_type: Option<String>,
    /// additional headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_headers: Option<String>,
    /// body template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// secret string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// ssl verification flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_verification: Option<bool>,
    /// cA file path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_file_path: Option<String>,
    /// custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// request for updating a webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWebhookRequest {
    /// updated webhook name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// updated payload url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_url: Option<String>,
    /// updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// updated http method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// updated http content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_content_type: Option<String>,
    /// updated additional headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_headers: Option<String>,
    /// updated body template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    /// updated secret string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// updated ssl verification flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_verification: Option<bool>,
    /// updated CA file path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_file_path: Option<String>,
    /// updated custom fields payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, Value>>,
    /// updated tag IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<i32>>,
}

/// bookmark model.
pub type Bookmark = crate::models::Bookmark;
/// config context profile model.
pub type ConfigContextProfile = crate::models::ConfigContextProfile;
/// config context model.
pub type ConfigContext = crate::models::ConfigContext;
/// config template model.
pub type ConfigTemplate = crate::models::ConfigTemplate;
/// custom field choice set model.
pub type CustomFieldChoiceSet = crate::models::CustomFieldChoiceSet;
/// custom field model.
pub type CustomField = crate::models::CustomField;
/// custom link model.
pub type CustomLink = crate::models::CustomLink;
/// dashboard model.
pub type Dashboard = crate::models::Dashboard;
/// dashboard request model.
pub type DashboardRequest = crate::models::DashboardRequest;
/// patched dashboard request model.
pub type PatchedDashboardRequest = crate::models::PatchedDashboardRequest;
/// event rule model.
pub type EventRule = crate::models::EventRule;
/// export template model.
pub type ExportTemplate = crate::models::ExportTemplate;
/// image attachment model.
pub type ImageAttachment = crate::models::ImageAttachment;
/// journal entry model.
pub type JournalEntry = crate::models::JournalEntry;
/// notification group model.
pub type NotificationGroup = crate::models::NotificationGroup;
/// notification model.
pub type Notification = crate::models::Notification;
/// object type model.
pub type ObjectType = crate::models::ObjectType;
/// saved filter model.
pub type SavedFilter = crate::models::SavedFilter;
/// script model.
pub type Script = crate::models::Script;
/// subscription model.
pub type Subscription = crate::models::Subscription;
/// table config model.
pub type TableConfig = crate::models::TableConfig;
/// tagged item model.
pub type TaggedItem = crate::models::TaggedItem;
/// tag model.
pub type Tag = crate::models::Tag;
/// webhook model.
pub type Webhook = crate::models::Webhook;

/// resource for bookmarks.
pub type BookmarksApi = Resource<crate::models::Bookmark>;
/// resource for config context profiles.
pub type ConfigContextProfilesApi = Resource<crate::models::ConfigContextProfile>;
/// resource for config contexts.
pub type ConfigContextsApi = Resource<crate::models::ConfigContext>;
/// resource for config templates.
pub type ConfigTemplatesApi = Resource<crate::models::ConfigTemplate>;
/// resource for custom field choice sets.
pub type CustomFieldChoiceSetsApi = Resource<crate::models::CustomFieldChoiceSet>;
/// resource for custom fields.
pub type CustomFieldsApi = Resource<crate::models::CustomField>;
/// resource for custom links.
pub type CustomLinksApi = Resource<crate::models::CustomLink>;
/// resource for event rules.
pub type EventRulesApi = Resource<crate::models::EventRule>;
/// resource for export templates.
pub type ExportTemplatesApi = Resource<crate::models::ExportTemplate>;
/// resource for image attachments.
pub type ImageAttachmentsApi = Resource<crate::models::ImageAttachment>;
/// resource for journal entries.
pub type JournalEntriesApi = Resource<crate::models::JournalEntry>;
/// resource for notification groups.
pub type NotificationGroupsApi = Resource<crate::models::NotificationGroup>;
/// resource for notifications.
pub type NotificationsApi = Resource<crate::models::Notification>;
/// resource for object types.
pub type ObjectTypesApi = Resource<crate::models::ObjectType>;
/// resource for saved filters.
pub type SavedFiltersApi = Resource<crate::models::SavedFilter>;
/// resource for scripts.
pub type ScriptsApi = Resource<crate::models::Script>;
/// resource for subscriptions.
pub type SubscriptionsApi = Resource<crate::models::Subscription>;
/// resource for table configs.
pub type TableConfigsApi = Resource<crate::models::TableConfig>;
/// resource for tagged objects.
pub type TaggedObjectsApi = Resource<crate::models::TaggedItem>;
/// resource for tags.
pub type TagsApi = Resource<crate::models::Tag>;
/// resource for webhooks.
pub type WebhooksApi = Resource<crate::models::Webhook>;

/// api for extras endpoints.
#[derive(Clone)]
pub struct ExtrasApi {
    client: Client,
}

impl ExtrasApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the bookmarks resource.
    pub fn bookmarks(&self) -> BookmarksApi {
        Resource::new(self.client.clone(), "extras/bookmarks/")
    }

    /// returns the config context profiles resource.
    pub fn config_context_profiles(&self) -> ConfigContextProfilesApi {
        Resource::new(self.client.clone(), "extras/config-context-profiles/")
    }

    /// returns the config contexts resource.
    pub fn config_contexts(&self) -> ConfigContextsApi {
        Resource::new(self.client.clone(), "extras/config-contexts/")
    }

    /// returns the config templates resource.
    pub fn config_templates(&self) -> ConfigTemplatesApi {
        Resource::new(self.client.clone(), "extras/config-templates/")
    }

    /// returns the custom field choice sets resource.
    pub fn custom_field_choice_sets(&self) -> CustomFieldChoiceSetsApi {
        Resource::new(self.client.clone(), "extras/custom-field-choice-sets/")
    }

    /// returns the custom fields resource.
    pub fn custom_fields(&self) -> CustomFieldsApi {
        Resource::new(self.client.clone(), "extras/custom-fields/")
    }

    /// returns the custom links resource.
    pub fn custom_links(&self) -> CustomLinksApi {
        Resource::new(self.client.clone(), "extras/custom-links/")
    }

    /// fetch the current dashboard configuration.
    pub async fn dashboard(&self) -> Result<Dashboard> {
        self.client.get("extras/dashboard/").await
    }

    /// update the dashboard configuration (full update).
    pub async fn update_dashboard(&self, body: &DashboardRequest) -> Result<Dashboard> {
        self.client.put("extras/dashboard/", body).await
    }

    /// update the dashboard configuration (partial update).
    pub async fn patch_dashboard(&self, body: &PatchedDashboardRequest) -> Result<Dashboard> {
        self.client.patch("extras/dashboard/", body).await
    }

    /// delete the current dashboard configuration.
    pub async fn delete_dashboard(&self) -> Result<()> {
        self.client.delete("extras/dashboard/").await
    }

    /// returns the event rules resource.
    pub fn event_rules(&self) -> EventRulesApi {
        Resource::new(self.client.clone(), "extras/event-rules/")
    }

    /// returns the export templates resource.
    pub fn export_templates(&self) -> ExportTemplatesApi {
        Resource::new(self.client.clone(), "extras/export-templates/")
    }

    /// returns the image attachments resource.
    pub fn image_attachments(&self) -> ImageAttachmentsApi {
        Resource::new(self.client.clone(), "extras/image-attachments/")
    }

    /// returns the journal entries resource.
    pub fn journal_entries(&self) -> JournalEntriesApi {
        Resource::new(self.client.clone(), "extras/journal-entries/")
    }

    /// returns the notification groups resource.
    pub fn notification_groups(&self) -> NotificationGroupsApi {
        Resource::new(self.client.clone(), "extras/notification-groups/")
    }

    /// returns the notifications resource.
    pub fn notifications(&self) -> NotificationsApi {
        Resource::new(self.client.clone(), "extras/notifications/")
    }

    /// returns the object types resource.
    pub fn object_types(&self) -> ObjectTypesApi {
        Resource::new(self.client.clone(), "extras/object-types/")
    }

    /// returns the saved filters resource.
    pub fn saved_filters(&self) -> SavedFiltersApi {
        Resource::new(self.client.clone(), "extras/saved-filters/")
    }

    /// returns the scripts resource.
    pub fn scripts(&self) -> ScriptsApi {
        Resource::new(self.client.clone(), "extras/scripts/")
    }

    /// returns the subscriptions resource.
    pub fn subscriptions(&self) -> SubscriptionsApi {
        Resource::new(self.client.clone(), "extras/subscriptions/")
    }

    /// returns the table configs resource.
    pub fn table_configs(&self) -> TableConfigsApi {
        Resource::new(self.client.clone(), "extras/table-configs/")
    }

    /// returns the tagged objects resource.
    pub fn tagged_objects(&self) -> TaggedObjectsApi {
        Resource::new(self.client.clone(), "extras/tagged-objects/")
    }

    /// returns the tags resource.
    pub fn tags(&self) -> TagsApi {
        Resource::new(self.client.clone(), "extras/tags/")
    }

    /// returns the webhooks resource.
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
        assert_path(
            api.config_context_profiles(),
            "extras/config-context-profiles/",
        );
        assert_path(api.config_contexts(), "extras/config-contexts/");
        assert_path(api.config_templates(), "extras/config-templates/");
        assert_path(
            api.custom_field_choice_sets(),
            "extras/custom-field-choice-sets/",
        );
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
