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

/// HealthResponse : Health check response providing service status and version information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthResponse {
    #[serde(rename = "status")]
    pub status: models::HealthStatus,
    /// Current API version identifier
    #[serde(rename = "version")]
    pub version: String,
    /// ISO 8601 timestamp when the health check was performed
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    /// Service uptime in seconds
    #[serde(rename = "uptime_seconds", skip_serializing_if = "Option::is_none")]
    pub uptime_seconds: Option<u32>,
    /// Health status of external dependencies
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<std::collections::HashMap<String, models::HealthStatus>>,
}

impl HealthResponse {
    /// Health check response providing service status and version information
    pub fn new(status: models::HealthStatus, version: String, timestamp: String) -> HealthResponse {
        HealthResponse {
            status,
            version,
            timestamp,
            uptime_seconds: None,
            dependencies: None,
        }
    }
}

