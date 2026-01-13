# PatchedWritableServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parent_object_type** | Option<**String**> |  | [optional]
**parent_object_id** | Option<**i64**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**protocol** | Option<**String**> | * `tcp` - TCP * `udp` - UDP * `sctp` - SCTP | [optional]
**ports** | Option<**Vec<i32>**> |  | [optional]
**ipaddresses** | Option<**Vec<i32>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


