# ExportTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**object_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**environment_params** | Option<[**serde_json::Value**](.md)> | Any <a href=\"https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment\">additional parameters</a> to pass when constructing the Jinja environment | [optional]
**template_code** | **String** | Jinja template code. | 
**mime_type** | Option<**String**> | Defaults to <code>text/plain; charset=utf-8</code> | [optional]
**file_name** | Option<**String**> | Filename to give to the rendered export file | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**as_attachment** | Option<**bool**> | Download file as attachment | [optional]
**data_source** | Option<[**crate::models::BriefDataSource**](BriefDataSource.md)> |  | [optional]
**data_path** | **String** | Path to remote file (relative to data source root) | [readonly]
**data_file** | [**crate::models::BriefDataFile**](BriefDataFile.md) |  | [readonly]
**data_synced** | Option<**String**> |  | [readonly]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


