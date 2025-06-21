# HealthResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**models::HealthStatus**](HealthStatus.md) |  | 
**version** | **String** | Current API version identifier | 
**timestamp** | **String** | ISO 8601 timestamp when the health check was performed | 
**uptime_seconds** | Option<**u32**> | Service uptime in seconds | [optional]
**dependencies** | Option<[**std::collections::HashMap<String, models::HealthStatus>**](HealthStatus.md)> | Health status of external dependencies | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


