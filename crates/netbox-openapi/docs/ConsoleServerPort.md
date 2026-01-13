# ConsoleServerPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device** | [**crate::models::BriefDevice**](BriefDevice.md) |  | 
**module** | Option<[**crate::models::BriefModule**](BriefModule.md)> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | Option<[**crate::models::ConsolePortType**](ConsolePort_type.md)> |  | [optional]
**speed** | Option<[**crate::models::ConsolePortSpeed**](ConsolePort_speed.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::BriefCable**](BriefCable.md)> |  | [readonly]
**cable_end** | **String** |  | [readonly]
**link_peers** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**link_peers_type** | Option<**String**> | Return the type of the peer link terminations, or None. | [readonly]
**connected_endpoints** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**connected_endpoints_type** | Option<**String**> |  | [readonly]
**connected_endpoints_reachable** | **bool** |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**_occupied** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


