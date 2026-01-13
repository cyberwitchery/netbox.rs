# WritableVirtualDeviceContextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**device** | [**crate::models::BriefInterfaceRequestDevice**](BriefInterfaceRequest_device.md) |  | 
**identifier** | Option<**i32**> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::DeviceWithConfigContextRequestPrimaryIp4**](DeviceWithConfigContextRequest_primary_ip4.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::DeviceWithConfigContextRequestPrimaryIp4**](DeviceWithConfigContextRequest_primary_ip4.md)> |  | [optional]
**status** | **String** | * `active` - Active * `planned` - Planned * `offline` - Offline | 
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


