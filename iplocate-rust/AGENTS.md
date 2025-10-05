# Temporal Rust SDK - Agent Knowledge Base

## Critical Payload Serialization Rules

**ALWAYS use SDK helpers, NEVER use raw `serde_json`:**

| Purpose | ❌ Wrong | ✅ Correct |
|---------|---------|-----------|
| Serialize for Temporal | `serde_json::to_vec(&data)?.into()` | `data.as_json_payload()?` |
| Deserialize from Temporal | `serde_json::from_slice(&payload.data)?` | `T::from_json_payload(&payload)?` |

**Required imports:**
```rust
use temporal_sdk_core_protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};
```

## Common SDK Helpers Reference

### Workflow Context (`WfContext`)

| Helper | Purpose | Example |
|--------|---------|---------|
| `.activity()` | Execute activity | `ctx.activity(ActivityOptions { ... }).await` |
| `.local_activity()` | Execute local activity | `ctx.local_activity(ActivityOptions { ... }).await` |
| `.timer()` | Create timer | `ctx.timer(TimerOptions { duration: Duration::from_secs(5) }).await` |
| `.child_workflow()` | Start child workflow | `ctx.child_workflow(ChildWorkflowOptions { ... })` |
| `.signal_workflow()` | Signal external workflow | `ctx.signal_workflow(Signal { ... })` |
| `.cancelled()` | Check if cancelled | `ctx.cancelled().await` |
| `.is_replaying()` | Check if replaying | `ctx.is_replaying()` |
| `.workflow_time()` | Get workflow time | `ctx.workflow_time()` |
| `.random_seed()` | Get random seed | `ctx.random_seed()` |

### Activity Context (`ActContext`)

| Helper | Purpose | Example |
|--------|---------|---------|
| `.cancelled()` | Wait for cancellation | `ctx.cancelled().await` |
| `.is_cancelled()` | Check if cancelled | `ctx.is_cancelled()` |
| `.record_heartbeat()` | Send heartbeat | `ctx.record_heartbeat(vec![details])` |
| `.get_heartbeat_details()` | Get last heartbeat | `ctx.get_heartbeat_details()` |
| `.get_info()` | Get activity info | `ctx.get_info()` |
| `.headers()` | Get headers | `ctx.headers()` |
| `.app_data::<T>()` | Get shared app data | `ctx.app_data::<MyData>()` |

### Worker Registration

| Pattern | Signature |
|---------|-----------|
| Activity | `worker.register_activity("name", \|ctx: ActContext, input: T\| async move { ... })` |
| Workflow | `worker.register_wf("name", \|ctx: WfContext\| async move { ... })` |

## Key Learnings for Temporal Rust Beginners

### 1. Client Configuration
```rust
// ALWAYS set identity
let opts = sdk_client_options(url)
    .identity("my-worker-id".to_string())
    .build()?;
```

### 2. start_workflow Parameter Order
```rust
client.start_workflow(
    vec![input.as_json_payload()?],  // 1. input
    task_queue,                       // 2. task_queue (fixed string)
    workflow_id,                      // 3. workflow_id (dynamic UUID)
    workflow_type,                    // 4. workflow_type
    None,                             // 5. request_id
    options,                          // 6. options
).await?
```

### 3. Activity Error Handling
```rust
// Retryable errors
Err(ActivityError::Retryable(anyhow!("temporary failure")))

// Non-retryable errors
Err(ActivityError::NonRetryable(anyhow!("permanent failure")))
```

### 4. Workflow Input Deserialization
```rust
// In worker registration
worker.register_wf("my_wf", |ctx: WfContext| async move {
    let input = MyInput::from_json_payload(
        ctx.get_args().first().expect("Missing input")
    )?;
    my_workflow(ctx, input).await
});
```

### 5. Activity Return Values
Activities automatically serialize return values - just return `Result<T, ActivityError>` where `T: Serialize`.

### 6. Workflow Activity Result Handling

**CRITICAL: Never use `.unwrap_ok_payload()`** - it panics on activity failures!

❌ **WRONG** (panics when activity fails):
```rust
let result = ctx.activity(ActivityOptions { ... }).await.unwrap_ok_payload();
```

