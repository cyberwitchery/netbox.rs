# DeviceRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**color** | Option<**String**> |  | [optional]
**vm_role** | Option<**bool**> | Virtual machines may be assigned to this role | [optional]
**config_template** | Option<[**crate::models::BriefConfigTemplate**](BriefConfigTemplate.md)> |  | [optional]
**parent** | Option<[**crate::models::NestedDeviceRole**](NestedDeviceRole.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**device_count** | **i32** |  | [readonly][default to 0]
**virtualmachine_count** | **i32** |  | [readonly][default to 0]
**comments** | Option<**String**> |  | [optional]
**_depth** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


