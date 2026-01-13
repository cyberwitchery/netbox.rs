# \CoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**core_background_queues_retrieve**](CoreApi.md#core_background_queues_retrieve) | **GET** /api/core/background-queues/ | 
[**core_background_queues_retrieve_by_name**](CoreApi.md#core_background_queues_retrieve_by_name) | **GET** /api/core/background-queues/{name}/ | 
[**core_background_tasks_delete_create**](CoreApi.md#core_background_tasks_delete_create) | **POST** /api/core/background-tasks/{id}/delete/ | 
[**core_background_tasks_enqueue_create**](CoreApi.md#core_background_tasks_enqueue_create) | **POST** /api/core/background-tasks/{id}/enqueue/ | 
[**core_background_tasks_requeue_create**](CoreApi.md#core_background_tasks_requeue_create) | **POST** /api/core/background-tasks/{id}/requeue/ | 
[**core_background_tasks_retrieve**](CoreApi.md#core_background_tasks_retrieve) | **GET** /api/core/background-tasks/ | 
[**core_background_tasks_retrieve_by_id**](CoreApi.md#core_background_tasks_retrieve_by_id) | **GET** /api/core/background-tasks/{id}/ | 
[**core_background_tasks_stop_create**](CoreApi.md#core_background_tasks_stop_create) | **POST** /api/core/background-tasks/{id}/stop/ | 
[**core_background_workers_retrieve**](CoreApi.md#core_background_workers_retrieve) | **GET** /api/core/background-workers/ | 
[**core_background_workers_retrieve_by_name**](CoreApi.md#core_background_workers_retrieve_by_name) | **GET** /api/core/background-workers/{name}/ | 
[**core_data_files_list**](CoreApi.md#core_data_files_list) | **GET** /api/core/data-files/ | 
[**core_data_files_retrieve**](CoreApi.md#core_data_files_retrieve) | **GET** /api/core/data-files/{id}/ | 
[**core_data_sources_bulk_destroy**](CoreApi.md#core_data_sources_bulk_destroy) | **DELETE** /api/core/data-sources/ | 
[**core_data_sources_bulk_partial_update**](CoreApi.md#core_data_sources_bulk_partial_update) | **PATCH** /api/core/data-sources/ | 
[**core_data_sources_bulk_update**](CoreApi.md#core_data_sources_bulk_update) | **PUT** /api/core/data-sources/ | 
[**core_data_sources_create**](CoreApi.md#core_data_sources_create) | **POST** /api/core/data-sources/ | 
[**core_data_sources_destroy**](CoreApi.md#core_data_sources_destroy) | **DELETE** /api/core/data-sources/{id}/ | 
[**core_data_sources_list**](CoreApi.md#core_data_sources_list) | **GET** /api/core/data-sources/ | 
[**core_data_sources_partial_update**](CoreApi.md#core_data_sources_partial_update) | **PATCH** /api/core/data-sources/{id}/ | 
[**core_data_sources_retrieve**](CoreApi.md#core_data_sources_retrieve) | **GET** /api/core/data-sources/{id}/ | 
[**core_data_sources_sync_create**](CoreApi.md#core_data_sources_sync_create) | **POST** /api/core/data-sources/{id}/sync/ | 
[**core_data_sources_update**](CoreApi.md#core_data_sources_update) | **PUT** /api/core/data-sources/{id}/ | 
[**core_jobs_list**](CoreApi.md#core_jobs_list) | **GET** /api/core/jobs/ | 
[**core_jobs_retrieve**](CoreApi.md#core_jobs_retrieve) | **GET** /api/core/jobs/{id}/ | 
[**core_object_changes_list**](CoreApi.md#core_object_changes_list) | **GET** /api/core/object-changes/ | 
[**core_object_changes_retrieve**](CoreApi.md#core_object_changes_retrieve) | **GET** /api/core/object-changes/{id}/ | 
[**core_object_types_list**](CoreApi.md#core_object_types_list) | **GET** /api/core/object-types/ | 
[**core_object_types_retrieve**](CoreApi.md#core_object_types_retrieve) | **GET** /api/core/object-types/{id}/ | 



## core_background_queues_retrieve

> ::std::collections::HashMap<String, serde_json::Value> core_background_queues_retrieve()


Retrieve a list of RQ Queues. Note: Queue names are not URL safe, so not returning a detail view.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_queues_retrieve_by_name

> ::std::collections::HashMap<String, serde_json::Value> core_background_queues_retrieve_by_name(name)


Retrieve a list of RQ Queues. Note: Queue names are not URL safe, so not returning a detail view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_tasks_delete_create

> crate::models::BackgroundTask core_background_tasks_delete_create(id, background_task_request)


Retrieve a list of RQ Tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**background_task_request** | [**BackgroundTaskRequest**](BackgroundTaskRequest.md) |  | [required] |

### Return type

[**crate::models::BackgroundTask**](BackgroundTask.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_tasks_enqueue_create

> crate::models::BackgroundTask core_background_tasks_enqueue_create(id, background_task_request)


Retrieve a list of RQ Tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**background_task_request** | [**BackgroundTaskRequest**](BackgroundTaskRequest.md) |  | [required] |

### Return type

[**crate::models::BackgroundTask**](BackgroundTask.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_tasks_requeue_create

> crate::models::BackgroundTask core_background_tasks_requeue_create(id, background_task_request)


Retrieve a list of RQ Tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**background_task_request** | [**BackgroundTaskRequest**](BackgroundTaskRequest.md) |  | [required] |

### Return type

[**crate::models::BackgroundTask**](BackgroundTask.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_tasks_retrieve

> ::std::collections::HashMap<String, serde_json::Value> core_background_tasks_retrieve()


Retrieve a list of RQ Tasks.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_tasks_retrieve_by_id

> ::std::collections::HashMap<String, serde_json::Value> core_background_tasks_retrieve_by_id(id)


Retrieve a list of RQ Tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_tasks_stop_create

> crate::models::BackgroundTask core_background_tasks_stop_create(id, background_task_request)


Retrieve a list of RQ Tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**background_task_request** | [**BackgroundTaskRequest**](BackgroundTaskRequest.md) |  | [required] |

### Return type

[**crate::models::BackgroundTask**](BackgroundTask.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_workers_retrieve

> ::std::collections::HashMap<String, serde_json::Value> core_background_workers_retrieve()


Retrieve a list of RQ Workers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_background_workers_retrieve_by_name

> ::std::collections::HashMap<String, serde_json::Value> core_background_workers_retrieve_by_name(name)


Retrieve a list of RQ Workers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_files_list

> crate::models::PaginatedDataFileList core_data_files_list(created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, hash, hash__empty, hash__ic, hash__ie, hash__iew, hash__iregex, hash__isw, hash__n, hash__nic, hash__nie, hash__niew, hash__nisw, hash__regex, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, offset, ordering, path, path__empty, path__ic, path__ie, path__iew, path__iregex, path__isw, path__n, path__nic, path__nie, path__niew, path__nisw, path__regex, q, size, size__empty, size__gt, size__gte, size__lt, size__lte, size__n, source, source__n, source_id, source_id__n, updated_by_request)


Get a list of data file objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created_by_request** | Option<**uuid::Uuid**> |  |  |
**hash** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__empty** | Option<**bool**> |  |  |
**hash__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__n** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**hash__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**last_updated** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__n** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified_by_request** | Option<**uuid::Uuid**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**path** | Option<[**Vec<String>**](String.md)> |  |  |
**path__empty** | Option<**bool**> |  |  |
**path__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**path__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**path__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**path__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**path__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**path__n** | Option<[**Vec<String>**](String.md)> |  |  |
**path__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**path__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**path__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**path__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**path__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**q** | Option<**String**> |  |  |
**size** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__empty** | Option<**bool**> |  |  |
**size__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**size__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**source** | Option<[**Vec<String>**](String.md)> | Data source (name) |  |
**source__n** | Option<[**Vec<String>**](String.md)> | Data source (name) |  |
**source_id** | Option<[**Vec<i32>**](i32.md)> | Data source (ID) |  |
**source_id__n** | Option<[**Vec<i32>**](i32.md)> | Data source (ID) |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::PaginatedDataFileList**](PaginatedDataFileList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_files_retrieve

> crate::models::DataFile core_data_files_retrieve(id)


Get a data file object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data file. | [required] |

### Return type

[**crate::models::DataFile**](DataFile.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_bulk_destroy

> core_data_sources_bulk_destroy(data_source_request)


Delete a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_source_request** | [**Vec<crate::models::DataSourceRequest>**](DataSourceRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_bulk_partial_update

> Vec<crate::models::DataSource> core_data_sources_bulk_partial_update(data_source_request)


Patch a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_source_request** | [**Vec<crate::models::DataSourceRequest>**](DataSourceRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::DataSource>**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_bulk_update

> Vec<crate::models::DataSource> core_data_sources_bulk_update(data_source_request)


Put a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_source_request** | [**Vec<crate::models::DataSourceRequest>**](DataSourceRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::DataSource>**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_create

> crate::models::DataSource core_data_sources_create(writable_data_source_request)


Post a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_data_source_request** | [**WritableDataSourceRequest**](WritableDataSourceRequest.md) |  | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_destroy

> core_data_sources_destroy(id)


Delete a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_list

> crate::models::PaginatedDataSourceList core_data_sources_list(created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, description, description__empty, description__ic, description__ie, description__iew, description__iregex, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, description__regex, enabled, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_synced, last_synced__empty, last_synced__gt, last_synced__gte, last_synced__lt, last_synced__lte, last_synced__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, name, name__empty, name__ic, name__ie, name__iew, name__iregex, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, name__regex, offset, ordering, q, source_url, source_url__empty, source_url__ic, source_url__ie, source_url__iew, source_url__iregex, source_url__isw, source_url__n, source_url__nic, source_url__nie, source_url__niew, source_url__nisw, source_url__regex, status, status__empty, status__ic, status__ie, status__iew, status__iregex, status__isw, status__n, status__nic, status__nie, status__niew, status__nisw, status__regex, sync_interval, sync_interval__ic, sync_interval__ie, sync_interval__iew, sync_interval__iregex, sync_interval__isw, sync_interval__n, sync_interval__nic, sync_interval__nie, sync_interval__niew, sync_interval__nisw, sync_interval__regex, tag, tag__n, tag_id, tag_id__n, r#type, type__empty, type__ic, type__ie, type__iew, type__iregex, type__isw, type__n, type__nic, type__nie, type__niew, type__nisw, type__regex, updated_by_request)


Get a list of data source objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created_by_request** | Option<**uuid::Uuid**> |  |  |
**description** | Option<[**Vec<String>**](String.md)> |  |  |
**description__empty** | Option<**bool**> |  |  |
**description__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**description__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__n** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**enabled** | Option<**bool**> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**last_synced** | Option<[**Vec<String>**](String.md)> |  |  |
**last_synced__empty** | Option<**bool**> |  |  |
**last_synced__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_synced__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_synced__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_synced__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_synced__n** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__n** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**modified_by_request** | Option<**uuid::Uuid**> |  |  |
**name** | Option<[**Vec<String>**](String.md)> |  |  |
**name__empty** | Option<**bool**> |  |  |
**name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**source_url** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__empty** | Option<**bool**> |  |  |
**source_url__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__n** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**source_url__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**status__empty** | Option<**bool**> |  |  |
**status__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**status__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**status__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**status__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**status__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**status__n** | Option<[**Vec<String>**](String.md)> |  |  |
**status__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**status__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**status__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**status__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**status__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**sync_interval** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__ic** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__ie** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__iew** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__iregex** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__isw** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__nic** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__nie** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__niew** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__nisw** | Option<[**Vec<i32>**](i32.md)> |  |  |
**sync_interval__regex** | Option<[**Vec<i32>**](i32.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**tag__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tag_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**tag_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**r#type** | Option<[**Vec<String>**](String.md)> |  |  |
**type__empty** | Option<**bool**> |  |  |
**type__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**type__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**type__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**type__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**type__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**type__n** | Option<[**Vec<String>**](String.md)> |  |  |
**type__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**type__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**type__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**type__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**type__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::PaginatedDataSourceList**](PaginatedDataSourceList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_partial_update

> crate::models::DataSource core_data_sources_partial_update(id, patched_writable_data_source_request)


Patch a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |
**patched_writable_data_source_request** | Option<[**PatchedWritableDataSourceRequest**](PatchedWritableDataSourceRequest.md)> |  |  |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_retrieve

> crate::models::DataSource core_data_sources_retrieve(id)


Get a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_sync_create

> crate::models::DataSource core_data_sources_sync_create(id, writable_data_source_request)


Enqueue a job to synchronize the DataSource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |
**writable_data_source_request** | [**WritableDataSourceRequest**](WritableDataSourceRequest.md) |  | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_data_sources_update

> crate::models::DataSource core_data_sources_update(id, writable_data_source_request)


Put a data source object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this data source. | [required] |
**writable_data_source_request** | [**WritableDataSourceRequest**](WritableDataSourceRequest.md) |  | [required] |

### Return type

[**crate::models::DataSource**](DataSource.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_jobs_list

> crate::models::PaginatedJobList core_jobs_list(completed, completed__after, completed__before, created, created__after, created__before, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, interval, interval__empty, interval__gt, interval__gte, interval__lt, interval__lte, interval__n, job_id, limit, name, name__empty, name__ic, name__ie, name__iew, name__iregex, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, name__regex, object_id, object_id__empty, object_id__gt, object_id__gte, object_id__lt, object_id__lte, object_id__n, object_type, object_type__n, offset, ordering, q, scheduled, scheduled__after, scheduled__before, started, started__after, started__before, status, status__empty, status__ic, status__ie, status__iew, status__iregex, status__isw, status__n, status__nic, status__nie, status__niew, status__nisw, status__regex, user, user__n)


Retrieve a list of job results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completed** | Option<**String**> |  |  |
**completed__after** | Option<**String**> |  |  |
**completed__before** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__after** | Option<**String**> |  |  |
**created__before** | Option<**String**> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__empty** | Option<**bool**> |  |  |
**interval__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**interval__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**job_id** | Option<**uuid::Uuid**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<[**Vec<String>**](String.md)> |  |  |
**name__empty** | Option<**bool**> |  |  |
**name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**object_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__empty** | Option<**bool**> |  |  |
**object_id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_type** | Option<**i32**> |  |  |
**object_type__n** | Option<**i32**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**scheduled** | Option<**String**> |  |  |
**scheduled__after** | Option<**String**> |  |  |
**scheduled__before** | Option<**String**> |  |  |
**started** | Option<**String**> |  |  |
**started__after** | Option<**String**> |  |  |
**started__before** | Option<**String**> |  |  |
**status** | Option<[**Vec<String>**](String.md)> |  |  |
**status__empty** | Option<**bool**> |  |  |
**status__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**status__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**status__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**status__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**status__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**status__n** | Option<[**Vec<String>**](String.md)> |  |  |
**status__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**status__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**status__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**status__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**status__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**user** | Option<**i32**> |  |  |
**user__n** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedJobList**](PaginatedJobList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_jobs_retrieve

> crate::models::Job core_jobs_retrieve(id)


Retrieve a list of job results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this job. | [required] |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_object_changes_list

> crate::models::PaginatedObjectChangeList core_object_changes_list(action, changed_object_id, changed_object_id__empty, changed_object_id__gt, changed_object_id__gte, changed_object_id__lt, changed_object_id__lte, changed_object_id__n, changed_object_type, changed_object_type__n, changed_object_type_id, changed_object_type_id__n, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, limit, object_repr, object_repr__empty, object_repr__ic, object_repr__ie, object_repr__iew, object_repr__iregex, object_repr__isw, object_repr__n, object_repr__nic, object_repr__nie, object_repr__niew, object_repr__nisw, object_repr__regex, offset, ordering, q, related_object_id, related_object_id__empty, related_object_id__gt, related_object_id__gte, related_object_id__lt, related_object_id__lte, related_object_id__n, related_object_type, related_object_type__n, request_id, time_after, time_before, user, user__n, user_id, user_id__n, user_name, user_name__empty, user_name__ic, user_name__ie, user_name__iew, user_name__iregex, user_name__isw, user_name__n, user_name__nic, user_name__nie, user_name__niew, user_name__nisw, user_name__regex)


Retrieve a list of recent changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<**String**> | * `create` - Created * `update` - Updated * `delete` - Deleted |  |
**changed_object_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**changed_object_id__empty** | Option<**bool**> |  |  |
**changed_object_id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**changed_object_id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**changed_object_id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**changed_object_id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**changed_object_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**changed_object_type** | Option<**String**> |  |  |
**changed_object_type__n** | Option<**String**> |  |  |
**changed_object_type_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**changed_object_type_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**object_repr** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__empty** | Option<**bool**> |  |  |
**object_repr__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__n** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**object_repr__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |
**related_object_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**related_object_id__empty** | Option<**bool**> |  |  |
**related_object_id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**related_object_id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**related_object_id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**related_object_id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**related_object_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**related_object_type** | Option<**i32**> |  |  |
**related_object_type__n** | Option<**i32**> |  |  |
**request_id** | Option<**uuid::Uuid**> |  |  |
**time_after** | Option<**String**> |  |  |
**time_before** | Option<**String**> |  |  |
**user** | Option<[**Vec<String>**](String.md)> | User name |  |
**user__n** | Option<[**Vec<String>**](String.md)> | User name |  |
**user_id** | Option<[**Vec<i32>**](i32.md)> | User (ID) |  |
**user_id__n** | Option<[**Vec<i32>**](i32.md)> | User (ID) |  |
**user_name** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__empty** | Option<**bool**> |  |  |
**user_name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**user_name__regex** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::PaginatedObjectChangeList**](PaginatedObjectChangeList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_object_changes_retrieve

> crate::models::ObjectChange core_object_changes_retrieve(id)


Retrieve a list of recent changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this object change. | [required] |

### Return type

[**crate::models::ObjectChange**](ObjectChange.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_object_types_list

> crate::models::PaginatedObjectTypeList core_object_types_list(app_label, app_label__empty, app_label__ic, app_label__ie, app_label__iew, app_label__iregex, app_label__isw, app_label__n, app_label__nic, app_label__nie, app_label__niew, app_label__nisw, app_label__regex, features, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, limit, model, model__empty, model__ic, model__ie, model__iew, model__iregex, model__isw, model__n, model__nic, model__nie, model__niew, model__nisw, model__regex, offset, ordering, public, q)


Read-only list of ObjectTypes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_label** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__empty** | Option<**bool**> |  |  |
**app_label__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__n** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**app_label__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**features** | Option<**String**> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**model** | Option<[**Vec<String>**](String.md)> |  |  |
**model__empty** | Option<**bool**> |  |  |
**model__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**model__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**model__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**model__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**model__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**model__n** | Option<[**Vec<String>**](String.md)> |  |  |
**model__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**model__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**model__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**model__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**model__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**public** | Option<**bool**> |  |  |
**q** | Option<**String**> | Search |  |

### Return type

[**crate::models::PaginatedObjectTypeList**](PaginatedObjectTypeList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_object_types_retrieve

> crate::models::ObjectType core_object_types_retrieve(id)


Read-only list of ObjectTypes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this object type. | [required] |

### Return type

[**crate::models::ObjectType**](ObjectType.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

