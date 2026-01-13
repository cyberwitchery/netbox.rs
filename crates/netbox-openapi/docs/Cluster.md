# Cluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**r#type** | [**crate::models::BriefClusterType**](BriefClusterType.md) |  | 
**group** | Option<[**crate::models::BriefClusterGroup**](BriefClusterGroup.md)> |  | [optional]
**status** | Option<[**crate::models::ClusterStatus**](Cluster_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**scope** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**device_count** | **i64** |  | [readonly]
**virtualmachine_count** | **i64** |  | [readonly]
**allocated_vcpus** | **f64** |  | [readonly]
**allocated_memory** | **i32** |  | [readonly]
**allocated_disk** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


