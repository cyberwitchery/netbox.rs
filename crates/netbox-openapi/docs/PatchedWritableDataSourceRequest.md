# PatchedWritableDataSourceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**source_url** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**sync_interval** | Option<**i32**> | * `1` - Minutely * `60` - Hourly * `720` - 12 hours * `1440` - Daily * `10080` - Weekly * `43200` - 30 days | [optional]
**parameters** | Option<[**serde_json::Value**](.md)> |  | [optional]
**ignore_rules** | Option<**String**> | Patterns (one per line) matching files to ignore when syncing | [optional]
**comments** | Option<**String**> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


