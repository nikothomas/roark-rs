# EvaluationMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier for the metric | 
**name** | **String** | Name of the evaluation metric | 
**value_type** | [**models::MetricValueType**](MetricValueType.md) |  | 
**numeric_value** | Option<**f64**> | Numeric value of the metric (for numeric metrics) | [optional]
**boolean_value** | Option<**bool**> | Boolean value of the metric (for boolean metrics) | [optional]
**text_value** | Option<**String**> | Text value of the metric (for text metrics) | [optional]
**confidence** | Option<**f64**> | Confidence score of the metric evaluation (0-1) | [optional]
**reasoning** | Option<**String**> | AI-generated explanation for the metric result | [optional]
**role** | [**models::MetricRole**](MetricRole.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


