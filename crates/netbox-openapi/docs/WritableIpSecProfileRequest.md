# WritableIpSecProfileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**mode** | **String** | * `esp` - ESP * `ah` - AH | 
**ike_policy** | [**crate::models::IpSecProfileRequestIkePolicy**](IPSecProfileRequest_ike_policy.md) |  | 
**ipsec_policy** | [**crate::models::IpSecProfileRequestIpsecPolicy**](IPSecProfileRequest_ipsec_policy.md) |  | 
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


