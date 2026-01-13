# TunnelTermination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**tunnel** | [**crate::models::BriefTunnel**](BriefTunnel.md) |  | 
**role** | [**crate::models::TunnelTerminationRole**](TunnelTermination_role.md) |  | 
**termination_type** | **String** |  | 
**termination_id** | Option<**i64**> |  | [optional]
**termination** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**outside_ip** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


