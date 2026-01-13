# CircuitGroupAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**group** | [**crate::models::BriefCircuitGroup**](BriefCircuitGroup.md) |  | 
**member_type** | **String** |  | 
**member_id** | **i64** |  | 
**member** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**priority** | Option<[**crate::models::BriefCircuitGroupAssignmentSerializerPriority**](BriefCircuitGroupAssignmentSerializer__priority.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


