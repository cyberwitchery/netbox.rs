# PowerFeed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**power_panel** | [**crate::models::BriefPowerPanel**](BriefPowerPanel.md) |  | 
**rack** | Option<[**crate::models::BriefRack**](BriefRack.md)> |  | [optional]
**name** | **String** |  | 
**status** | Option<[**crate::models::PowerFeedStatus**](PowerFeed_status.md)> |  | [optional]
**r#type** | Option<[**crate::models::PowerFeedType**](PowerFeed_type.md)> |  | [optional]
**supply** | Option<[**crate::models::PowerFeedSupply**](PowerFeed_supply.md)> |  | [optional]
**phase** | Option<[**crate::models::PowerFeedPhase**](PowerFeed_phase.md)> |  | [optional]
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::BriefCable**](BriefCable.md)> |  | [readonly]
**cable_end** | **String** |  | [readonly]
**link_peers** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**link_peers_type** | Option<**String**> | Return the type of the peer link terminations, or None. | [readonly]
**connected_endpoints** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**connected_endpoints_type** | Option<**String**> |  | [readonly]
**connected_endpoints_reachable** | **bool** |  | [readonly]
**description** | Option<**String**> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**_occupied** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


