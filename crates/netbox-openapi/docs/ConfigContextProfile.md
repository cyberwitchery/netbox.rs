# ConfigContextProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**schema** | Option<[**serde_json::Value**](.md)> | A JSON schema specifying the structure of the context data for this profile | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**data_source** | Option<[**crate::models::BriefDataSource**](BriefDataSource.md)> |  | [optional]
**data_path** | **String** | Path to remote file (relative to data source root) | [readonly]
**data_file** | [**crate::models::BriefDataFile**](BriefDataFile.md) |  | [readonly]
**data_synced** | Option<**String**> |  | [readonly]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


