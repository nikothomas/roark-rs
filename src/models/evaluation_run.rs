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

/// EvaluationRun : Complete evaluation run result for a single evaluator
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluationRun {
    /// Unique identifier for the evaluation run
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "status")]
    pub status: models::JobStatus,
    /// Overall score from this evaluation run (0-1)
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "score_classification", skip_serializing_if = "Option::is_none")]
    pub score_classification: Option<models::ScoreClassification>,
    /// AI-generated summary of the evaluation results
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// When the evaluation run started
    #[serde(rename = "started_at", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// When the evaluation run completed
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Individual metrics evaluated in this run
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<models::EvaluationMetric>>,
    /// Supporting evidence for the evaluation
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<models::EvaluationEvidence>>,
    #[serde(rename = "evaluator")]
    pub evaluator: models::Evaluator,
}

impl EvaluationRun {
    /// Complete evaluation run result for a single evaluator
    pub fn new(id: uuid::Uuid, status: models::JobStatus, evaluator: models::Evaluator) -> EvaluationRun {
        EvaluationRun {
            id,
            status,
            score: None,
            score_classification: None,
            summary: None,
            started_at: None,
            completed_at: None,
            metrics: None,
            evidence: None,
            evaluator,
        }
    }
}

