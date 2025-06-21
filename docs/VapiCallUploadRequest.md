# VapiCallUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vapi_end_of_call_report_payload** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Raw VAPI end-of-call-report webhook payload forwarded directly from VAPI | 
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Optional custom properties to include with the call for filtering and display | [optional]
**skip_already_imported** | Option<**bool**> | Skip import if a call with the same VAPI call ID already exists | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


