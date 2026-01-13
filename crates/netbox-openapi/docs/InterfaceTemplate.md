# InterfaceTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device_type** | Option<[**crate::models::BriefDeviceType**](BriefDeviceType.md)> |  | [optional]
**module_type** | Option<[**crate::models::BriefModuleType**](BriefModuleType.md)> |  | [optional]
**name** | **String** | {module} is accepted as a substitution for the module bay position when attached to a module type. | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | [**crate::models::InterfaceType**](Interface_type.md) |  | 
**enabled** | Option<**bool**> |  | [optional]
**mgmt_only** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**bridge** | Option<[**crate::models::NestedInterfaceTemplate**](NestedInterfaceTemplate.md)> |  | [optional]
**poe_mode** | Option<[**crate::models::InterfaceTemplatePoeMode**](InterfaceTemplate_poe_mode.md)> |  | [optional]
**poe_type** | Option<[**crate::models::InterfaceTemplatePoeType**](InterfaceTemplate_poe_type.md)> |  | [optional]
**rf_role** | Option<[**crate::models::InterfaceTemplateRfRole**](InterfaceTemplate_rf_role.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


