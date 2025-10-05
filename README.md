# Rust Temporal Demos

This repository contains Rust implementations of Temporal demos, ported from the original Go implementations. The demos showcase Temporal's core features using the Rust SDK.

## Demos

### 1. Schedule Payments (`schedule-payments-rust/`)

A payment scheduling system that demonstrates:
- **Scheduled Workflows**: Automated daily payment processing
- **Child Workflows**: Parallel payment processing
- **Activities**: Database queries and payment processing
- **Temporal Schedules**: Recurring workflow execution

**Key Features:**
- Daily, weekly, and monthly payment schedules
- Parallel payment processing using child workflows
- Simulated database queries and payment processing
- Configurable schedule intervals

### 2. Food Ordering (`food-ordering-rust/`)

A food ordering system that demonstrates:
- **Workflow Updates**: Restaurant status updates
- **Workflow Queries**: Customer order status queries
- **Update Validators**: Status validation
- **Conditional Waiting**: Waiting for specific conditions

**Key Features:**
- Complete order lifecycle management
- Real-time status updates from restaurants
- Customer order status queries
- Automatic refunds for rejected orders
- SMS notifications at each stage

### 3. IP Location (`iplocate-rust/`)

A simple IP geolocation workflow that demonstrates:
- **Basic Activities**: External API calls (IP lookup, geolocation)
- **Activity Options**: Timeout configuration and retry policies
- **Error Handling**: Retryable vs NonRetryable errors
- **Mock Testing**: Controlled failure scenarios without servers
- **Golden Snapshot Testing**: Realistic test data from fixtures

**Key Features:**
- Fetch public IP address from icanhazip.com
- Retrieve geolocation data from ip-api.com
- Optional timer/sleep between activities
- Comprehensive test suite with 17 tests covering:
  - Real HTTP API calls
  - Network failure simulations (no WiFi, flaky network, high latency)
  - Golden snapshot data for deterministic testing
  - Exponential backoff calculations
  - Activity timeout scenarios

## Prerequisites

- Rust 1.70+
- Temporal server running locally

## Quick Start

### 1. Start Temporal Server

For local development, use the ephemeral server:

```bash
temporal server start-dev
```

This starts a self-contained Temporal server suitable for development and testing.

### 2. Run Schedule Payments Demo

```bash
# Terminal 1: Start the worker
cd schedule-payments-rust
cargo run --bin worker

# Terminal 2: Create the schedule (runs every minute for demo purposes)
cargo run --bin schedule

# Terminal 3: Trigger a manual run
cargo run --bin starter
```

### 3. Run Food Ordering Demo

```bash
# Terminal 1: Start the worker
cd food-ordering-rust
cargo run --bin worker

# Terminal 2: Start an order
cargo run --bin starter
```

Then use the Temporal Web UI (http://localhost:8080) to:
- Query order status
- Update order status (ACCEPTED, PREPARING, READY, COMPLETED, REJECTED)

### 4. Run IP Location Demo

```bash
# Terminal 1: Start the worker
cd iplocate-rust
cargo run --bin worker

# Terminal 2: Trigger the workflow
cargo run --bin starter
```

**What it demonstrates:**
- Simple two-activity workflow (get IP → get location)
- Real HTTP calls to external APIs
- Workflow completes and returns structured output
- Check Temporal Web UI (http://localhost:8233) to see workflow execution history

## Environment Variables

- `TEMPORAL_ADDRESS`: Temporal server address (default: `localhost:7233`)

## Testing

All demos include comprehensive tests that use Temporal's test environment with ephemeral servers:

```bash
# Test schedule payments
cd schedule-payments-rust
cargo test

# Test food ordering
cd food-ordering-rust
cargo test

# Test IP location (includes 17 tests)
cd iplocate-rust
cargo test

# Run specific test suites
cargo test --test integration_tests        # Basic integration tests (8 tests)
cargo test --test advanced_integration_tests  # Advanced mock tests (9 tests)
```

### IP Location Test Features

The IP location demo showcases advanced testing patterns:

**Integration Tests (`tests/integration_tests.rs`):**
- Real HTTP API calls to icanhazip.com and ip-api.com
- Type serialization/deserialization validation
- API response structure verification

**Advanced Tests (`tests/advanced_integration_tests.rs`):**
- **Network failure simulations**: No WiFi (NonRetryable), flaky network (Retryable), high latency
- **Golden snapshot data**: Deterministic testing with `tests/fixtures/golden_ips.json`
- **Activity options**: Timeout configurations, exponential backoff calculations
- **Error type preservation**: Retryable vs NonRetryable error handling
- **Mock activities**: Test activity logic without running Temporal server

## Architecture

### Temporal Rust SDK Patterns

The implementations follow Temporal's idiomatic patterns:

1. **Workflows**: Deterministic functions that orchestrate business logic
2. **Activities**: Non-deterministic functions for external operations
3. **Queries**: Read-only access to workflow state
4. **Updates**: Modifications to workflow state with validation
5. **Child Workflows**: Parallel execution of related workflows
6. **Schedules**: Automated workflow triggers

### Key Dependencies

- `temporal-sdk-core`: Core Temporal SDK for Rust
- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `chrono`: Date/time handling
- `uuid`: Unique identifiers
- `tracing`: Structured logging
- `anyhow`: Error handling

## Development

### Project Structure

```
├── schedule-payments-rust/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── activities.rs
│   │   ├── data.rs
│   │   ├── workflows.rs
│   │   ├── tests.rs
│   │   └── bin/
│   │       ├── worker.rs
│   │       ├── starter.rs
│   │       └── schedule.rs
│   ├── Cargo.toml
│   └── README.md
├── food-ordering-rust/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── activities.rs
│   │   ├── constants.rs
│   │   ├── types.rs
│   │   ├── workflows.rs
│   │   ├── tests.rs
│   │   └── bin/
│   │       ├── worker.rs
│   │       └── starter.rs
│   ├── Cargo.toml
│   └── README.md
├── iplocate-rust/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── activities.rs
│   │   ├── types.rs
│   │   ├── workflows.rs
│   │   └── bin/
│   │       ├── worker.rs
│   │       └── starter.rs
│   ├── tests/
│   │   ├── integration_tests.rs
│   │   ├── advanced_integration_tests.rs
│   │   └── fixtures/
│   │       └── golden_ips.json
│   └── Cargo.toml
└── README-RUST.md
```

### Adding New Demos

To add a new demo:

1. Create a new directory with `Cargo.toml`
2. Implement workflows, activities, and types
3. Add worker and starter binaries
4. Include comprehensive tests
5. Add documentation

## Comparison with Go Implementation

The Rust implementations maintain feature parity with the original Go versions while leveraging Rust's strengths:

- **Type Safety**: Compile-time guarantees for data structures
- **Memory Safety**: No runtime memory errors
- **Performance**: Zero-cost abstractions and efficient async runtime
- **Error Handling**: Explicit error handling with `Result<T, E>`
- **Concurrency**: Safe concurrent programming with async/await

## Contributing

1. Follow Rust naming conventions
2. Use `cargo fmt` for formatting
3. Use `cargo clippy` for linting
4. Write comprehensive tests
5. Update documentation

## License

Licensed under the Apache License, Version 2.0. See the original Go implementations for the complete license text.
