# Aggregate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**family** | [**crate::models::AggregateFamily**](Aggregate_family.md) |  | 
**prefix** | **String** |  | 
**rir** | [**crate::models::BriefRir**](BriefRIR.md) |  | 
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**date_added** | Option<[**String**](string.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


