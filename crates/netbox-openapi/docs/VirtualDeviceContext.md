# VirtualDeviceContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**device** | [**crate::models::BriefDevice**](BriefDevice.md) |  | 
**identifier** | Option<**i32**> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [readonly]
**primary_ip4** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**status** | [**crate::models::VirtualDeviceContextStatus**](VirtualDeviceContext_status.md) |  | 
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**interface_count** | **i64** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


