# ContactAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**object_type** | **String** |  | 
**object_id** | **i64** |  | 
**object** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**contact** | [**crate::models::BriefContact**](BriefContact.md) |  | 
**role** | Option<[**crate::models::BriefContactRole**](BriefContactRole.md)> |  | [optional]
**priority** | Option<[**crate::models::BriefCircuitGroupAssignmentSerializerPriority**](BriefCircuitGroupAssignmentSerializer__priority.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


