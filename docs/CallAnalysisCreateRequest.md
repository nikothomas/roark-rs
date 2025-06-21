# CallAnalysisCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recording_url** | **String** | Publicly accessible URL of the call recording file (WAV or MP3 format). Can be a signed URL with appropriate expiration. | 
**stereo_recording_url** | Option<**String**> | Optional URL of stereo recording in WAV format for enhanced audio analysis and playback experience | [optional]
**started_at** | **String** | ISO 8601 timestamp when the call started | 
**interface_type** | [**models::InterfaceType**](InterfaceType.md) |  | 
**call_direction** | [**models::CallDirection**](CallDirection.md) |  | 
**participants** | [**Vec<models::CallParticipant>**](CallParticipant.md) | Exactly two participants: one agent and one customer | 
**ended_status** | Option<[**models::CallEndedStatus**](CallEndedStatus.md)> |  | [optional]
**ended_reason** | Option<**String**> | Additional context about why the call ended | [optional]
**tool_invocations** | Option<[**Vec<models::ToolInvocation>**](ToolInvocation.md)> | List of tools/functions invoked during the call | [optional]
**is_test** | Option<**bool**> | Mark this as a test call for development/QA purposes | [optional][default to false]
**vapi_call_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Original VAPI call ID if importing from VAPI platform | [optional]
**retell_call_id** | Option<**String**> | Original Retell call ID if importing from Retell platform | [optional]
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Custom metadata properties for filtering and categorization | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


