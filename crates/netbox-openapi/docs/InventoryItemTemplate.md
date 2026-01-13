# InventoryItemTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device_type** | [**crate::models::BriefDeviceType**](BriefDeviceType.md) |  | 
**parent** | Option<**i32**> |  | [optional]
**name** | **String** | {module} is accepted as a substitution for the module bay position when attached to a module type. | 
**label** | Option<**String**> | Physical label | [optional]
**role** | Option<[**crate::models::BriefInventoryItemRole**](BriefInventoryItemRole.md)> |  | [optional]
**manufacturer** | Option<[**crate::models::BriefManufacturer**](BriefManufacturer.md)> |  | [optional]
**part_id** | Option<**String**> | Manufacturer-assigned part identifier | [optional]
**description** | Option<**String**> |  | [optional]
**component_type** | Option<**String**> |  | [optional]
**component_id** | Option<**i64**> |  | [optional]
**component** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**_depth** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


