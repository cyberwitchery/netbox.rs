# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | Option<**String**> |  | [optional]
**device_type** | [**crate::models::BriefDeviceType**](BriefDeviceType.md) |  | 
**role** | [**crate::models::BriefDeviceRole**](BriefDeviceRole.md) |  | 
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**platform** | Option<[**crate::models::BriefPlatform**](BriefPlatform.md)> |  | [optional]
**serial** | Option<**String**> | Chassis serial number, assigned by the manufacturer | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**site** | [**crate::models::BriefSite**](BriefSite.md) |  | 
**location** | Option<[**crate::models::BriefLocation**](BriefLocation.md)> |  | [optional]
**rack** | Option<[**crate::models::BriefRack**](BriefRack.md)> |  | [optional]
**position** | Option<**f64**> |  | [optional]
**face** | Option<[**crate::models::DeviceFace**](Device_face.md)> |  | [optional]
**latitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**longitude** | Option<**f64**> | GPS coordinate in decimal format (xx.yyyyyy) | [optional]
**parent_device** | Option<[**crate::models::NestedDevice**](NestedDevice.md)> |  | [readonly]
**status** | Option<[**crate::models::DeviceStatus**](Device_status.md)> |  | [optional]
**airflow** | Option<[**crate::models::DeviceAirflow**](Device_airflow.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [readonly]
**primary_ip4** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**oob_ip** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**cluster** | Option<[**crate::models::BriefCluster**](BriefCluster.md)> |  | [optional]
**virtual_chassis** | Option<[**crate::models::BriefVirtualChassis**](BriefVirtualChassis.md)> |  | [optional]
**vc_position** | Option<**i32**> |  | [optional]
**vc_priority** | Option<**i32**> | Virtual chassis master election priority | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**config_template** | Option<[**crate::models::BriefConfigTemplate**](BriefConfigTemplate.md)> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> | Local config context data takes precedence over source contexts in the final rendered config context | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**console_port_count** | **i32** |  | [readonly]
**console_server_port_count** | **i32** |  | [readonly]
**power_port_count** | **i32** |  | [readonly]
**power_outlet_count** | **i32** |  | [readonly]
**interface_count** | **i32** |  | [readonly]
**front_port_count** | **i32** |  | [readonly]
**rear_port_count** | **i32** |  | [readonly]
**device_bay_count** | **i32** |  | [readonly]
**module_bay_count** | **i32** |  | [readonly]
**inventory_item_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


