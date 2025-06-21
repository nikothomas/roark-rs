# RetellCallUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retell_call_ended_payload** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Raw Retell call_ended webhook payload forwarded directly from Retell | 
**properties** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Optional custom properties to include with the call for filtering and display | [optional]
**skip_already_imported** | Option<**bool**> | Skip import if a call with the same Retell call ID already exists | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


