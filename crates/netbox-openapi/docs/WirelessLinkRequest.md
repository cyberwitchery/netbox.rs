# WirelessLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interface_a** | [**crate::models::PatchedWritableVirtualCircuitTerminationRequestInterface**](PatchedWritableVirtualCircuitTerminationRequest_interface.md) |  | 
**interface_b** | [**crate::models::PatchedWritableVirtualCircuitTerminationRequestInterface**](PatchedWritableVirtualCircuitTerminationRequest_interface.md) |  | 
**ssid** | Option<**String**> |  | [optional]
**status** | Option<**String**> | * `connected` - Connected * `planned` - Planned * `decommissioning` - Decommissioning | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**auth_type** | Option<**String**> | * `open` - Open * `wep` - WEP * `wpa-personal` - WPA Personal (PSK) * `wpa-enterprise` - WPA Enterprise | [optional]
**auth_cipher** | Option<**String**> | * `auto` - Auto * `tkip` - TKIP * `aes` - AES | [optional]
**auth_psk** | Option<**String**> |  | [optional]
**distance** | Option<**f64**> |  | [optional]
**distance_unit** | Option<**String**> | * `km` - Kilometers * `m` - Meters * `mi` - Miles * `ft` - Feet | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


