# EventRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**object_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**event_types** | **Vec<String>** | The types of event which will trigger this rule. | 
**conditions** | Option<[**serde_json::Value**](.md)> | A set of conditions which determine whether the event will be generated. | [optional]
**action_type** | [**crate::models::EventRuleActionType**](EventRule_action_type.md) |  | 
**action_object_type** | **String** |  | 
**action_object_id** | Option<**i64**> |  | [optional]
**action_object** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**description** | Option<**String**> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


