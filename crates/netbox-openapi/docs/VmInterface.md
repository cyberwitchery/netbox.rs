# VmInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**virtual_machine** | [**crate::models::BriefVirtualMachine**](BriefVirtualMachine.md) |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<[**crate::models::NestedVmInterface**](NestedVMInterface.md)> |  | [optional]
**bridge** | Option<[**crate::models::NestedVmInterface**](NestedVMInterface.md)> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [readonly]
**primary_mac_address** | Option<[**crate::models::BriefMacAddress**](BriefMACAddress.md)> |  | [optional]
**mac_addresses** | Option<[**Vec<crate::models::BriefMacAddress>**](BriefMACAddress.md)> |  | [readonly]
**description** | Option<**String**> |  | [optional]
**mode** | Option<[**crate::models::InterfaceMode**](Interface_mode.md)> |  | [optional]
**untagged_vlan** | Option<[**crate::models::BriefVlan**](BriefVLAN.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::Vlan>**](VLAN.md)> |  | [optional]
**qinq_svlan** | Option<[**crate::models::BriefVlan**](BriefVLAN.md)> |  | [optional]
**vlan_translation_policy** | Option<[**crate::models::BriefVlanTranslationPolicy**](BriefVLANTranslationPolicy.md)> |  | [optional]
**vrf** | Option<[**crate::models::BriefVrf**](BriefVRF.md)> |  | [optional]
**l2vpn_termination** | Option<[**crate::models::BriefL2VpnTermination**](BriefL2VPNTermination.md)> |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**count_ipaddresses** | **i32** |  | [readonly]
**count_fhrp_groups** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


