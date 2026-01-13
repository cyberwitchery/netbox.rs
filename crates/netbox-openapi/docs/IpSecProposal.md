# IpSecProposal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**encryption_algorithm** | Option<[**crate::models::IkeProposalEncryptionAlgorithm**](IKEProposal_encryption_algorithm.md)> |  | [optional]
**authentication_algorithm** | Option<[**crate::models::IkeProposalAuthenticationAlgorithm**](IKEProposal_authentication_algorithm.md)> |  | [optional]
**sa_lifetime_seconds** | Option<**i32**> | Security association lifetime (seconds) | [optional]
**sa_lifetime_data** | Option<**i32**> | Security association lifetime (in kilobytes) | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


