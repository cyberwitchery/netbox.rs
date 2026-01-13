# WritableCircuitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cid** | **String** | Unique circuit ID | 
**provider** | [**crate::models::BriefCircuitRequestProvider**](BriefCircuitRequest_provider.md) |  | 
**provider_account** | Option<[**crate::models::CircuitRequestProviderAccount**](CircuitRequest_provider_account.md)> |  | [optional]
**r#type** | [**crate::models::CircuitRequestType**](CircuitRequest_type.md) |  | 
**status** | Option<**String**> | * `planned` - Planned * `provisioning` - Provisioning * `active` - Active * `offline` - Offline * `deprovisioning` - Deprovisioning * `decommissioned` - Decommissioned | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**install_date** | Option<[**String**](string.md)> |  | [optional]
**termination_date** | Option<[**String**](string.md)> |  | [optional]
**commit_rate** | Option<**i32**> | Committed rate | [optional]
**description** | Option<**String**> |  | [optional]
**distance** | Option<**f64**> |  | [optional]
**distance_unit** | Option<**String**> | * `km` - Kilometers * `m` - Meters * `mi` - Miles * `ft` - Feet | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**assignments** | Option<[**Vec<crate::models::BriefCircuitGroupAssignmentSerializerRequest>**](BriefCircuitGroupAssignmentSerializer_Request.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


