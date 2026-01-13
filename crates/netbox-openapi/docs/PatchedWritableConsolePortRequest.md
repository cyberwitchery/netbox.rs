# PatchedWritableConsolePortRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | Option<[**crate::models::BriefInterfaceRequestDevice**](BriefInterfaceRequest_device.md)> |  | [optional]
**module** | Option<[**crate::models::ConsolePortRequestModule**](ConsolePortRequest_module.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**label** | Option<**String**> | Physical label | [optional]
**r#type** | Option<**String**> | Physical port type  * `de-9` - DE-9 * `db-25` - DB-25 * `rj-11` - RJ-11 * `rj-12` - RJ-12 * `rj-45` - RJ-45 * `mini-din-8` - Mini-DIN 8 * `usb-a` - USB Type A * `usb-b` - USB Type B * `usb-c` - USB Type C * `usb-mini-a` - USB Mini A * `usb-mini-b` - USB Mini B * `usb-micro-a` - USB Micro A * `usb-micro-b` - USB Micro B * `usb-micro-ab` - USB Micro AB * `other` - Other | [optional]
**speed** | Option<**i32**> | Port speed in bits per second  * `1200` - 1200 bps * `2400` - 2400 bps * `4800` - 4800 bps * `9600` - 9600 bps * `19200` - 19.2 kbps * `38400` - 38.4 kbps * `57600` - 57.6 kbps * `115200` - 115.2 kbps | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


