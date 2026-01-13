# PowerPortTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device_type** | Option<[**crate::models::BriefDeviceType**](BriefDeviceType.md)> |  | [optional]
**module_type** | Option<[**crate::models::BriefModuleType**](BriefModuleType.md)> |  | [optional]
**name** | **String** | {module} is accepted as a substitution for the module bay position when attached to a module type. | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | Option<[**crate::models::PowerPortType**](PowerPort_type.md)> |  | [optional]
**maximum_draw** | Option<**i32**> | Maximum power draw (watts) | [optional]
**allocated_draw** | Option<**i32**> | Allocated power draw (watts) | [optional]
**description** | Option<**String**> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


