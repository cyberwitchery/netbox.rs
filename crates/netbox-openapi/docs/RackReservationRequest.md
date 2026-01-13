# RackReservationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rack** | [**crate::models::PatchedWritableRackReservationRequestRack**](PatchedWritableRackReservationRequest_rack.md) |  | 
**units** | **Vec<i32>** |  | 
**status** | Option<**String**> | * `pending` - Pending * `active` - Active * `stale` - Stale | [optional]
**user** | [**crate::models::BookmarkRequestUser**](BookmarkRequest_user.md) |  | 
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**description** | **String** |  | 
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


