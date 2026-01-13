# EventRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**event_types** | **Vec<String>** | The types of event which will trigger this rule. | 
**conditions** | Option<[**serde_json::Value**](.md)> | A set of conditions which determine whether the event will be generated. | [optional]
**action_type** | **String** | * `webhook` - Webhook * `script` - Script * `notification` - Notification | 
**action_object_type** | **String** |  | 
**action_object_id** | Option<**i64**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


