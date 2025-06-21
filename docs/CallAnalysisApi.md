# \CallAnalysisApi

All URIs are relative to *https://api.roark.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_call_analysis_job**](CallAnalysisApi.md#create_call_analysis_job) | **POST** /v1/call-analysis | Create call analysis job
[**get_call_analysis_job**](CallAnalysisApi.md#get_call_analysis_job) | **GET** /v1/call-analysis/{job_id} | Get call analysis job



## create_call_analysis_job

> models::CreateCallAnalysisJob201Response create_call_analysis_job(call_analysis_create_request)
Create call analysis job

Upload a call recording and create an analysis job to extract insights, transcripts, and metadata from the audio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_analysis_create_request** | [**CallAnalysisCreateRequest**](CallAnalysisCreateRequest.md) |  | [required] |

### Return type

[**models::CreateCallAnalysisJob201Response**](create_call_analysis_job_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call_analysis_job

> models::CreateCallAnalysisJob201Response get_call_analysis_job(job_id)
Get call analysis job

Retrieve the status and results of a call analysis job by its unique identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Unique identifier of the analysis job | [required] |

### Return type

[**models::CreateCallAnalysisJob201Response**](create_call_analysis_job_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

