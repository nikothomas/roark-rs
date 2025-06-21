# \CallOperationsApi

All URIs are relative to *https://api.roark.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_call_evaluation_runs**](CallOperationsApi.md#get_call_evaluation_runs) | **GET** /v1/call/{call_id}/evaluation-run | Get call evaluation runs
[**get_call_sentiment_analysis**](CallOperationsApi.md#get_call_sentiment_analysis) | **GET** /v1/call/{call_id}/sentiment-run | Get call sentiment analysis



## get_call_evaluation_runs

> models::GetCallEvaluationRuns200Response get_call_evaluation_runs(call_id)
Get call evaluation runs

Fetch evaluation run results for a specific call, including scores, metrics, and supporting evidence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_id** | **uuid::Uuid** | Unique identifier of the call to fetch evaluation runs for | [required] |

### Return type

[**models::GetCallEvaluationRuns200Response**](get_call_evaluation_runs_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call_sentiment_analysis

> models::GetCallSentimentAnalysis200Response get_call_sentiment_analysis(call_id)
Get call sentiment analysis

Fetch detailed sentiment analysis results for a specific call, including emotional tone, key phrases, and sentiment scores across 64+ emotions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_id** | **uuid::Uuid** | Unique identifier of the call to fetch sentiment analysis for | [required] |

### Return type

[**models::GetCallSentimentAnalysis200Response**](get_call_sentiment_analysis_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

