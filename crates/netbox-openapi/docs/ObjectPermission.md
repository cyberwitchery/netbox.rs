# ObjectPermission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**object_types** | **Vec<String>** |  | 
**actions** | **Vec<String>** | The list of actions granted by this permission | 
**constraints** | Option<[**serde_json::Value**](.md)> | Queryset filter matching the applicable objects of the selected type(s) | [optional]
**groups** | Option<[**Vec<crate::models::NestedGroup>**](NestedGroup.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::NestedUser>**](NestedUser.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


