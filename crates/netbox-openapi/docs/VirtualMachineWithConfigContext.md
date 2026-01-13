# VirtualMachineWithConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**status** | Option<[**crate::models::VirtualMachineWithConfigContextStatus**](VirtualMachineWithConfigContext_status.md)> |  | [optional]
**site** | Option<[**crate::models::BriefSite**](BriefSite.md)> |  | [optional]
**cluster** | Option<[**crate::models::BriefCluster**](BriefCluster.md)> |  | [optional]
**device** | Option<[**crate::models::BriefDevice**](BriefDevice.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**role** | Option<[**crate::models::BriefDeviceRole**](BriefDeviceRole.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**platform** | Option<[**crate::models::BriefPlatform**](BriefPlatform.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [readonly]
**primary_ip4** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::BriefIpAddress**](BriefIPAddress.md)> |  | [optional]
**vcpus** | Option<**f64**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**config_template** | Option<[**crate::models::BriefConfigTemplate**](BriefConfigTemplate.md)> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> | Local config context data takes precedence over source contexts in the final rendered config context | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**config_context** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**interface_count** | **i32** |  | [readonly]
**virtual_disk_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


