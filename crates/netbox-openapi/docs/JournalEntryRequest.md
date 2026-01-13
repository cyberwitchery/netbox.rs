# JournalEntryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assigned_object_type** | **String** |  | 
**assigned_object_id** | **i64** |  | 
**created_by** | Option<**i32**> |  | [optional]
**kind** | Option<**String**> | * `info` - Info * `success` - Success * `warning` - Warning * `danger` - Danger | [optional]
**comments** | **String** |  | 
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


