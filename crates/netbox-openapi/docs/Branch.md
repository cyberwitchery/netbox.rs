# Branch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**status** | Option<[**crate::models::BranchStatus**](Branch_status.md)> |  | [optional]
**owner** | [**crate::models::BriefUser**](BriefUser.md) |  | [readonly]
**description** | Option<**String**> |  | [optional]
**schema_id** | **String** |  | [readonly]
**last_sync** | Option<**String**> |  | [readonly]
**merged_time** | Option<**String**> |  | [optional]
**merged_by** | [**crate::models::BriefUser**](BriefUser.md) |  | [readonly]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


