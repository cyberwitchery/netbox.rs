# DataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**r#type** | [**crate::models::DataSourceType**](DataSource_type.md) |  | 
**source_url** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**status** | [**crate::models::DataSourceStatus**](DataSource_status.md) |  | 
**description** | Option<**String**> |  | [optional]
**sync_interval** | Option<**i32**> | * `1` - Minutely * `60` - Hourly * `720` - 12 hours * `1440` - Daily * `10080` - Weekly * `43200` - 30 days | [optional]
**parameters** | Option<[**serde_json::Value**](.md)> |  | [optional]
**ignore_rules** | Option<**String**> | Patterns (one per line) matching files to ignore when syncing | [optional]
**comments** | Option<**String**> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**last_synced** | Option<**String**> |  | [readonly]
**file_count** | **i64** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


