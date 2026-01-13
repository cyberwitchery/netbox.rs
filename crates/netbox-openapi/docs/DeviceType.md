# DeviceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**manufacturer** | [**crate::models::BriefManufacturer**](BriefManufacturer.md) |  | 
**default_platform** | Option<[**crate::models::BriefPlatform**](BriefPlatform.md)> |  | [optional]
**model** | **String** |  | 
**slug** | **String** |  | 
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**u_height** | Option<**f64**> |  | [optional][default to 1.0]
**exclude_from_utilization** | Option<**bool**> | Devices of this type are excluded when calculating rack utilization. | [optional]
**is_full_depth** | Option<**bool**> | Device consumes both front and rear rack faces. | [optional]
**subdevice_role** | Option<[**crate::models::DeviceTypeSubdeviceRole**](DeviceType_subdevice_role.md)> |  | [optional]
**airflow** | Option<[**crate::models::DeviceTypeAirflow**](DeviceType_airflow.md)> |  | [optional]
**weight** | Option<**f64**> |  | [optional]
**weight_unit** | Option<[**crate::models::DeviceTypeWeightUnit**](DeviceType_weight_unit.md)> |  | [optional]
**front_image** | Option<**String**> |  | [optional]
**rear_image** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**device_count** | **i64** |  | [readonly]
**console_port_template_count** | **i32** |  | [readonly]
**console_server_port_template_count** | **i32** |  | [readonly]
**power_port_template_count** | **i32** |  | [readonly]
**power_outlet_template_count** | **i32** |  | [readonly]
**interface_template_count** | **i32** |  | [readonly]
**front_port_template_count** | **i32** |  | [readonly]
**rear_port_template_count** | **i32** |  | [readonly]
**device_bay_template_count** | **i32** |  | [readonly]
**module_bay_template_count** | **i32** |  | [readonly]
**inventory_item_template_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


