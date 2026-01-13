# Tunnel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**status** | [**crate::models::TunnelStatus**](Tunnel_status.md) |  | 
**group** | Option<[**crate::models::BriefTunnelGroup**](BriefTunnelGroup.md)> |  | [optional]
**encapsulation** | [**crate::models::TunnelEncapsulation**](Tunnel_encapsulation.md) |  | 
**ipsec_profile** | Option<[**crate::models::BriefIpSecProfile**](BriefIPSecProfile.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**tunnel_id** | Option<**i64**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**terminations_count** | **i64** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


