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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetEvaluationJob200ResponseAllOfData {
    /// Unique identifier of the evaluation job
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "status")]
    pub status: models::EvaluationJobStatus,
    /// When the evaluation job was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the evaluation job completed (null if still processing)
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl GetEvaluationJob200ResponseAllOfData {
    pub fn new(id: uuid::Uuid, status: models::EvaluationJobStatus) -> GetEvaluationJob200ResponseAllOfData {
        GetEvaluationJob200ResponseAllOfData {
            id,
            status,
            created_at: None,
            completed_at: None,
        }
    }
}

