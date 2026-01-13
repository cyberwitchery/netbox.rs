# PatchedWritableIpSecProfileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<**String**> | * `esp` - ESP * `ah` - AH | [optional]
**ike_policy** | Option<[**crate::models::IpSecProfileRequestIkePolicy**](IPSecProfileRequest_ike_policy.md)> |  | [optional]
**ipsec_policy** | Option<[**crate::models::IpSecProfileRequestIpsecPolicy**](IPSecProfileRequest_ipsec_policy.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


