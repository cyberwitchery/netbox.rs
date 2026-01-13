# PatchedWritableVirtualMachineWithConfigContextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**status** | Option<**String**> | * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `decommissioning` - Decommissioning * `paused` - Paused | [optional]
**site** | Option<[**crate::models::PatchedWritableVlanRequestSite**](PatchedWritableVLANRequest_site.md)> |  | [optional]
**cluster** | Option<[**crate::models::DeviceWithConfigContextRequestCluster**](DeviceWithConfigContextRequest_cluster.md)> |  | [optional]
**device** | Option<[**crate::models::DeviceBayRequestInstalledDevice**](DeviceBayRequest_installed_device.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**role** | Option<[**crate::models::PatchedWritableVirtualMachineWithConfigContextRequestRole**](PatchedWritableVirtualMachineWithConfigContextRequest_role.md)> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**platform** | Option<[**crate::models::DeviceTypeRequestDefaultPlatform**](DeviceTypeRequest_default_platform.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::DeviceWithConfigContextRequestPrimaryIp4**](DeviceWithConfigContextRequest_primary_ip4.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::DeviceWithConfigContextRequestPrimaryIp4**](DeviceWithConfigContextRequest_primary_ip4.md)> |  | [optional]
**vcpus** | Option<**f64**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**config_template** | Option<[**crate::models::DeviceRoleRequestConfigTemplate**](DeviceRoleRequest_config_template.md)> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> | Local config context data takes precedence over source contexts in the final rendered config context | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


