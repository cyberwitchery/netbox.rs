# Platform

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**parent** | Option<[**crate::models::NestedPlatform**](NestedPlatform.md)> |  | [optional]
**name** | **String** |  | 
**slug** | **String** |  | 
**manufacturer** | Option<[**crate::models::BriefManufacturer**](BriefManufacturer.md)> |  | [optional]
**config_template** | Option<[**crate::models::BriefConfigTemplate**](BriefConfigTemplate.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**device_count** | **i32** |  | [readonly][default to 0]
**virtualmachine_count** | **i32** |  | [readonly][default to 0]
**_depth** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


