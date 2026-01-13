# L2Vpn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**identifier** | Option<**i64**> |  | [optional]
**name** | **String** |  | 
**slug** | **String** |  | 
**r#type** | Option<[**crate::models::BriefL2VpnType**](BriefL2VPN_type.md)> |  | [optional]
**status** | Option<[**crate::models::L2VpnStatus**](L2VPN_status.md)> |  | [optional]
**import_targets** | Option<[**Vec<crate::models::RouteTarget>**](RouteTarget.md)> |  | [optional]
**export_targets** | Option<[**Vec<crate::models::RouteTarget>**](RouteTarget.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


