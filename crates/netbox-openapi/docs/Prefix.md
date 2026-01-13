# Prefix

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**family** | [**crate::models::AggregateFamily**](Aggregate_family.md) |  | 
**prefix** | **String** |  | 
**vrf** | Option<[**crate::models::BriefVrf**](BriefVRF.md)> |  | [optional]
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**scope** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**vlan** | Option<[**crate::models::BriefVlan**](BriefVLAN.md)> |  | [optional]
**status** | Option<[**crate::models::PrefixStatus**](Prefix_status.md)> |  | [optional]
**role** | Option<[**crate::models::BriefRole**](BriefRole.md)> |  | [optional]
**is_pool** | Option<**bool**> | All IP addresses within this prefix are considered usable | [optional]
**mark_utilized** | Option<**bool**> | Treat as fully utilized | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**children** | **i32** |  | [readonly]
**_depth** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


