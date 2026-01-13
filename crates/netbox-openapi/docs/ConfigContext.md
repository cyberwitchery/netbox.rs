# ConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**weight** | Option<**i32**> |  | [optional]
**profile** | Option<[**crate::models::BriefConfigContextProfile**](BriefConfigContextProfile.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**regions** | Option<[**Vec<crate::models::Region>**](Region.md)> |  | [optional]
**site_groups** | Option<[**Vec<crate::models::SiteGroup>**](SiteGroup.md)> |  | [optional]
**sites** | Option<[**Vec<crate::models::Site>**](Site.md)> |  | [optional]
**locations** | Option<[**Vec<crate::models::Location>**](Location.md)> |  | [optional]
**device_types** | Option<[**Vec<crate::models::DeviceType>**](DeviceType.md)> |  | [optional]
**roles** | Option<[**Vec<crate::models::DeviceRole>**](DeviceRole.md)> |  | [optional]
**platforms** | Option<[**Vec<crate::models::Platform>**](Platform.md)> |  | [optional]
**cluster_types** | Option<[**Vec<crate::models::ClusterType>**](ClusterType.md)> |  | [optional]
**cluster_groups** | Option<[**Vec<crate::models::ClusterGroup>**](ClusterGroup.md)> |  | [optional]
**clusters** | Option<[**Vec<crate::models::Cluster>**](Cluster.md)> |  | [optional]
**tenant_groups** | Option<[**Vec<crate::models::TenantGroup>**](TenantGroup.md)> |  | [optional]
**tenants** | Option<[**Vec<crate::models::Tenant>**](Tenant.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**data_source** | Option<[**crate::models::BriefDataSource**](BriefDataSource.md)> |  | [optional]
**data_path** | **String** | Path to remote file (relative to data source root) | [readonly]
**data_file** | [**crate::models::BriefDataFile**](BriefDataFile.md) |  | [readonly]
**data_synced** | Option<**String**> |  | [readonly]
**data** | Option<[**serde_json::Value**](.md)> |  | 
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


