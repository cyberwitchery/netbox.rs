# IpAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**family** | [**crate::models::AggregateFamily**](Aggregate_family.md) |  | 
**address** | **String** |  | 
**vrf** | Option<[**crate::models::BriefVrf**](BriefVRF.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**status** | Option<[**crate::models::IpAddressStatus**](IPAddress_status.md)> |  | [optional]
**role** | Option<[**crate::models::IpAddressRole**](IPAddress_role.md)> |  | [optional]
**assigned_object_type** | Option<**String**> |  | [optional]
**assigned_object_id** | Option<**i64**> |  | [optional]
**assigned_object** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**nat_inside** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**nat_outside** | [**Vec<crate::models::NestedIpAddress>**](NestedIPAddress.md) |  | [readonly]
**dns_name** | Option<**String**> | Hostname or FQDN (not case-sensitive) | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


