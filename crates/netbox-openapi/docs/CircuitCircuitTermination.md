# CircuitCircuitTermination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**termination_type** | Option<**String**> |  | [optional]
**termination_id** | Option<**i32**> |  | [optional]
**termination** | Option<[**serde_json::Value**](.md)> |  | [readonly]
**port_speed** | Option<**i32**> | Physical circuit speed | [optional]
**upstream_speed** | Option<**i32**> | Upstream speed, if different from port speed | [optional]
**xconnect_id** | Option<**String**> | ID of the local cross-connect | [optional]
**description** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


