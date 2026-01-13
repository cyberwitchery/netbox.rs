# PatchedWritableContactAssignmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object_type** | Option<**String**> |  | [optional]
**object_id** | Option<**i64**> |  | [optional]
**contact** | Option<[**crate::models::ContactAssignmentRequestContact**](ContactAssignmentRequest_contact.md)> |  | [optional]
**role** | Option<[**crate::models::ContactAssignmentRequestRole**](ContactAssignmentRequest_role.md)> |  | [optional]
**priority** | Option<**String**> | * `primary` - Primary * `secondary` - Secondary * `tertiary` - Tertiary * `inactive` - Inactive | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


