# PatchedWritableClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**r#type** | Option<[**crate::models::ClusterRequestType**](ClusterRequest_type.md)> |  | [optional]
**group** | Option<[**crate::models::ClusterRequestGroup**](ClusterRequest_group.md)> |  | [optional]
**status** | Option<**String**> | * `planned` - Planned * `staging` - Staging * `active` - Active * `decommissioning` - Decommissioning * `offline` - Offline | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


