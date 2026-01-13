# PatchedWritableDeviceTypeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**manufacturer** | Option<[**crate::models::BriefDeviceTypeRequestManufacturer**](BriefDeviceTypeRequest_manufacturer.md)> |  | [optional]
**default_platform** | Option<[**crate::models::DeviceTypeRequestDefaultPlatform**](DeviceTypeRequest_default_platform.md)> |  | [optional]
**model** | Option<**String**> |  | [optional]
**slug** | Option<**String**> |  | [optional]
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**u_height** | Option<**f64**> |  | [optional][default to 1.0]
**exclude_from_utilization** | Option<**bool**> | Devices of this type are excluded when calculating rack utilization. | [optional]
**is_full_depth** | Option<**bool**> | Device consumes both front and rear rack faces. | [optional]
**subdevice_role** | Option<**String**> | Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.  * `parent` - Parent * `child` - Child | [optional]
**airflow** | Option<**String**> | * `front-to-rear` - Front to rear * `rear-to-front` - Rear to front * `left-to-right` - Left to right * `right-to-left` - Right to left * `side-to-rear` - Side to rear * `rear-to-side` - Rear to side * `bottom-to-top` - Bottom to top * `top-to-bottom` - Top to bottom * `passive` - Passive * `mixed` - Mixed | [optional]
**weight** | Option<**f64**> |  | [optional]
**weight_unit** | Option<**String**> | * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces | [optional]
**front_image** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> |  | [optional]
**rear_image** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


