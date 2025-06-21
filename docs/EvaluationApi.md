# \EvaluationApi

All URIs are relative to *https://api.roark.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_evaluation_job**](EvaluationApi.md#create_evaluation_job) | **POST** /v1/evaluation/job | Create evaluation job
[**get_evaluation_job**](EvaluationApi.md#get_evaluation_job) | **GET** /v1/evaluation/job/{job_id} | Get evaluation job
[**get_evaluation_job_runs**](EvaluationApi.md#get_evaluation_job_runs) | **GET** /v1/evaluation/job/{job_id}/runs | Get evaluation job runs



## create_evaluation_job

> models::CreateEvaluationJob201Response create_evaluation_job(evaluation_job_create_request)
Create evaluation job

Create an evaluation job to assess a single call or dataset of calls using specified evaluators. This allows for comprehensive analysis of agent performance and call quality.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evaluation_job_create_request** | [**EvaluationJobCreateRequest**](EvaluationJobCreateRequest.md) |  | [required] |

### Return type

[**models::CreateEvaluationJob201Response**](create_evaluation_job_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_evaluation_job

> models::GetEvaluationJob200Response get_evaluation_job(job_id)
Get evaluation job

Retrieve details of a specific evaluation job, including its current status and configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Unique identifier of the evaluation job | [required] |

### Return type

[**models::GetEvaluationJob200Response**](get_evaluation_job_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_evaluation_job_runs

> models::GetEvaluationJobRuns200Response get_evaluation_job_runs(job_id, next_cursor, limit)
Get evaluation job runs

Retrieve paginated details of evaluation runs for a specific job, including scores, metrics, and evidence for each evaluator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | Unique identifier of the evaluation job | [required] |
**next_cursor** | Option<**String**> | Cursor for the next page of results |  |
**limit** | Option<**u8**> | Number of items to return per page (1-50) |  |[default to 10]

### Return type

[**models::GetEvaluationJobRuns200Response**](get_evaluation_job_runs_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

