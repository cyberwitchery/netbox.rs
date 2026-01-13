# PatchedWritableDeviceRoleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**slug** | Option<**String**> |  | [optional]
**color** | Option<**String**> |  | [optional]
**vm_role** | Option<**bool**> | Virtual machines may be assigned to this role | [optional]
**config_template** | Option<[**crate::models::DeviceRoleRequestConfigTemplate**](DeviceRoleRequest_config_template.md)> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


