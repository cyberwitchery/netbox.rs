# BackgroundTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**url** | **String** |  | [readonly]
**description** | **String** |  | 
**origin** | **String** |  | 
**func_name** | **String** |  | 
**args** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**kwargs** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**result** | **String** |  | 
**timeout** | **i32** |  | 
**result_ttl** | **i32** |  | 
**created_at** | **String** |  | 
**enqueued_at** | **String** |  | 
**started_at** | **String** |  | 
**ended_at** | **String** |  | 
**worker_name** | **String** |  | 
**position** | **i32** |  | [readonly]
**status** | **String** |  | [readonly]
**meta** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**last_heartbeat** | **String** |  | 
**is_finished** | **bool** |  | 
**is_queued** | **bool** |  | 
**is_failed** | **bool** |  | 
**is_started** | **bool** |  | 
**is_deferred** | **bool** |  | 
**is_canceled** | **bool** |  | 
**is_scheduled** | **bool** |  | 
**is_stopped** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


