# Vlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**site** | Option<[**crate::models::BriefSite**](BriefSite.md)> |  | [optional]
**group** | Option<[**crate::models::BriefVlanGroup**](BriefVLANGroup.md)> |  | [optional]
**vid** | **i32** | Numeric VLAN ID (1-4094) | 
**name** | **String** |  | 
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**status** | Option<[**crate::models::IpRangeStatus**](IPRange_status.md)> |  | [optional]
**role** | Option<[**crate::models::BriefRole**](BriefRole.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**qinq_role** | Option<[**crate::models::VlanQinqRole**](VLAN_qinq_role.md)> |  | [optional]
**qinq_svlan** | Option<[**crate::models::NestedVlan**](NestedVLAN.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**l2vpn_termination** | Option<[**crate::models::BriefL2VpnTermination**](BriefL2VPNTermination.md)> |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**prefix_count** | **i64** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


