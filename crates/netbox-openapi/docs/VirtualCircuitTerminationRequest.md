# VirtualCircuitTerminationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**virtual_circuit** | [**crate::models::PatchedWritableVirtualCircuitTerminationRequestVirtualCircuit**](PatchedWritableVirtualCircuitTerminationRequest_virtual_circuit.md) |  | 
**role** | Option<**String**> | * `peer` - Peer * `hub` - Hub * `spoke` - Spoke | [optional]
**interface** | [**crate::models::PatchedWritableVirtualCircuitTerminationRequestInterface**](PatchedWritableVirtualCircuitTerminationRequest_interface.md) |  | 
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


