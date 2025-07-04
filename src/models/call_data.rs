/*
 * Roark Analytics API
 *
 * # Roark Analytics API - Voice AI Analytics Platform  The Roark Analytics API provides comprehensive monitoring, evaluation, and analytics capabilities for voice AI agents. This API allows developers to seamlessly integrate with the Roark platform to track call quality, analyze agent performance, and extract insights from voice interactions.  ## Key Features  - **Real-time Call Analysis**: Upload and analyze voice call recordings with AI-powered insights - **Sentiment Analysis**: Extract emotional tone, key phrases, and sentiment scores across 64+ emotions - **Agent Performance Evaluation**: Create custom evaluation jobs with configurable metrics and scoring - **Platform Integrations**: Native support for VAPI and Retell AI with webhook-based data ingestion - **Custom Analytics**: Build custom analytics pipelines with flexible data models and properties  ## Authentication  All API endpoints require Bearer token authentication. Include your API token in the Authorization header:  ``` Authorization: Bearer YOUR_API_TOKEN ```  ## Rate Limiting  The API implements rate limiting to ensure service stability. Rate limit headers are included in responses.  ## Error Handling  The API uses standard HTTP status codes and returns structured error responses with detailed error information including error types, codes, and human-readable messages.  ## Rust Code Generation  This OpenAPI specification has been optimized for Rust code generation with: - Snake_case field naming conventions - Proper nullable field handling with Option<T> - Comprehensive documentation for generated code - Type-safe enum definitions - Structured error handling
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@roark.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CallData : Comprehensive call information including participants, timing, and metadata
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallData {
    /// Unique identifier for the call record
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "status")]
    pub status: models::CallStatus,
    /// AI-generated summary of the call content and outcomes
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "call_direction")]
    pub call_direction: models::CallDirection,
    /// ISO 8601 timestamp when the call began
    #[serde(rename = "started_at")]
    pub started_at: String,
    /// ISO 8601 timestamp when the call ended (null if call is ongoing)
    #[serde(rename = "ended_at", skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Total call duration in milliseconds (null if call is ongoing)
    #[serde(rename = "duration_ms", skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u32>,
    /// Detailed reason why the call ended
    #[serde(rename = "ended_reason", skip_serializing_if = "Option::is_none")]
    pub ended_reason: Option<String>,
    #[serde(rename = "ended_status", skip_serializing_if = "Option::is_none")]
    pub ended_status: Option<models::CallEndedStatus>,
    /// Whether this is a test call used for development or QA purposes
    #[serde(rename = "is_test")]
    pub is_test: bool,
    /// List of call participants (exactly 2: agent and customer)
    #[serde(rename = "participants")]
    pub participants: Vec<models::CallParticipant>,
    /// List of tools or functions invoked during the call
    #[serde(rename = "tool_invocations", skip_serializing_if = "Option::is_none")]
    pub tool_invocations: Option<Vec<models::ToolInvocation>>,
    /// Custom key-value properties for filtering and categorization
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl CallData {
    /// Comprehensive call information including participants, timing, and metadata
    pub fn new(id: uuid::Uuid, status: models::CallStatus, call_direction: models::CallDirection, started_at: String, is_test: bool, participants: Vec<models::CallParticipant>) -> CallData {
        CallData {
            id,
            status,
            summary: None,
            call_direction,
            started_at,
            ended_at: None,
            duration_ms: None,
            ended_reason: None,
            ended_status: None,
            is_test,
            participants,
            tool_invocations: None,
            properties: None,
        }
    }
}

