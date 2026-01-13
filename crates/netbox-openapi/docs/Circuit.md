# Circuit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**cid** | **String** | Unique circuit ID | 
**provider** | [**crate::models::BriefProvider**](BriefProvider.md) |  | 
**provider_account** | Option<[**crate::models::BriefProviderAccount**](BriefProviderAccount.md)> |  | [optional]
**r#type** | [**crate::models::BriefCircuitType**](BriefCircuitType.md) |  | 
**status** | Option<[**crate::models::CircuitStatus**](Circuit_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**install_date** | Option<[**String**](string.md)> |  | [optional]
**termination_date** | Option<[**String**](string.md)> |  | [optional]
**commit_rate** | Option<**i32**> | Committed rate | [optional]
**description** | Option<**String**> |  | [optional]
**distance** | Option<**f64**> |  | [optional]
**distance_unit** | Option<[**crate::models::CircuitDistanceUnit**](Circuit_distance_unit.md)> |  | [optional]
**termination_a** | Option<[**crate::models::CircuitCircuitTermination**](CircuitCircuitTermination.md)> |  | [readonly]
**termination_z** | Option<[**crate::models::CircuitCircuitTermination**](CircuitCircuitTermination.md)> |  | [readonly]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**assignments** | Option<[**Vec<crate::models::BriefCircuitGroupAssignmentSerializer>**](BriefCircuitGroupAssignmentSerializer_.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


