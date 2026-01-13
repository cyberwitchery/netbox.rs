# PatchedWritableModuleTypeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**profile** | Option<[**crate::models::BriefModuleTypeRequestProfile**](BriefModuleTypeRequest_profile.md)> |  | [optional]
**manufacturer** | Option<[**crate::models::BriefDeviceTypeRequestManufacturer**](BriefDeviceTypeRequest_manufacturer.md)> |  | [optional]
**model** | Option<**String**> |  | [optional]
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**airflow** | Option<**String**> | * `front-to-rear` - Front to rear * `rear-to-front` - Rear to front * `left-to-right` - Left to right * `right-to-left` - Right to left * `side-to-rear` - Side to rear * `passive` - Passive | [optional]
**weight** | Option<**f64**> |  | [optional]
**weight_unit** | Option<**String**> | * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces | [optional]
**description** | Option<**String**> |  | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


