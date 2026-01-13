# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**parent_object_type** | **String** |  | 
**parent_object_id** | **i64** |  | 
**parent** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**name** | **String** |  | 
**protocol** | Option<[**crate::models::ServiceProtocol**](Service_protocol.md)> |  | [optional]
**ports** | **Vec<i32>** |  | 
**ipaddresses** | Option<[**Vec<crate::models::IpAddress>**](IPAddress.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


