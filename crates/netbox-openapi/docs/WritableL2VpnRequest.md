# WritableL2VpnRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**i64**> |  | [optional]
**name** | **String** |  | 
**slug** | **String** |  | 
**r#type** | **String** | * `vpws` - VPWS * `vpls` - VPLS * `vxlan` - VXLAN * `vxlan-evpn` - VXLAN-EVPN * `mpls-evpn` - MPLS EVPN * `pbb-evpn` - PBB EVPN * `evpn-vpws` - EVPN VPWS * `epl` - EPL * `evpl` - EVPL * `ep-lan` - Ethernet Private LAN * `evp-lan` - Ethernet Virtual Private LAN * `ep-tree` - Ethernet Private Tree * `evp-tree` - Ethernet Virtual Private Tree * `spb` - SPB | 
**status** | Option<**String**> | * `active` - Active * `planned` - Planned * `decommissioning` - Decommissioning | [optional]
**import_targets** | Option<**Vec<i32>**> |  | [optional]
**export_targets** | Option<**Vec<i32>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


