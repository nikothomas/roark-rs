# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an auto-generated Rust API client library for the Roark Analytics API - a Voice AI Analytics Platform. The entire codebase is generated from the OpenAPI specification file `roark-analytics-api-openapi.json` using OpenAPI Generator v7.13.0.

## Common Development Commands

### Build and Test
```bash
# Build the project
cargo build

# Run tests
cargo test

# Check code without building
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt

# Generate and open documentation
cargo doc --open

# Run a specific test
cargo test test_name
```

### Regenerating the Client
If you need to regenerate the client from the OpenAPI spec:
```bash
openapi-generator-cli generate -i roark-analytics-api-openapi.json -g rust -c config.yaml
```

## Architecture and Code Structure

### Module Organization
- **`src/apis/`** - API client implementations (one module per API endpoint group)
  - `configuration.rs` - Client configuration and authentication setup
  - `*_api.rs` - Individual API endpoint implementations
- **`src/models/`** - Data structures matching the API schemas
  - Each model corresponds to a schema in the OpenAPI spec
  - All models use `serde` for JSON serialization/deserialization
- **`docs/`** - Auto-generated API documentation

### Key Architectural Patterns

1. **Authentication**: All API calls require Bearer token authentication configured via the `Configuration` struct
2. **Async/Await**: All API methods are async and use `reqwest` for HTTP requests
3. **Error Handling**: Each API module has its own error enum with typed error variants
4. **Serialization**: Heavy use of `serde` with derive macros for automatic JSON handling
5. **Auto-generated Code**: Do not manually modify the generated code - changes should be made to the OpenAPI spec and regenerated

### API Endpoints
- **CallAnalysisApi**: Create and retrieve voice call analysis jobs
- **CallOperationsApi**: Get evaluation runs and sentiment analysis results
- **EvaluationApi**: Create and manage evaluation jobs
- **HealthApi**: Check API health status
- **IntegrationsApi**: Platform integrations (VAPI and Retell AI)

## Important Notes

- The entire `src/` directory is auto-generated - do not manually edit these files
- To make changes, modify the `roark-analytics-api-openapi.json` spec and regenerate
- The project uses Rust edition 2024 (likely a typo in Cargo.toml, should be 2021)
- All API methods return `Result<T, Error<E>>` where E is the endpoint-specific error type
- Rate limiting is implemented on the API side - handle 429 responses appropriately