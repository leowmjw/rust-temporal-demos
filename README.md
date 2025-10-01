# Rust Temporal Demos

This repository contains Rust implementations of two Temporal workflow applications, ported from Go to demonstrate the Temporal Rust SDK capabilities.

## Projects

### 1. Schedule Payments (`schedule-payments-rust/`)
A payment scheduling system that finds payments due on a given day and processes them in parallel using child workflows.

**Features:**
- Finds payments due today based on schedule (daily, weekly, monthly)
- Processes payments in parallel using child workflows
- Simulates payment processing with realistic delays
- Supports scheduled execution via Temporal schedules

**Workflows:**
- `find_due_payments_workflow`: Main workflow that finds and processes due payments
- `make_payment`: Child workflow that processes individual payments

**Activities:**
- `find_payments_for_day`: Simulates database query for due payments
- `send_payment`: Simulates payment processing

### 2. Food Ordering (`food-ordering-rust/`)
A food ordering system that manages the complete order lifecycle from payment to completion.

**Features:**
- Handles payment processing
- Sends notifications at each stage
- Manages order status updates
- Simulates restaurant order processing

**Workflows:**
- `order_workflow`: Main workflow that manages the order lifecycle

**Activities:**
- `take_payment`: Processes customer payment
- `send_text_message`: Sends status notifications
- `refund_payment`: Handles payment refunds

## Prerequisites

- Rust 1.90.0 or later
- Cargo
- Make (optional, for using Makefiles)

## Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd rust-tdemo
```

### 2. Build All Projects

```bash
make build
```

### 3. Run Tests

```bash
make test
```

### 4. Run Applications

#### Schedule Payments
```bash
# Run worker and starter together
make run-schedule-payments

# Or run individually
cd schedule-payments-rust
make run-worker    # In one terminal
make run-starter   # In another terminal
```

#### Food Ordering
```bash
# Run worker and starter together
make run-food-ordering

# Or run individually
cd food-ordering-rust
make run-worker    # In one terminal
make run-starter   # In another terminal
```

## Project Structure

```
rust-tdemo/
├── schedule-payments-rust/          # Payment scheduling application
│   ├── src/
│   │   ├── activities.rs            # Activity implementations
│   │   ├── data.rs                  # Data structures and generation
│   │   ├── workflows.rs             # Workflow implementations
│   │   ├── lib.rs                   # Library entry point
│   │   └── bin/                     # Binary executables
│   │       ├── worker.rs            # Temporal worker
│   │       ├── starter.rs           # Workflow starter
│   │       └── schedule.rs          # Schedule creator
│   ├── tests/
│   │   └── integration_tests.rs     # Integration tests
│   ├── Cargo.toml                   # Project dependencies
│   └── Makefile                     # Build automation
├── food-ordering-rust/              # Food ordering application
│   ├── src/
│   │   ├── activities.rs            # Activity implementations
│   │   ├── constants.rs             # Application constants
│   │   ├── types.rs                 # Data type definitions
│   │   ├── workflows.rs             # Workflow implementations
│   │   ├── lib.rs                   # Library entry point
│   │   └── bin/                     # Binary executables
│   │       ├── worker.rs            # Temporal worker
│   │       └── starter.rs           # Workflow starter
│   ├── tests/
│   │   └── integration_tests.rs     # Integration tests
│   ├── Cargo.toml                   # Project dependencies
│   └── Makefile                     # Build automation
├── sdk-core/                        # Temporal Rust SDK (cloned)
└── Makefile                         # Root build automation
```

## Architecture

### Temporal Patterns Used

1. **Workflows**: Deterministic business logic that orchestrates activities
2. **Activities**: Non-deterministic operations that interact with external systems
3. **Child Workflows**: Parallel execution of related workflows
4. **Timers**: Workflow delays and scheduling
5. **Updates**: Dynamic workflow state modification (food-ordering)
6. **Schedules**: Automated workflow triggers (schedule-payments)

### Rust-Specific Adaptations

1. **Error Handling**: Uses Rust's `Result` types with Temporal's error types
2. **Async/Await**: Leverages Rust's async runtime for workflow execution
3. **Ownership**: Proper memory management with Rust's ownership system
4. **Type Safety**: Strong typing with `serde` for serialization
5. **Testing**: Comprehensive test suites using ephemeral Temporal servers

## Development

### Running Tests

Each project includes comprehensive integration tests that use Temporal's ephemeral server:

```bash
# Run all tests
make test

# Run tests with verbose output
make test-verbose

# Run tests for specific project
cd schedule-payments-rust && cargo test
cd food-ordering-rust && cargo test
```

### Code Quality

```bash
# Check code without building
make check

# Format code
make fmt

# Run linter
make clippy
```

### Individual Project Commands

#### Schedule Payments
```bash
cd schedule-payments-rust

# Build
make build

# Run worker
make run-worker

# Trigger workflow
make run-starter

# Create schedule
make run-schedule

# Run all together
make run-all
```

#### Food Ordering
```bash
cd food-ordering-rust

# Build
make build

# Run worker
make run-worker

# Trigger workflow
make run-starter

# Run all together
make run-all
```

## Configuration

### Environment Variables

- `TEMPORAL_ADDRESS`: Temporal server address (default: `http://localhost:7233`)

### Task Queues

- Schedule Payments: `payments`
- Food Ordering: `order_food`

## Testing

The test suites use Temporal's ephemeral server for isolated testing:

- **Unit Tests**: Test individual activities and workflows
- **Integration Tests**: Test complete workflow execution
- **End-to-End Tests**: Test full application workflows
- **Worker Tests**: Test worker registration and execution

## Key Differences from Go Implementation

1. **Activity Functions**: Standalone functions instead of struct methods
2. **Error Handling**: Rust's `Result` types instead of Go's error interface
3. **Serialization**: `serde` instead of Go's JSON marshaling
4. **Async Runtime**: Tokio instead of Go's goroutines
5. **Type Safety**: Compile-time type checking with Rust's type system

## Troubleshooting

### Common Issues

1. **Build Errors**: Ensure you have Rust 1.90.0+ and all dependencies
2. **Connection Issues**: Check that Temporal server is running
3. **Worker Issues**: Ensure only one worker per task queue is running
4. **Test Failures**: Run tests individually to isolate issues

### Debug Mode

Run with debug logging:

```bash
RUST_LOG=debug cargo run --bin worker
RUST_LOG=debug cargo run --bin starter
```

## Contributing

1. Follow Rust coding standards
2. Add tests for new functionality
3. Update documentation as needed
4. Use conventional commit messages

## License

Licensed under the Apache License, Version 2.0. See LICENSE file for details.

## Acknowledgments

- Ported from Go Temporal SDK examples
- Uses Temporal Rust SDK (alpha)
- Inspired by Temporal's best practices and patterns