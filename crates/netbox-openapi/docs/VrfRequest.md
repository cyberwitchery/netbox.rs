# VrfRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**rd** | Option<**String**> | Unique route distinguisher (as defined in RFC 4364) | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**enforce_unique** | Option<**bool**> | Prevent duplicate prefixes/IP addresses within this VRF | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**import_targets** | Option<**Vec<i32>**> |  | [optional]
**export_targets** | Option<**Vec<i32>**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


