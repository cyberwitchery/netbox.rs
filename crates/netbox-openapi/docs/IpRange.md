# IpRange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**family** | [**crate::models::AggregateFamily**](Aggregate_family.md) |  | 
**start_address** | **String** |  | 
**end_address** | **String** |  | 
**size** | **i32** |  | [readonly]
**vrf** | Option<[**crate::models::BriefVrf**](BriefVRF.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**status** | Option<[**crate::models::IpRangeStatus**](IPRange_status.md)> |  | [optional]
**role** | Option<[**crate::models::BriefRole**](BriefRole.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**mark_populated** | Option<**bool**> | Prevent the creation of IP addresses within this range | [optional]
**mark_utilized** | Option<**bool**> | Report space as 100% utilized | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


