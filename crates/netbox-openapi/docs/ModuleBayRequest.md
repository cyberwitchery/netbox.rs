# ModuleBayRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | [**crate::models::BriefInterfaceRequestDevice**](BriefInterfaceRequest_device.md) |  | 
**module** | Option<[**crate::models::ConsolePortRequestModule**](ConsolePortRequest_module.md)> |  | [optional]
**name** | **String** |  | 
**installed_module** | Option<[**crate::models::ConsolePortRequestModule**](ConsolePortRequest_module.md)> |  | [optional]
**label** | Option<**String**> | Physical label | [optional]
**position** | Option<**String**> | Identifier to reference when renaming installed components | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


