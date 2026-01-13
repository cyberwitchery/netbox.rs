# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**object_type** | **String** |  | [readonly]
**object_id** | Option<**i64**> |  | [optional]
**name** | **String** |  | 
**status** | [**crate::models::BriefJobStatus**](BriefJob_status.md) |  | 
**created** | **String** |  | [readonly]
**scheduled** | Option<**String**> |  | [optional]
**interval** | Option<**i32**> | Recurrence interval (in minutes) | [optional]
**started** | Option<**String**> |  | [optional]
**completed** | Option<**String**> |  | [optional]
**user** | [**crate::models::BriefUser**](BriefUser.md) |  | [readonly]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**error** | **String** |  | [readonly]
**job_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**log_entries** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


