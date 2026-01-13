# Interface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**device** | [**crate::models::BriefDevice**](BriefDevice.md) |  | 
**vdcs** | Option<[**Vec<crate::models::VirtualDeviceContext>**](VirtualDeviceContext.md)> |  | [optional]
**module** | Option<[**crate::models::BriefModule**](BriefModule.md)> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | [**crate::models::InterfaceType**](Interface_type.md) |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | [optional]
**bridge** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | [optional]
**lag** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [readonly]
**primary_mac_address** | Option<[**crate::models::BriefMacAddress**](BriefMACAddress.md)> |  | [optional]
**mac_addresses** | Option<[**Vec<crate::models::BriefMacAddress>**](BriefMACAddress.md)> |  | [readonly]
**speed** | Option<**i32**> |  | [optional]
**duplex** | Option<[**crate::models::InterfaceDuplex**](Interface_duplex.md)> |  | [optional]
**wwn** | Option<**String**> |  | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<[**crate::models::InterfaceMode**](Interface_mode.md)> |  | [optional]
**rf_role** | Option<[**crate::models::InterfaceRfRole**](Interface_rf_role.md)> |  | [optional]
**rf_channel** | Option<[**crate::models::InterfaceRfChannel**](Interface_rf_channel.md)> |  | [optional]
**poe_mode** | Option<[**crate::models::InterfacePoeMode**](Interface_poe_mode.md)> |  | [optional]
**poe_type** | Option<[**crate::models::InterfacePoeType**](Interface_poe_type.md)> |  | [optional]
**rf_channel_frequency** | Option<**f64**> | Populated by selected channel (if set) | [optional]
**rf_channel_width** | Option<**f64**> | Populated by selected channel (if set) | [optional]
**tx_power** | Option<**i32**> |  | [optional]
**untagged_vlan** | Option<[**crate::models::BriefVlan**](BriefVLAN.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::Vlan>**](VLAN.md)> |  | [optional]
**qinq_svlan** | Option<[**crate::models::BriefVlan**](BriefVLAN.md)> |  | [optional]
**vlan_translation_policy** | Option<[**crate::models::BriefVlanTranslationPolicy**](BriefVLANTranslationPolicy.md)> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::BriefCable**](BriefCable.md)> |  | [readonly]
**cable_end** | **String** |  | [readonly]
**wireless_link** | Option<[**crate::models::NestedWirelessLink**](NestedWirelessLink.md)> |  | [readonly]
**link_peers** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | [readonly]
**link_peers_type** | Option<**String**> | Return the type of the peer link terminations, or None. | [readonly]
**wireless_lans** | Option<[**Vec<crate::models::WirelessLan>**](WirelessLAN.md)> |  | [optional]
**vrf** | Option<[**crate::models::BriefVrf**](BriefVRF.md)> |  | [optional]
**l2vpn_termination** | Option<[**crate::models::BriefL2VpnTermination**](BriefL2VPNTermination.md)> |  | [readonly]
**connected_endpoints** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [readonly]
**connected_endpoints_type** | Option<**String**> |  | [readonly]
**connected_endpoints_reachable** | **bool** |  | [readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]
**count_ipaddresses** | **i32** |  | [readonly]
**count_fhrp_groups** | **i32** |  | [readonly]
**_occupied** | **bool** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


