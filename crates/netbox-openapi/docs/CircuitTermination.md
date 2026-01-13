# CircuitTermination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**circuit** | [**crate::models::BriefCircuit**](BriefCircuit.md) |  | 
**term_side** | **String** | * `A` - A * `Z` - Z | 
**termination_type** | Option<**String**> |  | [optional]
**termination_id** | Option<**i32**> |  | [optional]
**termination** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**port_speed** | Option<**i32**> | Physical circuit speed | [optional]
**upstream_speed** | Option<**i32**> | Upstream speed, if different from port speed | [optional]
**xconnect_id** | Option<**String**> | ID of the local cross-connect | [optional]
**pp_info** | Option<**String**> | Patch panel ID and port number(s) | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::BriefCable**](BriefCable.md)> |  | [readonly]
**cable_end** | **String** |  | [readonly]
**link_peers** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**link_peers_type** | Option<**String**> | Return the type of the peer link terminations, or None. | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**_occupied** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


