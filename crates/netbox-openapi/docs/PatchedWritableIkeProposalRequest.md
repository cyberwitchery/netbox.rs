# PatchedWritableIkeProposalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**authentication_method** | Option<**String**> | * `preshared-keys` - Pre-shared keys * `certificates` - Certificates * `rsa-signatures` - RSA signatures * `dsa-signatures` - DSA signatures | [optional]
**encryption_algorithm** | Option<**String**> | * `aes-128-cbc` - 128-bit AES (CBC) * `aes-128-gcm` - 128-bit AES (GCM) * `aes-192-cbc` - 192-bit AES (CBC) * `aes-192-gcm` - 192-bit AES (GCM) * `aes-256-cbc` - 256-bit AES (CBC) * `aes-256-gcm` - 256-bit AES (GCM) * `3des-cbc` - 3DES * `des-cbc` - DES | [optional]
**authentication_algorithm** | Option<**String**> | * `hmac-sha1` - SHA-1 HMAC * `hmac-sha256` - SHA-256 HMAC * `hmac-sha384` - SHA-384 HMAC * `hmac-sha512` - SHA-512 HMAC * `hmac-md5` - MD5 HMAC | [optional]
**group** | Option<**i32**> | Diffie-Hellman group ID  * `1` - Group 1 * `2` - Group 2 * `5` - Group 5 * `14` - Group 14 * `15` - Group 15 * `16` - Group 16 * `17` - Group 17 * `18` - Group 18 * `19` - Group 19 * `20` - Group 20 * `21` - Group 21 * `22` - Group 22 * `23` - Group 23 * `24` - Group 24 * `25` - Group 25 * `26` - Group 26 * `27` - Group 27 * `28` - Group 28 * `29` - Group 29 * `30` - Group 30 * `31` - Group 31 * `32` - Group 32 * `33` - Group 33 * `34` - Group 34 | [optional]
**sa_lifetime** | Option<**i32**> | Security association lifetime (in seconds) | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


