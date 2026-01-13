# WritableInventoryItemRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device** | [**crate::models::BriefInterfaceRequestDevice**](BriefInterfaceRequest_device.md) |  | 
**parent** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**status** | Option<**String**> | * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `decommissioning` - Decommissioning | [optional]
**role** | Option<[**crate::models::InventoryItemRequestRole**](InventoryItemRequest_role.md)> |  | [optional]
**manufacturer** | Option<[**crate::models::InventoryItemRequestManufacturer**](InventoryItemRequest_manufacturer.md)> |  | [optional]
**part_id** | Option<**String**> | Manufacturer-assigned part identifier | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this item | [optional]
**discovered** | Option<**bool**> | This item was automatically discovered | [optional]
**description** | Option<**String**> |  | [optional]
**component_type** | Option<**String**> |  | [optional]
**component_id** | Option<**i64**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


