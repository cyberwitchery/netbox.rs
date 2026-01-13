# VmInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**virtual_machine** | [**crate::models::PatchedVirtualDiskRequestVirtualMachine**](PatchedVirtualDiskRequest_virtual_machine.md) |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<[**crate::models::NestedVmInterfaceRequest**](NestedVMInterfaceRequest.md)> |  | [optional]
**bridge** | Option<[**crate::models::NestedVmInterfaceRequest**](NestedVMInterfaceRequest.md)> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**primary_mac_address** | Option<[**crate::models::InterfaceRequestPrimaryMacAddress**](InterfaceRequest_primary_mac_address.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<**String**> | * `access` - Access * `tagged` - Tagged * `tagged-all` - Tagged (All) * `q-in-q` - Q-in-Q (802.1ad) | [optional]
**untagged_vlan** | Option<[**crate::models::InterfaceRequestUntaggedVlan**](InterfaceRequest_untagged_vlan.md)> |  | [optional]
**tagged_vlans** | Option<**Vec<i32>**> |  | [optional]
**qinq_svlan** | Option<[**crate::models::InterfaceRequestUntaggedVlan**](InterfaceRequest_untagged_vlan.md)> |  | [optional]
**vlan_translation_policy** | Option<[**crate::models::InterfaceRequestVlanTranslationPolicy**](InterfaceRequest_vlan_translation_policy.md)> |  | [optional]
**vrf** | Option<[**crate::models::IpAddressRequestVrf**](IPAddressRequest_vrf.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


