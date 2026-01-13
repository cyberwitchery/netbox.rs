# VirtualCircuit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**cid** | **String** | Unique circuit ID | 
**provider_network** | [**crate::models::BriefProviderNetwork**](BriefProviderNetwork.md) |  | 
**provider_account** | Option<[**crate::models::BriefProviderAccount**](BriefProviderAccount.md)> |  | [optional]
**r#type** | [**crate::models::BriefVirtualCircuitType**](BriefVirtualCircuitType.md) |  | 
**status** | Option<[**crate::models::CircuitStatus**](Circuit_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


