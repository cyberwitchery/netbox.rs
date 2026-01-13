# \PluginsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**plugins_branching_branch_events_list**](PluginsApi.md#plugins_branching_branch_events_list) | **GET** /api/plugins/branching/branch-events/ | 
[**plugins_branching_branch_events_retrieve**](PluginsApi.md#plugins_branching_branch_events_retrieve) | **GET** /api/plugins/branching/branch-events/{id}/ | 
[**plugins_branching_branches_create**](PluginsApi.md#plugins_branching_branches_create) | **POST** /api/plugins/branching/branches/ | 
[**plugins_branching_branches_destroy**](PluginsApi.md#plugins_branching_branches_destroy) | **DELETE** /api/plugins/branching/branches/{id}/ | 
[**plugins_branching_branches_list**](PluginsApi.md#plugins_branching_branches_list) | **GET** /api/plugins/branching/branches/ | 
[**plugins_branching_branches_merge_create**](PluginsApi.md#plugins_branching_branches_merge_create) | **POST** /api/plugins/branching/branches/{id}/merge/ | 
[**plugins_branching_branches_partial_update**](PluginsApi.md#plugins_branching_branches_partial_update) | **PATCH** /api/plugins/branching/branches/{id}/ | 
[**plugins_branching_branches_retrieve**](PluginsApi.md#plugins_branching_branches_retrieve) | **GET** /api/plugins/branching/branches/{id}/ | 
[**plugins_branching_branches_revert_create**](PluginsApi.md#plugins_branching_branches_revert_create) | **POST** /api/plugins/branching/branches/{id}/revert/ | 
[**plugins_branching_branches_sync_create**](PluginsApi.md#plugins_branching_branches_sync_create) | **POST** /api/plugins/branching/branches/{id}/sync/ | 
[**plugins_branching_branches_update**](PluginsApi.md#plugins_branching_branches_update) | **PUT** /api/plugins/branching/branches/{id}/ | 
[**plugins_branching_changes_list**](PluginsApi.md#plugins_branching_changes_list) | **GET** /api/plugins/branching/changes/ | 
[**plugins_branching_changes_retrieve**](PluginsApi.md#plugins_branching_changes_retrieve) | **GET** /api/plugins/branching/changes/{id}/ | 



## plugins_branching_branch_events_list

> crate::models::PaginatedBranchEventList plugins_branching_branch_events_list(id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, limit, offset, ordering, time, time__empty, time__gt, time__gte, time__lt, time__lte, time__n, r#type, type__empty, type__ic, type__ie, type__iew, type__iregex, type__isw, type__n, type__nic, type__nie, type__niew, type__nisw, type__regex)


Get a list of branch event objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**time** | Option<[**Vec<String>**](String.md)> |  |  |
**time__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**time__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**time__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**time__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**time__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**time__n** | Option<[**Vec<String>**](String.md)> |  |  |
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

### Return type

[**crate::models::PaginatedBranchEventList**](PaginatedBranchEventList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branch_events_retrieve

> crate::models::BranchEvent plugins_branching_branch_events_retrieve(id)


Get a branch event object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch event. | [required] |

### Return type

[**crate::models::BranchEvent**](BranchEvent.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_create

> crate::models::Branch plugins_branching_branches_create(writable_branch_request)


Post a list of branch objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**writable_branch_request** | [**WritableBranchRequest**](WritableBranchRequest.md) |  | [required] |

### Return type

[**crate::models::Branch**](Branch.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_destroy

> plugins_branching_branches_destroy(id)


Delete a branch object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch. | [required] |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_list

> crate::models::PaginatedBranchList plugins_branching_branches_list(created, created__empty, created__gt, created__gte, created__lt, created__lte, created__n, created_by_request, description, description__empty, description__ic, description__ie, description__iew, description__iregex, description__isw, description__n, description__nic, description__nie, description__niew, description__nisw, description__regex, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_sync, last_sync__empty, last_sync__gt, last_sync__gte, last_sync__lt, last_sync__lte, last_sync__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, modified_by_request, name, name__empty, name__ic, name__ie, name__iew, name__iregex, name__isw, name__n, name__nic, name__nie, name__niew, name__nisw, name__regex, offset, ordering, q, status, status__empty, status__ic, status__ie, status__iew, status__iregex, status__isw, status__n, status__nic, status__nie, status__niew, status__nisw, status__regex, tag, tag__n, tag_id, tag_id__n, updated_by_request)


Get a list of branch objects.

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
**id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__empty** | Option<**bool**> |  |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**last_sync** | Option<[**Vec<String>**](String.md)> |  |  |
**last_sync__empty** | Option<[**Vec<String>**](String.md)> |  |  |
**last_sync__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_sync__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_sync__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_sync__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_sync__n** | Option<[**Vec<String>**](String.md)> |  |  |
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
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**tag__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tag_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**tag_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**updated_by_request** | Option<**uuid::Uuid**> |  |  |

### Return type

[**crate::models::PaginatedBranchList**](PaginatedBranchList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_merge_create

> crate::models::Job plugins_branching_branches_merge_create(id, commit_request)


Enqueue a background job to merge a branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch. | [required] |
**commit_request** | Option<[**CommitRequest**](CommitRequest.md)> |  |  |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_partial_update

> crate::models::Branch plugins_branching_branches_partial_update(id, patched_writable_branch_request)


Patch a branch object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch. | [required] |
**patched_writable_branch_request** | Option<[**PatchedWritableBranchRequest**](PatchedWritableBranchRequest.md)> |  |  |

### Return type

[**crate::models::Branch**](Branch.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_retrieve

> crate::models::Branch plugins_branching_branches_retrieve(id)


Get a branch object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch. | [required] |

### Return type

[**crate::models::Branch**](Branch.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_revert_create

> crate::models::Job plugins_branching_branches_revert_create(id, commit_request)


Enqueue a background job to revert a merged branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch. | [required] |
**commit_request** | Option<[**CommitRequest**](CommitRequest.md)> |  |  |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_sync_create

> crate::models::Job plugins_branching_branches_sync_create(id, commit_request)


Enqueue a background job to synchronize a branch from main.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch. | [required] |
**commit_request** | Option<[**CommitRequest**](CommitRequest.md)> |  |  |

### Return type

[**crate::models::Job**](Job.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_branches_update

> crate::models::Branch plugins_branching_branches_update(id, writable_branch_request)


Put a branch object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this branch. | [required] |
**writable_branch_request** | [**WritableBranchRequest**](WritableBranchRequest.md) |  | [required] |

### Return type

[**crate::models::Branch**](Branch.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_changes_list

> crate::models::PaginatedChangeDiffList plugins_branching_changes_list(action, action__empty, action__ic, action__ie, action__iew, action__iregex, action__isw, action__n, action__nic, action__nie, action__niew, action__nisw, action__regex, branch, branch__n, branch_id, branch_id__n, has_conflicts, id, id__empty, id__gt, id__gte, id__lt, id__lte, id__n, last_updated, last_updated__empty, last_updated__gt, last_updated__gte, last_updated__lt, last_updated__lte, last_updated__n, limit, object_id, object_id__empty, object_id__gt, object_id__gte, object_id__lt, object_id__lte, object_id__n, object_type, object_type__n, object_type_id, object_type_id__n, offset, ordering, q)


Get a list of change diff objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | Option<[**Vec<String>**](String.md)> |  |  |
**action__empty** | Option<**bool**> |  |  |
**action__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**action__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**action__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**action__iregex** | Option<[**Vec<String>**](String.md)> |  |  |
**action__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**action__n** | Option<[**Vec<String>**](String.md)> |  |  |
**action__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**action__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**action__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**action__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**action__regex** | Option<[**Vec<String>**](String.md)> |  |  |
**branch** | Option<[**Vec<String>**](String.md)> | Branch (schema ID) |  |
**branch__n** | Option<[**Vec<String>**](String.md)> | Branch (schema ID) |  |
**branch_id** | Option<[**Vec<i32>**](i32.md)> | Branch (ID) |  |
**branch_id__n** | Option<[**Vec<i32>**](i32.md)> | Branch (ID) |  |
**has_conflicts** | Option<**bool**> |  |  |
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
**object_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__empty** | Option<**bool**> |  |  |
**object_id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_type** | Option<**String**> |  |  |
**object_type__n** | Option<**String**> |  |  |
**object_type_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_type_id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**q** | Option<**String**> | Search |  |

### Return type

[**crate::models::PaginatedChangeDiffList**](PaginatedChangeDiffList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_branching_changes_retrieve

> crate::models::ChangeDiff plugins_branching_changes_retrieve(id)


Get a change diff object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this change diff. | [required] |

### Return type

[**crate::models::ChangeDiff**](ChangeDiff.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

