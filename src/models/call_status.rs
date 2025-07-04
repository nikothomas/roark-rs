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

/// CallStatus : Current status of the call: - ringing: Call is being established but not yet connected - in_progress: Call is active and participants are connected - ended: Call has been terminated
/// Current status of the call: - ringing: Call is being established but not yet connected - in_progress: Call is active and participants are connected - ended: Call has been terminated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallStatus {
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "ended")]
    Ended,

}

impl std::fmt::Display for CallStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ringing => write!(f, "ringing"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Ended => write!(f, "ended"),
        }
    }
}

impl Default for CallStatus {
    fn default() -> CallStatus {
        Self::Ringing
    }
}

