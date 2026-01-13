# WirelessLanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ssid** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**group** | Option<[**crate::models::PatchedWritableWirelessLanRequestGroup**](PatchedWritableWirelessLANRequest_group.md)> |  | [optional]
**status** | Option<**String**> | * `active` - Active * `reserved` - Reserved * `disabled` - Disabled * `deprecated` - Deprecated | [optional]
**vlan** | Option<[**crate::models::InterfaceRequestUntaggedVlan**](InterfaceRequest_untagged_vlan.md)> |  | [optional]
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**auth_type** | Option<**String**> | * `open` - Open * `wep` - WEP * `wpa-personal` - WPA Personal (PSK) * `wpa-enterprise` - WPA Enterprise | [optional]
**auth_cipher** | Option<**String**> | * `auto` - Auto * `tkip` - TKIP * `aes` - AES | [optional]
**auth_psk** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


