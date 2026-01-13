# WritableRackRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**facility_id** | Option<**String**> |  | [optional]
**site** | [**crate::models::DeviceWithConfigContextRequestSite**](DeviceWithConfigContextRequest_site.md) |  | 
**location** | Option<[**crate::models::DeviceWithConfigContextRequestLocation**](DeviceWithConfigContextRequest_location.md)> |  | [optional]
**tenant** | Option<[**crate::models::AsnRangeRequestTenant**](ASNRangeRequest_tenant.md)> |  | [optional]
**status** | Option<**String**> | * `reserved` - Reserved * `available` - Available * `planned` - Planned * `active` - Active * `deprecated` - Deprecated | [optional]
**role** | Option<[**crate::models::PatchedWritableRackRequestRole**](PatchedWritableRackRequest_role.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this rack | [optional]
**rack_type** | Option<[**crate::models::PatchedWritableRackRequestRackType**](PatchedWritableRackRequest_rack_type.md)> |  | [optional]
**form_factor** | Option<**String**> | * `2-post-frame` - 2-post frame * `4-post-frame` - 4-post frame * `4-post-cabinet` - 4-post cabinet * `wall-frame` - Wall-mounted frame * `wall-frame-vertical` - Wall-mounted frame (vertical) * `wall-cabinet` - Wall-mounted cabinet * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical) | [optional]
**width** | Option<**i32**> | Rail-to-rail width  * `10` - 10 inches * `19` - 19 inches * `21` - 21 inches * `23` - 23 inches | [optional]
**u_height** | Option<**i32**> | Height in rack units | [optional]
**starting_unit** | Option<**i32**> | Starting unit for rack | [optional]
**weight** | Option<**f64**> |  | [optional]
**max_weight** | Option<**i32**> | Maximum load capacity for the rack | [optional]
**weight_unit** | Option<**String**> | * `kg` - Kilograms * `g` - Grams * `lb` - Pounds * `oz` - Ounces | [optional]
**desc_units** | Option<**bool**> | Units are numbered top-to-bottom | [optional]
**outer_width** | Option<**i32**> | Outer dimension of rack (width) | [optional]
**outer_height** | Option<**i32**> | Outer dimension of rack (height) | [optional]
**outer_depth** | Option<**i32**> | Outer dimension of rack (depth) | [optional]
**outer_unit** | Option<**String**> | * `mm` - Millimeters * `in` - Inches | [optional]
**mounting_depth** | Option<**i32**> | Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails. | [optional]
**airflow** | Option<**String**> | * `front-to-rear` - Front to rear * `rear-to-front` - Rear to front | [optional]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


