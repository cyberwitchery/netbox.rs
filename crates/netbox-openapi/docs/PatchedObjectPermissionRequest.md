# PatchedObjectPermissionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**object_types** | Option<**Vec<String>**> |  | [optional]
**actions** | Option<**Vec<String>**> | The list of actions granted by this permission | [optional]
**constraints** | Option<[**serde_json::Value**](.md)> | Queryset filter matching the applicable objects of the selected type(s) | [optional]
**groups** | Option<**Vec<i32>**> |  | [optional]
**users** | Option<**Vec<i32>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


