# JournalEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**assigned_object_type** | **String** |  | 
**assigned_object_id** | **i64** |  | 
**assigned_object** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**created** | Option<**String**> |  | [readonly]
**created_by** | Option<**i32**> |  | [optional]
**kind** | Option<[**crate::models::JournalEntryKind**](JournalEntry_kind.md)> |  | [optional]
**comments** | **String** |  | 
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


