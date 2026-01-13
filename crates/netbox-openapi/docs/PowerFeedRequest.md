# PowerFeedRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**power_panel** | [**crate::models::PatchedWritablePowerFeedRequestPowerPanel**](PatchedWritablePowerFeedRequest_power_panel.md) |  | 
**rack** | Option<[**crate::models::DeviceWithConfigContextRequestRack**](DeviceWithConfigContextRequest_rack.md)> |  | [optional]
**name** | **String** |  | 
**status** | Option<**String**> | * `offline` - Offline * `active` - Active * `planned` - Planned * `failed` - Failed | [optional]
**r#type** | Option<**String**> | * `primary` - Primary * `redundant` - Redundant | [optional]
**supply** | Option<**String**> | * `ac` - AC * `dc` - DC | [optional]
**phase** | Option<**String**> | * `single-phase` - Single phase * `three-phase` - Three-phase | [optional]
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**description** | Option<**String**> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


