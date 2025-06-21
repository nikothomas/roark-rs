# SentimentAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **String** | Status of the sentiment analysis job | 
**average_sentiment** | Option<**f64**> | Average sentiment score across the entire call (0-1, where 1 is most positive) | [optional]
**average_categorical_sentiment** | Option<[**models::SentimentCategory**](SentimentCategory.md)> |  | [optional]
**common_emotion** | Option<**String**> | Most frequently detected emotion throughout the call | [optional]
**emotion_breakdown** | Option<**std::collections::HashMap<String, f64>**> | Detailed breakdown of emotions detected with their frequencies | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


