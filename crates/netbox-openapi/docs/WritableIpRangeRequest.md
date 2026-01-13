# WritableIpRangeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_address** | **String** |  | 
**end_address** | **String** |  | 
**vrf** | Option<[**crate::models::IpAddressRequestVrf**](IPAddressRequest_vrf.md)> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**status** | Option<**String**> | Operational status of this range  * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<[**crate::models::IpRangeRequestRole**](IPRangeRequest_role.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**mark_populated** | Option<**bool**> | Prevent the creation of IP addresses within this range | [optional]
**mark_utilized** | Option<**bool**> | Report space as 100% utilized | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


