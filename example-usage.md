# Example Usage

This document provides step-by-step examples of how to run the Rust Temporal demos.

## Prerequisites

1. **Install Rust**: https://rustup.rs/
2. **Install Temporal CLI**: https://docs.temporal.io/cli (optional, for full server functionality)

## Quick Start

### 1. Start Temporal Server

```bash
# Start the development server
temporal server start-dev

# Or use the Makefile
make -f Makefile-rust temporal-server
```

The server will be available at:
- Temporal Web UI: http://localhost:8080
- gRPC endpoint: localhost:7233

### 2. Run Schedule Payments Demo

```bash
# Terminal 1: Start the worker
cd schedule-payments-rust
cargo run --bin worker

# Terminal 2: Create the schedule (runs every minute for demo)
cargo run --bin schedule

# Terminal 3: Trigger a manual run
cargo run --bin starter
```

**What happens:**
1. Worker starts and registers workflows/activities
2. Schedule is created to run every minute (and daily at 2am)
3. Manual trigger finds payments due today
4. Child workflows process each payment in parallel

### 3. Run Food Ordering Demo

```bash
# Terminal 1: Start the worker
cd food-ordering-rust
cargo run --bin worker

# Terminal 2: Start an order
cargo run --bin starter
```

**What happens:**
1. Worker starts and registers workflows/activities
2. Order workflow starts with sample order
3. Payment is processed
4. Order status is set to PENDING
5. Workflow waits for restaurant updates

**Update order status via Temporal Web UI:**
1. Go to http://localhost:8080
2. Find your workflow
3. Use the "Update" tab to change status to:
   - `ACCEPTED` - Restaurant accepted
   - `PREPARING` - Food being prepared
   - `READY` - Ready for pickup
   - `COMPLETED` - Order completed
   - `REJECTED` - Order rejected (triggers refund)

## Testing with Ephemeral Servers

The demos include comprehensive tests that use ephemeral Temporal servers:

```bash
# Run all tests
./run-tests.sh

# Or run tests individually
cd schedule-payments-rust && cargo test
cd food-ordering-rust && cargo test
```

**Test features:**
- Ephemeral server setup/teardown
- Workflow execution testing
- Activity testing
- Update/query testing
- Error handling testing

## Using the Makefile

The `Makefile-rust` provides convenient commands:

```bash
# See all available commands
make -f Makefile-rust help

# Install dependencies
make -f Makefile-rust install

# Run all tests
make -f Makefile-rust test

# Run schedule payments demo
make -f Makefile-rust schedule-payments

# Run food ordering demo
make -f Makefile-rust food-ordering

# Format code
make -f Makefile-rust fmt

# Lint code
make -f Makefile-rust lint
```

## Environment Variables

Both demos respect the `TEMPORAL_ADDRESS` environment variable:

```bash
# Use custom Temporal server
export TEMPORAL_ADDRESS="my-temporal-server:7233"
cargo run --bin worker
```

## Troubleshooting

### Common Issues

1. **"Connection refused" errors**
   - Make sure Temporal server is running: `temporal server start-dev`
   - Check the server address in `TEMPORAL_ADDRESS`

2. **"Workflow not found" errors**
   - Make sure the worker is running and has registered the workflow
   - Check that the task queue names match

3. **Test failures**
   - Make sure you have the latest version of `temporal-sdk-core`
   - Check that all dependencies are installed: `cargo build`

### Debug Mode

Enable debug logging:

```bash
RUST_LOG=debug cargo run --bin worker
```

### Temporal Web UI

Use the Temporal Web UI to:
- Monitor workflow execution
- View workflow history
- Query workflow state
- Update workflow state
- Debug issues

Access at: http://localhost:8080

## Next Steps

1. **Explore the code**: Read through the workflow and activity implementations
2. **Modify the demos**: Try changing payment amounts, order items, or status flows
3. **Add new features**: Implement additional activities or workflows
4. **Production deployment**: See Temporal's production deployment guides

## Resources

- [Temporal Documentation](https://docs.temporal.io/)
- [Temporal Rust SDK](https://github.com/temporalio/sdk-core)
- [Temporal Web UI](https://github.com/temporalio/ui)
- [Temporal CLI](https://docs.temporal.io/cli)
