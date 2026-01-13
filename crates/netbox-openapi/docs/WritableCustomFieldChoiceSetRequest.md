# WritableCustomFieldChoiceSetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**base_choices** | Option<**String**> | Base set of predefined choices (optional)  * `IATA` - IATA (Airport codes) * `ISO_3166` - ISO 3166 (Country codes) * `UN_LOCODE` - UN/LOCODE (Location codes) | [optional]
**extra_choices** | [**Vec<Vec<serde_json::Value>>**](array.md) |  | 
**order_alphabetically** | Option<**bool**> | Choices are automatically ordered alphabetically | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


