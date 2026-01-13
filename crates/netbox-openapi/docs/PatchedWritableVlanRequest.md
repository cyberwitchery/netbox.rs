# PatchedWritableVlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**site** | Option<[**crate::models::PatchedWritableVlanRequestSite**](PatchedWritableVLANRequest_site.md)> |  | [optional]
**group** | Option<[**crate::models::PatchedWritableVlanRequestGroup**](PatchedWritableVLANRequest_group.md)> |  | [optional]
**vid** | Option<**i32**> | Numeric VLAN ID (1-4094) | [optional]
**name** | Option<**String**> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**status** | Option<**String**> | Operational status of this VLAN  * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<[**crate::models::IpRangeRequestRole**](IPRangeRequest_role.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**qinq_role** | Option<**String**> | Customer/service VLAN designation (for Q-in-Q/IEEE 802.1ad)  * `svlan` - Service * `cvlan` - Customer | [optional]
**qinq_svlan** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


