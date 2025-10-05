# Agent Guide: Porting Temporal Go to Temporal Rust SDK

This guide documents key learnings and patterns for future AI agents working on porting Temporal Go workflows to the unofficial Rust SDK.

## Table of Contents

1. [SDK Architecture Overview](#sdk-architecture-overview)
2. [Key Differences: Go vs Rust SDK](#key-differences-go-vs-rust-sdk)
3. [Workflow Patterns](#workflow-patterns)
4. [Activity Patterns](#activity-patterns)
5. [Worker Registration](#worker-registration)
6. [Testing Patterns](#testing-patterns)
7. [Common Gotchas](#common-gotchas)
8. [Best Practices](#best-practices)

---

## SDK Architecture Overview

### SDK Location
The Rust SDK is embedded in this repository at `sdk-core/`:
```
sdk-core/
├── sdk/          # High-level Rust SDK
├── core/         # Core runtime
├── core-api/     # API definitions
├── client/       # Client implementation
└── sdk-core-protos/  # Protocol buffers
```

### Dependencies in Cargo.toml
```toml
[dependencies]
temporal-sdk = { path = "../sdk-core/sdk" }
temporal-sdk-core = { path = "../sdk-core/core" }
temporal-sdk-core-api = { path = "../sdk-core/core-api" }
temporal-client = { path = "../sdk-core/client" }
temporal-sdk-core-protos = { path = "../sdk-core/sdk-core-protos" }
```

---

## Key Differences: Go vs Rust SDK

### 1. Activity Error Handling

**Go:**
```go
func MyActivity(ctx context.Context) error {
    return fmt.Errorf("some error")
}
```

**Rust:**
```rust
use temporal_sdk::{ActContext, ActivityError};

async fn my_activity(_ctx: ActContext) -> Result<String, ActivityError> {
    // Retryable error
    Err(ActivityError::Retryable {
        source: anyhow::anyhow!("Temporary failure"),
        explicit_delay: None,
    })

    // NonRetryable error (permanent failure)
    Err(ActivityError::NonRetryable(
        anyhow::anyhow!("Permanent failure")
    ))
}
```

**Key Points:**
- `ActivityError::Retryable` is a struct variant with `source` and `explicit_delay`
- `ActivityError::NonRetryable` is a tuple variant with just the error
- Use `Retryable` for transient failures (network timeouts, rate limits)
- Use `NonRetryable` for permanent failures (validation errors, missing data)

### 2. Workflow Return Types

**Go:**
```go
func MyWorkflow(ctx workflow.Context) (string, error) {
    return "result", nil
}
```

**Rust:**
```rust
use temporal_sdk::{WfContext, WfExitValue};

async fn my_workflow(ctx: WfContext) -> Result<WfExitValue<String>, anyhow::Error> {
    Ok(WfExitValue::Normal("result".to_string()))
}
```

**Key Points:**
- Workflows return `Result<WfExitValue<T>, anyhow::Error>`
- `WfExitValue::Normal(value)` for successful completion
- Must wrap result in `WfExitValue`

### 3. Activity Options

**Go:**
```go
ao := workflow.ActivityOptions{
    StartToCloseTimeout: time.Minute,
}
ctx = workflow.WithActivityOptions(ctx, ao)

err := workflow.ExecuteActivity(ctx, MyActivity).Get(ctx, &result)
```

**Rust:**
```rust
use temporal_sdk::ActivityOptions;
use std::time::Duration;

let result = ctx
    .activity(ActivityOptions {
        activity_type: "my_activity".to_string(),
        input: serde_json::to_vec(&input)?.into(),
        start_to_close_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    })
    .await
    .unwrap_ok_payload();

let value: MyType = serde_json::from_slice(&result.data)?;
```

**Key Points:**
- Must manually serialize input: `serde_json::to_vec(&input)?.into()`
- Must manually deserialize output: `serde_json::from_slice(&result.data)?`
- Use `.unwrap_ok_payload()` to extract successful result
- Activity type is a string, not a function reference

### 4. Timers/Sleep

**Go:**
```go
workflow.Sleep(ctx, time.Second * 5)
```

**Rust:**
```rust
use temporal_sdk::TimerOptions;
use tokio::time::Duration;

ctx.timer(TimerOptions {
    duration: Duration::from_secs(5),
    summary: None,
}).await;
```

---

## Workflow Patterns

### Basic Workflow Structure

```rust
use temporal_sdk::{WfContext, WfExitValue, ActivityOptions};
use tokio::time::Duration;

async fn my_workflow(
    ctx: WfContext,
    input: MyInput
) -> Result<WfExitValue<MyOutput>, anyhow::Error> {
    // Execute activity
    let result = ctx
        .activity(ActivityOptions {
            activity_type: "my_activity".to_string(),
            input: serde_json::to_vec(&input)?.into(),
            start_to_close_timeout: Some(Duration::from_secs(30)),
            ..Default::default()
        })
        .await
        .unwrap_ok_payload();

    let data: ActivityResult = serde_json::from_slice(&result.data)?;

    // Timer
    ctx.timer(TimerOptions {
        duration: Duration::from_secs(5),
        summary: None,
    }).await;

    // Return
    Ok(WfExitValue::Normal(MyOutput { /* ... */ }))
}
```

### Getting Workflow Input

Workflows receive input as a parameter after deserialization in the worker:

```rust
// In worker registration:
worker.register_wf("my_workflow", |ctx: WfContext| async move {
    let input: MyInput = serde_json::from_slice(
        &ctx.get_args().first().expect("Missing input").data
    ).expect("Failed to deserialize");

    my_workflow(ctx, input).await
});
```

---

## Activity Patterns

### Basic Activity Structure

```rust
use temporal_sdk::{ActContext, ActivityError};

async fn my_activity(
    _ctx: ActContext,
    param: String,
) -> Result<MyResult, ActivityError> {
    // Perform work
    let result = do_work(&param)
        .await
        .map_err(|e| ActivityError::Retryable {
            source: anyhow::anyhow!("Failed: {}", e),
            explicit_delay: None,
        })?;

    Ok(result)
}
```

### HTTP Requests in Activities

```rust
async fn fetch_data(_ctx: ActContext, url: String) -> Result<String, ActivityError> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| ActivityError::Retryable {
            source: anyhow::anyhow!("HTTP error: {}", e),
            explicit_delay: None,
        })?;

    let text = response
        .text()
        .await
        .map_err(|e| ActivityError::NonRetryable(
            anyhow::anyhow!("Failed to read response: {}", e)
        ))?;

    Ok(text)
}
```

---

## Worker Registration

### Complete Worker Setup

```rust
use temporal_sdk::{sdk_client_options, Worker};
use temporal_sdk_core::{init_worker, Url, CoreRuntime};
use temporal_sdk_core_api::{
    worker::{WorkerConfigBuilder, WorkerVersioningStrategy},
    telemetry::TelemetryOptionsBuilder
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let temporal_address = std::env::var("TEMPORAL_ADDRESS")
        .unwrap_or_else(|_| "http://localhost:7233".to_string());

    // Create client
    let server_options = sdk_client_options(Url::from_str(&temporal_address)?).build()?;
    let client = server_options.connect("default", None).await?;

    // Create runtime and worker config
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("my-task-queue")
        .versioning_strategy(WorkerVersioningStrategy::None {
            build_id: "rust-sdk".to_owned()
        })
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), "my-task-queue");

    // Register workflows with input deserialization
    worker.register_wf("my_workflow", |ctx: WfContext| async move {
        let input: MyInput = serde_json::from_slice(
            &ctx.get_args().first().expect("Missing input").data
        ).expect("Failed to deserialize");

        my_workflow(ctx, input).await
    });

    // Register activities with parameter mapping
    worker.register_activity("my_activity", |ctx, param: String| {
        my_activity(ctx, param)
    });

    worker.run().await?;
    Ok(())
}
```

### Activity Registration Patterns

```rust
// Activity with no parameters
worker.register_activity("no_params", |ctx, _: ()| my_activity(ctx));

// Activity with single parameter
worker.register_activity("single_param", |ctx, param: String| {
    my_activity(ctx, param)
});

// Activity with multiple parameters (use tuple)
worker.register_activity("multi_params", |ctx, (a, b): (String, i32)| {
    my_activity(ctx, a, b)
});
```

---

## Testing Patterns

### 1. Basic Unit Tests (No Temporal Server)

```rust
#[tokio::test]
async fn test_type_serialization() {
    let input = MyInput { field: "value".to_string() };
    let serialized = serde_json::to_string(&input).unwrap();
    let deserialized: MyInput = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.field, input.field);
}
```

### 2. HTTP Logic Tests (Direct API Calls)

```rust
#[tokio::test]
async fn test_api_call() {
    let result = reqwest::get("https://api.example.com/data").await;
    match result {
        Ok(response) => {
            let data = response.json::<MyData>().await.unwrap();
            assert!(!data.field.is_empty());
        }
        Err(_) => {
            // Network errors acceptable in tests
        }
    }
}
```

### 3. Golden Snapshot Testing

Create `tests/fixtures/golden_data.json`:
```json
{
  "test_case_1": {
    "input": "value",
    "expected_output": "result"
  }
}
```

```rust
fn load_golden_data() -> HashMap<String, TestCase> {
    let json = include_str!("fixtures/golden_data.json");
    serde_json::from_str(json).unwrap()
}

#[tokio::test]
async fn test_with_golden_data() {
    let golden = load_golden_data();
    let test_case = &golden["test_case_1"];
    // Use golden data in test
}
```

### 4. Mock Activity Testing

```rust
#[tokio::test]
async fn test_retry_logic() {
    let attempt_counter = Arc::new(AtomicU32::new(0));

    // Simulate 3 attempts
    for _ in 0..3 {
        let attempt = attempt_counter.fetch_add(1, Ordering::SeqCst);
        if attempt < 2 {
            // Fail first 2 attempts
            assert!(attempt < 2);
        } else {
            // Succeed on 3rd
            assert!(true);
        }
    }
}
```

### 5. Error Type Testing

```rust
#[tokio::test]
async fn test_error_types() {
    // NonRetryable
    let error: Result<String, ActivityError> = Err(ActivityError::NonRetryable(
        anyhow::anyhow!("Permanent failure")
    ));
    assert!(matches!(error, Err(ActivityError::NonRetryable(_))));

    // Retryable
    let error: Result<String, ActivityError> = Err(ActivityError::Retryable {
        source: anyhow::anyhow!("Temporary failure"),
        explicit_delay: None,
    });
    assert!(matches!(error, Err(ActivityError::Retryable { .. })));
}
```

---

## Common Gotchas

### 1. ActContext Cannot Be Default-Constructed
❌ **Wrong:**
```rust
let ctx = ActContext::default(); // ERROR: no Default impl
```

✅ **Right:**
```rust
// Test activity logic without ActContext
async fn my_logic(input: String) -> Result<String> { /* ... */ }

#[tokio::test]
async fn test() {
    let result = my_logic("test".to_string()).await;
    assert!(result.is_ok());
}
```

### 2. Activity Type Names Must Be Strings
❌ **Wrong:**
```rust
ctx.activity(ActivityOptions {
    activity_type: my_activity, // ERROR: expected String
    // ...
})
```

✅ **Right:**
```rust
ctx.activity(ActivityOptions {
    activity_type: "my_activity".to_string(),
    // ...
})
```

### 3. Input Serialization is Manual
❌ **Wrong:**
```rust
ctx.activity(ActivityOptions {
    input: my_input, // ERROR: expected Payload
    // ...
})
```

✅ **Right:**
```rust
ctx.activity(ActivityOptions {
    input: serde_json::to_vec(&my_input)?.into(),
    // ...
})
```

### 4. Workflow Input Extraction
❌ **Wrong:**
```rust
worker.register_wf("wf", my_workflow); // ERROR: wrong signature
```

✅ **Right:**
```rust
worker.register_wf("wf", |ctx: WfContext| async move {
    let input: MyInput = serde_json::from_slice(
        &ctx.get_args().first().expect("Missing input").data
    ).expect("Failed to deserialize");
    my_workflow(ctx, input).await
});
```

### 5. ActivityError Variant Types
❌ **Wrong:**
```rust
ActivityError::Retryable(anyhow::anyhow!("error")) // ERROR: wrong variant type
ActivityError::NonRetryable { source: err } // ERROR: wrong variant type
```

✅ **Right:**
```rust
// Retryable is a struct variant
ActivityError::Retryable {
    source: anyhow::anyhow!("error"),
    explicit_delay: None,
}

// NonRetryable is a tuple variant
ActivityError::NonRetryable(anyhow::anyhow!("error"))
```

---

## Best Practices

### 1. Project Structure
```
my-demo-rust/
├── src/
│   ├── lib.rs              # Re-export modules
│   ├── types.rs            # Shared types and constants
│   ├── activities.rs       # Activity implementations
│   ├── workflows.rs        # Workflow implementations
│   └── bin/
│       ├── worker.rs       # Worker binary
│       └── starter.rs      # Workflow starter
├── tests/
│   ├── integration_tests.rs
│   ├── advanced_tests.rs
│   └── fixtures/
│       └── golden_data.json
└── Cargo.toml
```

### 2. Type Definitions
```rust
// types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInput {
    pub field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOutput {
    pub result: String,
}

pub const TASK_QUEUE_NAME: &str = "my-task-queue";
```

### 3. Error Handling Strategy
- Use `Retryable` for transient failures that might succeed on retry
- Use `NonRetryable` for permanent failures (validation errors, bad input)
- Wrap errors with context: `anyhow::anyhow!("Context: {}", e)`
- Consider `explicit_delay` for rate limiting scenarios

### 4. Testing Strategy
1. **Unit tests**: Type serialization, business logic
2. **Integration tests**: Real HTTP calls (with network error tolerance)
3. **Mock tests**: Simulate failures, timeouts, retries
4. **Golden tests**: Use fixtures for deterministic testing
5. **Activity options tests**: Validate timeout configurations

### 5. Logging
```rust
use tracing::{info, error};

// In activities
info!("Processing request for {}", id);

// In workflows
info!("Starting workflow for user: {}", input.name);

// In worker
tracing_subscriber::fmt::init();
```

### 6. Async Runtime
- Always use `tokio::time::Duration` for Temporal durations
- Use `#[tokio::main]` for main functions
- Use `#[tokio::test]` for async tests

---

## Quick Reference Checklist

When porting a Go workflow to Rust:

- [ ] Create project structure with `src/` and `tests/`
- [ ] Add SDK dependencies to `Cargo.toml` (path-based, from `../sdk-core`)
- [ ] Define types in `types.rs` with Serialize/Deserialize
- [ ] Implement activities returning `Result<T, ActivityError>`
- [ ] Use `Retryable` for transient errors, `NonRetryable` for permanent
- [ ] Implement workflow returning `Result<WfExitValue<T>, anyhow::Error>`
- [ ] Manually serialize activity inputs with `serde_json::to_vec()`
- [ ] Manually deserialize activity outputs with `serde_json::from_slice()`
- [ ] Register workflows with input deserialization closure
- [ ] Register activities with parameter mapping closure
- [ ] Write unit tests (serialization, logic)
- [ ] Write integration tests (real HTTP calls)
- [ ] Write mock tests (failure scenarios)
- [ ] Add golden snapshot data if needed
- [ ] Initialize tracing in worker and tests

---

## Example: Complete Minimal Workflow

**types.rs:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input { pub name: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output { pub greeting: String }

pub const TASK_QUEUE: &str = "greeting-queue";
```

**activities.rs:**
```rust
use temporal_sdk::{ActContext, ActivityError};

pub async fn greet(_ctx: ActContext, name: String) -> Result<String, ActivityError> {
    Ok(format!("Hello, {}!", name))
}
```

**workflows.rs:**
```rust
use crate::types::{Input, Output};
use temporal_sdk::{WfContext, WfExitValue, ActivityOptions};
use tokio::time::Duration;

pub async fn greeting_workflow(
    ctx: WfContext,
    input: Input
) -> Result<WfExitValue<Output>, anyhow::Error> {
    let result = ctx
        .activity(ActivityOptions {
            activity_type: "greet".to_string(),
            input: serde_json::to_vec(&input.name)?.into(),
            start_to_close_timeout: Some(Duration::from_secs(30)),
            ..Default::default()
        })
        .await
        .unwrap_ok_payload();

    let greeting: String = serde_json::from_slice(&result.data)?;
    Ok(WfExitValue::Normal(Output { greeting }))
}
```

**worker.rs:**
```rust
use my_demo::{activities::greet, workflows::greeting_workflow, types::*};
use temporal_sdk::{sdk_client_options, Worker, WfContext};
use temporal_sdk_core::{init_worker, Url, CoreRuntime};
use temporal_sdk_core_api::{
    worker::{WorkerConfigBuilder, WorkerVersioningStrategy},
    telemetry::TelemetryOptionsBuilder
};
use std::{env, str::FromStr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = env::var("TEMPORAL_ADDRESS")
        .unwrap_or_else(|_| "http://localhost:7233".to_string());

    let client = sdk_client_options(Url::from_str(&addr)?)
        .build()?
        .connect("default", None)
        .await?;

    let runtime = CoreRuntime::new_assume_tokio(
        TelemetryOptionsBuilder::default().build()?
    )?;

    let config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue(TASK_QUEUE)
        .versioning_strategy(WorkerVersioningStrategy::None {
            build_id: "rust".to_owned()
        })
        .build()?;

    let core = init_worker(&runtime, config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core), TASK_QUEUE);

    worker.register_wf("greeting_workflow", |ctx: WfContext| async move {
        let input: Input = serde_json::from_slice(
            &ctx.get_args().first().expect("input").data
        ).expect("deser");
        greeting_workflow(ctx, input).await
    });

    worker.register_activity("greet", |ctx, name: String| greet(ctx, name));

    worker.run().await?;
    Ok(())
}
```

---

## Summary

The unofficial Temporal Rust SDK requires more manual work than the Go SDK (explicit serialization, activity registration with closures), but provides type safety and excellent testing capabilities. Focus on:

1. Understanding the error types (Retryable vs NonRetryable variants)
2. Manual serialization/deserialization for inputs/outputs
3. Worker registration patterns with closures
4. Comprehensive testing with mocks and golden data

Good luck with your porting! 🦀
