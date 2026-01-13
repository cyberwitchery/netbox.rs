# CustomFieldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object_types** | **Vec<String>** |  | 
**r#type** | **String** | * `text` - Text * `longtext` - Text (long) * `integer` - Integer * `decimal` - Decimal * `boolean` - Boolean (true/false) * `date` - Date * `datetime` - Date & time * `url` - URL * `json` - JSON * `select` - Selection * `multiselect` - Multiple selection * `object` - Object * `multiobject` - Multiple objects | 
**related_object_type** | Option<**String**> |  | [optional]
**name** | **String** | Internal field name | 
**label** | Option<**String**> | Name of the field as displayed to users (if not provided, 'the field's name will be used) | [optional]
**group_name** | Option<**String**> | Custom fields within the same group will be displayed together | [optional]
**description** | Option<**String**> |  | [optional]
**required** | Option<**bool**> | This field is required when creating new objects or editing an existing object. | [optional]
**unique** | Option<**bool**> | The value of this field must be unique for the assigned object | [optional]
**search_weight** | Option<**i32**> | Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored. | [optional]
**filter_logic** | Option<**String**> | * `disabled` - Disabled * `loose` - Loose * `exact` - Exact | [optional]
**ui_visible** | Option<**String**> | * `always` - Always * `if-set` - If set * `hidden` - Hidden | [optional]
**ui_editable** | Option<**String**> | * `yes` - Yes * `no` - No * `hidden` - Hidden | [optional]
**is_cloneable** | Option<**bool**> | Replicate this value when cloning objects | [optional]
**default** | Option<[**serde_json::Value**](.md)> | Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. \"Foo\"). | [optional]
**related_object_filter** | Option<[**serde_json::Value**](.md)> | Filter the object selection choices using a query_params dict (must be a JSON value).Encapsulate strings with double quotes (e.g. \"Foo\"). | [optional]
**weight** | Option<**i32**> | Fields with higher weights appear lower in a form. | [optional]
**validation_minimum** | Option<**f64**> | Minimum allowed value (for numeric fields) | [optional]
**validation_maximum** | Option<**f64**> | Maximum allowed value (for numeric fields) | [optional]
**validation_regex** | Option<**String**> | Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters. | [optional]
**choice_set** | Option<[**crate::models::CustomFieldRequestChoiceSet**](CustomFieldRequest_choice_set.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


