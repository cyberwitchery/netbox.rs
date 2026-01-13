# Notification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**object_type** | **String** |  | 
**object_id** | **i64** |  | 
**object** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**user** | [**crate::models::BriefUser**](BriefUser.md) |  | 
**created** | **String** |  | [readonly]
**read** | Option<**String**> |  | [optional]
**event_type** | **String** | * `object_created` - Object created * `object_updated` - Object updated * `object_deleted` - Object deleted * `job_started` - Job started * `job_completed` - Job completed * `job_failed` - Job failed * `job_errored` - Job errored * `branch_provisioned` - Branch provisioned * `branch_deprovisioned` - Branch deprovisioned * `branch_synced` - Branch synced * `branch_merged` - Branch merged * `branch_reverted` - Branch reverted | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


