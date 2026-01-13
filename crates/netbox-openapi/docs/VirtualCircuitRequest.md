# VirtualCircuitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | **String** | Unique circuit ID | 
**provider_network** | [**crate::models::BriefVirtualCircuitRequestProviderNetwork**](BriefVirtualCircuitRequest_provider_network.md) |  | 
**provider_account** | Option<[**crate::models::CircuitRequestProviderAccount**](CircuitRequest_provider_account.md)> |  | [optional]
**r#type** | [**crate::models::PatchedWritableVirtualCircuitRequestType**](PatchedWritableVirtualCircuitRequest_type.md) |  | 
**status** | Option<**String**> | * `planned` - Planned * `provisioning` - Provisioning * `active` - Active * `offline` - Offline * `deprovisioning` - Deprovisioning * `decommissioned` - Decommissioned | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


