# ChangeDiff

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**branch** | [**crate::models::BriefBranch**](BriefBranch.md) |  | [readonly]
**object_type** | **String** |  | [readonly]
**object_id** | **i64** |  | 
**object** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**object_repr** | **String** |  | [readonly]
**action** | [**crate::models::ChangeDiffAction**](ChangeDiff_action.md) |  | 
**conflicts** | Option<**Vec<String>**> |  | [readonly]
**diff** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**original_data** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**modified_data** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**current_data** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**last_updated** | **String** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


