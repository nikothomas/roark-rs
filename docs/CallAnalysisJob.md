# CallAnalysisJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for tracking the analysis job progress | 
**status** | [**models::JobStatus**](JobStatus.md) |  | 
**call** | [**models::CallData**](CallData.md) |  | 
**created_at** | Option<**String**> | When the analysis job was created | [optional]
**completed_at** | Option<**String**> | When the analysis job completed (null if still processing) | [optional]
**error_message** | Option<**String**> | Error message if the job failed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


