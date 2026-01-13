# PatchedWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**payload_url** | Option<**String**> | This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body. | [optional]
**http_method** | Option<**String**> | * `GET` - GET * `POST` - POST * `PUT` - PUT * `PATCH` - PATCH * `DELETE` - DELETE | [optional]
**http_content_type** | Option<**String**> | The complete list of official content types is available <a href=\"https://www.iana.org/assignments/media-types/media-types.xhtml\">here</a>. | [optional]
**additional_headers** | Option<**String**> | User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below). | [optional]
**body_template** | Option<**String**> | Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>. | [optional]
**secret** | Option<**String**> | When provided, the request will include a <code>X-Hook-Signature</code> header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request. | [optional]
**ssl_verification** | Option<**bool**> | Enable SSL certificate verification. Disable with caution! | [optional]
**ca_file_path** | Option<**String**> | The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults. | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTagRequest>**](NestedTagRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


