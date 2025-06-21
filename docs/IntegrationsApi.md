# \IntegrationsApi

All URIs are relative to *https://api.roark.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_retell_call**](IntegrationsApi.md#create_retell_call) | **POST** /v1/retell/call | Send a Retell call to Roark
[**create_vapi_call**](IntegrationsApi.md#create_vapi_call) | **POST** /v1/vapi/call | Send a VAPI call to Roark



## create_retell_call

> models::CreateVapiCall201Response create_retell_call(retell_call_upload_request)
Send a Retell call to Roark

Upload and process a Retell call recording by forwarding the call_ended webhook payload from Retell AI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retell_call_upload_request** | [**RetellCallUploadRequest**](RetellCallUploadRequest.md) |  | [required] |

### Return type

[**models::CreateVapiCall201Response**](create_vapi_call_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vapi_call

> models::CreateVapiCall201Response create_vapi_call(vapi_call_upload_request)
Send a VAPI call to Roark

Upload and process a VAPI call recording by forwarding the end-of-call-report webhook payload from VAPI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vapi_call_upload_request** | [**VapiCallUploadRequest**](VapiCallUploadRequest.md) |  | [required] |

### Return type

[**models::CreateVapiCall201Response**](create_vapi_call_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

