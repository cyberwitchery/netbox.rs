# CustomLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**link_text** | **String** | Jinja2 template code for link text | 
**link_url** | **String** | Jinja2 template code for link URL | 
**weight** | Option<**i32**> |  | [optional]
**group_name** | Option<**String**> | Links with the same group will appear as a dropdown menu | [optional]
**button_class** | Option<**String**> | The class of the first link in a group will be used for the dropdown button  * `default` - Default * `blue` - Blue * `indigo` - Indigo * `purple` - Purple * `pink` - Pink * `red` - Red * `orange` - Orange * `yellow` - Yellow * `green` - Green * `teal` - Teal * `cyan` - Cyan * `gray` - Gray * `black` - Black * `white` - White * `ghost-dark` - Link | [optional]
**new_window** | Option<**bool**> | Force link to open in a new window | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


