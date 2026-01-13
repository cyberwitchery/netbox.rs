# PatchedWritableModuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | Option<[**crate::models::BriefInterfaceRequestDevice**](BriefInterfaceRequest_device.md)> |  | [optional]
**module_bay** | Option<**i32**> |  | [optional]
**module_type** | Option<[**crate::models::ModuleRequestModuleType**](ModuleRequest_module_type.md)> |  | [optional]
**status** | Option<**String**> | * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `decommissioning` - Decommissioning | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


