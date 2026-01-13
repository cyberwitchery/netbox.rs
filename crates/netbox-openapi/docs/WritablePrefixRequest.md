# WritablePrefixRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prefix** | **String** |  | 
**vrf** | Option<[**crate::models::IpAddressRequestVrf**](IPAddressRequest_vrf.md)> |  | [optional]
**scope_type** | Option<**String**> |  | [optional]
**scope_id** | Option<**i32**> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**vlan** | Option<[**crate::models::InterfaceRequestUntaggedVlan**](InterfaceRequest_untagged_vlan.md)> |  | [optional]
**status** | Option<**String**> | Operational status of this prefix  * `container` - Container * `active` - Active * `reserved` - Reserved * `deprecated` - Deprecated | [optional]
**role** | Option<[**crate::models::IpRangeRequestRole**](IPRangeRequest_role.md)> |  | [optional]
**is_pool** | Option<**bool**> | All IP addresses within this prefix are considered usable | [optional]
**mark_utilized** | Option<**bool**> | Treat as fully utilized | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


