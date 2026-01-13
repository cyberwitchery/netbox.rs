# WritableFrontPortTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_type** | Option<[**crate::models::ConsolePortTemplateRequestDeviceType**](ConsolePortTemplateRequest_device_type.md)> |  | [optional]
**module_type** | Option<[**crate::models::ConsolePortTemplateRequestModuleType**](ConsolePortTemplateRequest_module_type.md)> |  | [optional]
**name** | **String** | {module} is accepted as a substitution for the module bay position when attached to a module type. | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | **String** | * `8p8c` - 8P8C * `8p6c` - 8P6C * `8p4c` - 8P4C * `8p2c` - 8P2C * `6p6c` - 6P6C * `6p4c` - 6P4C * `6p2c` - 6P2C * `4p4c` - 4P4C * `4p2c` - 4P2C * `gg45` - GG45 * `tera-4p` - TERA 4P * `tera-2p` - TERA 2P * `tera-1p` - TERA 1P * `110-punch` - 110 Punch * `bnc` - BNC * `f` - F Connector * `n` - N Connector * `mrj21` - MRJ21 * `fc` - FC * `fc-pc` - FC/PC * `fc-upc` - FC/UPC * `fc-apc` - FC/APC * `lc` - LC * `lc-pc` - LC/PC * `lc-upc` - LC/UPC * `lc-apc` - LC/APC * `lsh` - LSH * `lsh-pc` - LSH/PC * `lsh-upc` - LSH/UPC * `lsh-apc` - LSH/APC * `lx5` - LX.5 * `lx5-pc` - LX.5/PC * `lx5-upc` - LX.5/UPC * `lx5-apc` - LX.5/APC * `mpo` - MPO * `mtrj` - MTRJ * `sc` - SC * `sc-pc` - SC/PC * `sc-upc` - SC/UPC * `sc-apc` - SC/APC * `st` - ST * `cs` - CS * `sn` - SN * `sma-905` - SMA 905 * `sma-906` - SMA 906 * `urm-p2` - URM-P2 * `urm-p4` - URM-P4 * `urm-p8` - URM-P8 * `splice` - Splice * `usb-a` - USB Type A * `usb-b` - USB Type B * `usb-c` - USB Type C * `usb-mini-a` - USB Mini A * `usb-mini-b` - USB Mini B * `usb-micro-a` - USB Micro A * `usb-micro-b` - USB Micro B * `usb-micro-ab` - USB Micro AB * `other` - Other | 
**color** | Option<**String**> |  | [optional]
**rear_port** | [**crate::models::FrontPortTemplateRequestRearPort**](FrontPortTemplateRequest_rear_port.md) |  | 
**rear_port_position** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


