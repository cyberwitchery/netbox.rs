# PatchedAsnRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asn** | Option<**i64**> | 16- or 32-bit autonomous system number | [optional]
**rir** | Option<[**crate::models::AsnRequestRir**](ASNRequest_rir.md)> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