✅ **CORRECT** (propagates errors gracefully):
```rust
let activity_result = ctx.activity(ActivityOptions {
    activity_type: "my_activity".to_string(),
    input: data.as_json_payload()?,
    start_to_close_timeout: Some(Duration::from_secs(60)),
    ..Default::default()
}).await;

// Properly handle the ActivityResolution
let payload = activity_result
    .success_payload_or_error()?
    .ok_or_else(|| anyhow::anyhow!("Activity returned no payload"))?;

let result = MyType::from_json_payload(&payload)?;
```

**Why this matters**:
- `.unwrap_ok_payload()` immediately panics if the activity returns an error (Retryable or NonRetryable)
- `.success_payload_or_error()?` converts activity failures into workflow errors, allowing proper error propagation
- This enables workflows to handle activity failures gracefully and test failure scenarios

### 7. Common Pitfalls
- ❌ Mixing up task_queue (fixed) and workflow_id (dynamic)
- ❌ Using `serde_json` instead of SDK payload helpers
- ❌ Forgetting `.identity()` on client options
- ❌ Not using `async move` in activity/workflow registration
- ❌ Using `.unwrap_ok_payload()` instead of `.success_payload_or_error()?`
- ❌ Using `.unwrap()` on activity results (use proper error handling)

## Testing Pattern
```rust
// Use from_json_payload in tests too
let result = MyType::from_json_payload(&payload)?;
```

## Beginner Error Tests
See `tests/sdk_best_practices_tests.rs` for comprehensive tests that catch common mistakes:

- ✅ `test_correct_payload_serialization` - Shows proper use of `as_json_payload()`
- ❌ `test_wrong_payload_serialization_fails` - Demonstrates why raw `serde_json` fails
- ✅ `test_correct_empty_input_serialization` - How to handle `()` inputs
- 📋 `test_start_workflow_parameter_order_documentation` - Documents correct parameter order
- 🔑 `test_client_identity_is_required` - Why identity is mandatory
- 🔄 `test_activity_error_types` - Retryable vs NonRetryable vs Cancelled
- 📛 `test_activity_workflow_name_conventions` - Avoid string typos with constants

Run: `cargo test --test sdk_best_practices_tests`

## Integration Testing

### End-to-End Testing with Ephemeral Server (Recommended for CI/CD)

See `tests/e2e_ephemeral_tests.rs` for **fully self-contained** E2E tests:

**Key Pattern - Ephemeral Server Setup:**
```rust
use temporal_sdk_core::ephemeral_server::{TemporalDevServerConfigBuilder, default_cached_download};

// 1. Start ephemeral server (auto-downloads Temporal CLI)
let server_config = TemporalDevServerConfigBuilder::default()
    .exe(default_cached_download())
    .build()?;
let mut server = server_config.start_server().await?;

// 2. Connect client to ephemeral server
let client = ClientOptionsBuilder::default()
    .identity("e2e-worker".to_string())
    .target_url(Url::parse(&format!("http://{}", server.target))?)
    .build()?
    .connect("default", None).await?;

// 3. Create worker, register workflows/activities, run tests...

// 4. Cleanup
server.shutdown().await?;
```

**Run worker with tokio::select!:**
```rust
let result = tokio::select! {
    res = wf_handle.get_workflow_result(Default::default()) => res?,
    _ = worker.run() => panic!("Worker stopped unexpectedly"),
};
```

**Enable ephemeral server feature in Cargo.toml:**
```toml
[dependencies]
temporal-sdk-core = { path = "../sdk-core/core", features = ["ephemeral-server"] }
```

### Test Examples
- ✅ `e2e_iplocate_happy_path` - Full workflow with real HTTP calls
- ❌ `e2e_iplocate_permanent_failure` - NonRetryable error, immediate failure
- 🔄 `e2e_iplocate_retryable_success` - Retryable errors with eventual success

**Run:**
```bash
# All E2E tests (ignored by default - run explicitly in CI)
cargo test --test e2e_ephemeral_tests -- --ignored --test-threads=1

# Single test
cargo test --test e2e_ephemeral_tests e2e_iplocate_happy_path -- --ignored

# NOTE: E2E tests use #[ignore] to prevent running during `cargo test`
```

**Why use ephemeral server tests?**
- ✅ Zero external dependencies
- ✅ Automatic Temporal CLI download
- ✅ Automatic server start/stop
- ✅ Perfect for CI/CD pipelines
- ✅ True end-to-end validation
