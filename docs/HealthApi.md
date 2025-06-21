# \HealthApi

All URIs are relative to *https://api.roark.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_health**](HealthApi.md#get_api_health) | **GET** /health | Get API health status



## get_api_health

> models::GetApiHealth200Response get_api_health()
Get API health status

Returns the health status of the API and its dependencies. This endpoint can be used for monitoring and health checks.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetApiHealth200Response**](get_api_health_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

