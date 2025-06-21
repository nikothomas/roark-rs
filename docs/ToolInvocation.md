# ToolInvocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name identifier of the invoked tool or function | 
**description** | Option<**String**> | Human-readable description of the tool's purpose | [optional]
**parameters** | [**std::collections::HashMap<String, models::ToolInvocationParametersValue>**](ToolInvocation_parameters_value.md) | Parameters provided to the tool during invocation | 
**result** | [**models::ToolInvocationResult**](ToolInvocation_result.md) |  | 
**start_offset_ms** | **u32** | Offset in milliseconds from call start when tool was invoked | 
**end_offset_ms** | Option<**u32**> | Offset in milliseconds from call start when tool execution completed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


