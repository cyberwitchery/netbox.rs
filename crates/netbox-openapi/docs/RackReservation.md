# RackReservation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**rack** | [**crate::models::BriefRack**](BriefRack.md) |  | 
**units** | **Vec<i32>** |  | 
**status** | Option<[**crate::models::RackReservationStatus**](RackReservation_status.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**user** | [**crate::models::BriefUser**](BriefUser.md) |  | 
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**description** | **String** |  | 
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


