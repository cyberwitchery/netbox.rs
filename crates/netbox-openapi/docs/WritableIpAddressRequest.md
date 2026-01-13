# WritableIpAddressRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** |  | 
**vrf** | Option<[**crate::models::IpAddressRequestVrf**](IPAddressRequest_vrf.md)> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**status** | Option<**String**> | The operational status of this IP  * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated * `dhcp` - DHCP * `slaac` - SLAAC | [optional]
**role** | Option<**String**> | The functional role of this IP  * `loopback` - Loopback * `secondary` - Secondary * `anycast` - Anycast * `vip` - VIP * `vrrp` - VRRP * `hsrp` - HSRP * `glbp` - GLBP * `carp` - CARP | [optional]
**assigned_object_type** | Option<**String**> |  | [optional]
**assigned_object_id** | Option<**i64**> |  | [optional]
**nat_inside** | Option<**i32**> | The IP for which this address is the \"outside\" IP | [optional]
**dns_name** | Option<**String**> | Hostname or FQDN (not case-sensitive) | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


