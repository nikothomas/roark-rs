# ErrorResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_type** | [**models::ErrorType**](ErrorType.md) |  | 
**error_code** | **String** | Machine-readable error code identifier for programmatic handling | 
**message** | **String** | Human-readable error message providing context about the error | 
**parameter** | Option<**String**> | The specific parameter that caused the error (if applicable) | [optional]
**details** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Additional contextual information about the error | [optional]
**request_id** | Option<**String**> | Unique request identifier for debugging and support purposes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


