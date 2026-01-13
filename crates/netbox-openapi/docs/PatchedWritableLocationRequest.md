# PatchedWritableLocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**slug** | Option<**String**> |  | [optional]
**site** | Option<[**crate::models::DeviceWithConfigContextRequestSite**](DeviceWithConfigContextRequest_site.md)> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**status** | Option<**String**> | * `planned` - Planned * `staging` - Staging * `active` - Active * `decommissioning` - Decommissioning * `retired` - Retired | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**facility** | Option<**String**> | Local facility ID or description | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


