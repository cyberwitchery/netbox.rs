# Cable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**url** | **String** |  | [readonly]
**display_url** | **String** |  | [readonly]
**display** | **String** |  | [readonly]
**r#type** | Option<**String**> | * `cat3` - CAT3 * `cat5` - CAT5 * `cat5e` - CAT5e * `cat6` - CAT6 * `cat6a` - CAT6a * `cat7` - CAT7 * `cat7a` - CAT7a * `cat8` - CAT8 * `mrj21-trunk` - MRJ21 Trunk * `dac-active` - Direct Attach Copper (Active) * `dac-passive` - Direct Attach Copper (Passive) * `coaxial` - Coaxial * `mmf` - Multimode Fiber * `mmf-om1` - Multimode Fiber (OM1) * `mmf-om2` - Multimode Fiber (OM2) * `mmf-om3` - Multimode Fiber (OM3) * `mmf-om4` - Multimode Fiber (OM4) * `mmf-om5` - Multimode Fiber (OM5) * `smf` - Single-mode Fiber * `smf-os1` - Single-mode Fiber (OS1) * `smf-os2` - Single-mode Fiber (OS2) * `aoc` - Active Optical Cabling (AOC) * `power` - Power * `usb` - USB | [optional]
**a_terminations** | Option<[**Vec<crate::models::GenericObject>**](GenericObject.md)> |  | [optional]
**b_terminations** | Option<[**Vec<crate::models::GenericObject>**](GenericObject.md)> |  | [optional]
**status** | Option<[**crate::models::CableStatus**](Cable_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BriefTenant**](BriefTenant.md)> |  | [optional]
**label** | Option<**String**> |  | [optional]
**color** | Option<**String**> |  | [optional]
**length** | Option<**f64**> |  | [optional]
**length_unit** | Option<[**crate::models::CableLengthUnit**](Cable_length_unit.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**created** | Option<**String**> |  | [readonly]
**last_updated** | Option<**String**> |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


