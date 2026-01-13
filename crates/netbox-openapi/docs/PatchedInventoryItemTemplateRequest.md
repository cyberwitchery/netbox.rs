# PatchedInventoryItemTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_type** | Option<[**crate::models::DeviceBayTemplateRequestDeviceType**](DeviceBayTemplateRequest_device_type.md)> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**name** | Option<**String**> | {module} is accepted as a substitution for the module bay position when attached to a module type. | [optional]
**label** | Option<**String**> | Physical label | [optional]
**role** | Option<[**crate::models::InventoryItemRequestRole**](InventoryItemRequest_role.md)> |  | [optional]
**manufacturer** | Option<[**crate::models::InventoryItemRequestManufacturer**](InventoryItemRequest_manufacturer.md)> |  | [optional]
**part_id** | Option<**String**> | Manufacturer-assigned part identifier | [optional]
**description** | Option<**String**> |  | [optional]
**component_type** | Option<**String**> |  | [optional]
**component_id** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


