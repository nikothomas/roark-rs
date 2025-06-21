# CallData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for the call record | 
**status** | [**models::CallStatus**](CallStatus.md) |  | 
**summary** | Option<**String**> | AI-generated summary of the call content and outcomes | [optional]
**call_direction** | [**models::CallDirection**](CallDirection.md) |  | 
**started_at** | **String** | ISO 8601 timestamp when the call began | 
**ended_at** | Option<**String**> | ISO 8601 timestamp when the call ended (null if call is ongoing) | [optional]
**duration_ms** | Option<**u32**> | Total call duration in milliseconds (null if call is ongoing) | [optional]
**ended_reason** | Option<**String**> | Detailed reason why the call ended | [optional]
**ended_status** | Option<[**models::CallEndedStatus**](CallEndedStatus.md)> |  | [optional]
**is_test** | **bool** | Whether this is a test call used for development or QA purposes | [default to false]
**participants** | [**Vec<models::CallParticipant>**](CallParticipant.md) | List of call participants (exactly 2: agent and customer) | 
**tool_invocations** | Option<[**Vec<models::ToolInvocation>**](ToolInvocation.md)> | List of tools or functions invoked during the call | [optional]
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Custom key-value properties for filtering and categorization | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


