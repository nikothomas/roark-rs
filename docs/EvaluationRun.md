# EvaluationRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for the evaluation run | 
**status** | [**models::JobStatus**](JobStatus.md) |  | 
**score** | Option<**f64**> | Overall score from this evaluation run (0-1) | [optional]
**score_classification** | Option<[**models::ScoreClassification**](ScoreClassification.md)> |  | [optional]
**summary** | Option<**String**> | AI-generated summary of the evaluation results | [optional]
**started_at** | Option<**String**> | When the evaluation run started | [optional]
**completed_at** | Option<**String**> | When the evaluation run completed | [optional]
**metrics** | Option<[**Vec<models::EvaluationMetric>**](EvaluationMetric.md)> | Individual metrics evaluated in this run | [optional]
**evidence** | Option<[**Vec<models::EvaluationEvidence>**](EvaluationEvidence.md)> | Supporting evidence for the evaluation | [optional]
**evaluator** | [**models::Evaluator**](Evaluator.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


