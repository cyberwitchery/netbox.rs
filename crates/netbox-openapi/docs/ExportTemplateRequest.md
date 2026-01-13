# ExportTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**environment_params** | Option<[**serde_json::Value**](.md)> | Any <a href=\"https://jinja.palletsprojects.com/en/stable/api/#jinja2.Environment\">additional parameters</a> to pass when constructing the Jinja environment | [optional]
**template_code** | **String** | Jinja template code. | 
**mime_type** | Option<**String**> | Defaults to <code>text/plain; charset=utf-8</code> | [optional]
**file_name** | Option<**String**> | Filename to give to the rendered export file | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**as_attachment** | Option<**bool**> | Download file as attachment | [optional]
**data_source** | Option<[**crate::models::ConfigContextProfileRequestDataSource**](ConfigContextProfileRequest_data_source.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


