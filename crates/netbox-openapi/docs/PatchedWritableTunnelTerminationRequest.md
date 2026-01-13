# PatchedWritableTunnelTerminationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tunnel** | Option<[**crate::models::PatchedWritableTunnelTerminationRequestTunnel**](PatchedWritableTunnelTerminationRequest_tunnel.md)> |  | [optional]
**role** | Option<**String**> | * `peer` - Peer * `hub` - Hub * `spoke` - Spoke | [optional]
**termination_type** | Option<**String**> |  | [optional]
**termination_id** | Option<**i64**> |  | [optional]
**outside_ip** | Option<[**crate::models::DeviceWithConfigContextRequestPrimaryIp4**](DeviceWithConfigContextRequest_primary_ip4.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


