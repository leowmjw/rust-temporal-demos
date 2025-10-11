# Integration Tests

## Test Categories

### 1. SDK Best Practices Tests (`sdk_best_practices_tests.rs`)
Unit tests that demonstrate common Temporal SDK mistakes and correct patterns.

**Run:**
```bash
cargo test --test sdk_best_practices_tests
```

**Tests:**
- ✅ Correct payload serialization with `as_json_payload()`
- ❌ Why raw `serde_json` fails
- 🔑 Client identity requirements
- 📋 start_workflow parameter order
- 🔄 Activity error types (Retryable vs NonRetryable)

### 2. Mock Integration Tests (`advanced_integration_tests.rs`, `integration_tests.rs`)
Mock-based integration tests that validate workflow and activity logic.

**Run:**
```bash
cargo test --test integration_tests
cargo test --test advanced_integration_tests
```

### 3. ⭐ End-to-End Tests with Ephemeral Server (`e2e_ephemeral_tests.rs`)
**Fully self-contained** end-to-end tests that spin up their own Temporal server.

✨ **Perfect for CI/CD - NO external dependencies!**

**Run:**
```bash
# All E2E tests (ignored by default - only run explicitly)
cargo test --test e2e_ephemeral_tests -- --ignored --test-threads=1

# Single test
cargo test --test e2e_ephemeral_tests e2e_iplocate_happy_path -- --ignored

# NOTE: E2E tests are marked with #[ignore] to prevent them from running
# during regular `cargo test`. They only run when explicitly requested with --ignored.
```

**Test Scenarios:**
- ✅ **Happy Path** (`e2e_iplocate_happy_path`) - Full workflow with real HTTP calls
- ❌ **Permanent Failure** (`e2e_iplocate_permanent_failure`) - NonRetryable error, workflow fails immediately
- 🔄 **Retry Success** (`e2e_iplocate_retryable_success`) - Retryable errors, eventual success

**What Happens:**
1. Downloads & starts ephemeral Temporal CLI server
2. Creates client connected to ephemeral server
3. Registers workflows and activities
4. Starts workflow and runs worker
5. Waits for result and validates
6. Shuts down ephemeral server cleanly


## Test Output Examples

### E2E Test Output:
```
✅ Ephemeral Temporal server started on 127.0.0.1:58472
✅ Workflow started: e2e-happy-ca7f9e2d-3b1a-4c8f-9d5e-1a2b3c4d5e6f
✅ E2E Happy Path Success!
   IP: 198.51.100.42
   Location: San Francisco, California, United States
✅ Ephemeral server shut down cleanly
```

```
✅ E2E Permanent Failure Test Passed!
   Workflow failed as expected: WIFI_DISABLED: No network connection available
```

```
   Attempt 1: Simulating transient network failure...
   Attempt 2: Simulating transient network failure...
   Attempt 3: Success!
✅ E2E Retry Test Success!
   Total attempts: 3
   IP: 203.0.113.42
   Location: Mountain View, California, United States
```

## CI/CD Integration

The E2E tests are perfect for CI/CD because they:
- ✅ Require **zero external setup**
- ✅ Download Temporal CLI automatically
- ✅ Start/stop server automatically
- ✅ Clean up resources properly
- ✅ Run in parallel (use `--test-threads=1` if needed)

**Example GitHub Actions:**
```yaml
- name: Run E2E Tests
  run: cargo test --test e2e_ephemeral_tests -- --test-threads=1
```
