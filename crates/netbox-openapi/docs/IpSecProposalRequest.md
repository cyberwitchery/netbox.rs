# IpSecProposalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**encryption_algorithm** | Option<**String**> | * `aes-128-cbc` - 128-bit AES (CBC) * `aes-128-gcm` - 128-bit AES (GCM) * `aes-192-cbc` - 192-bit AES (CBC) * `aes-192-gcm` - 192-bit AES (GCM) * `aes-256-cbc` - 256-bit AES (CBC) * `aes-256-gcm` - 256-bit AES (GCM) * `3des-cbc` - 3DES * `des-cbc` - DES | [optional]
**authentication_algorithm** | Option<**String**> | * `hmac-sha1` - SHA-1 HMAC * `hmac-sha256` - SHA-256 HMAC * `hmac-sha384` - SHA-384 HMAC * `hmac-sha512` - SHA-512 HMAC * `hmac-md5` - MD5 HMAC | [optional]
**sa_lifetime_seconds** | Option<**i32**> | Security association lifetime (seconds) | [optional]
**sa_lifetime_data** | Option<**i32**> | Security association lifetime (in kilobytes) | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


