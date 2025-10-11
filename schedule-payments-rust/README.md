# Schedule Payments (Rust)

Look for payments due today and schedule them

## Overview

This is an example of a company that needs to trigger a series of regular
payments. Some payments are daily, some are weekly and others are monthly.
A Temporal schedule is created which runs an activity to find the payments that
are due today and then triggers a child workflow to make each of these payments.

The logic to get due payments is very much a demo - in a production system, you
would want to ensure that you only pull transactions that need to be made from your
database. There is no database in this example, but a simple function to filter
payments not required "today". The purpose is demonstrate how Temporal can
create robust schedules, not to show how to pull things from a database.

## Prerequisites

- Rust 1.70+
- Temporal server running (use `temporal server start-dev` for local development)

## Steps to run

### Run the worker

```sh
cargo run --bin worker
```

The worker is where the workflow is defined.

### Create the schedule

```sh
cargo run --bin schedule
```

Create the schedule.

By default, the schedule runs daily at 2am. From a demo point of view, you don't
want to wait all day for this so it's also triggers every 60 seconds.

### Trigger a run

```sh
cargo run --bin starter
```

This enables you to trigger an individual run, testing out the workflow.

## Environment Variables

- `TEMPORAL_ADDRESS`: The address of the Temporal server (default: `localhost:7233`)

## Testing

Run the tests with:

```sh
cargo test
```

## Architecture

This Rust implementation follows Temporal's idiomatic patterns:

- **Workflows**: Deterministic functions that orchestrate business logic
- **Activities**: Non-deterministic functions that perform external operations
- **Child Workflows**: Used to process payments in parallel
- **Schedules**: Automated triggers for recurring workflows

The implementation uses:
- `temporal-sdk-core` for Temporal integration
- `tokio` for async runtime
- `serde` for serialization
- `chrono` for date/time handling
- `uuid` for unique identifiers
