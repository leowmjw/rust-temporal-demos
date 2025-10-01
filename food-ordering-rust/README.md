# Food Ordering (Rust)

A food ordering system that demonstrates Temporal workflows with queries and updates.

## Overview

This is an example of a food ordering system that demonstrates how to use Temporal's
queries and updates features. The workflow handles the complete order lifecycle from
payment to completion, with the ability for restaurants to update order status and
customers to query the current status.

The system includes:
- Payment processing
- Order status tracking
- Restaurant updates via workflow updates
- Customer queries for order status
- Automatic refunds for rejected orders

## Prerequisites

- Rust 1.70+
- Temporal server running (use `temporal server start-dev` for local development)

## Steps to run

### Run the worker

```sh
cargo run --bin worker
```

The worker is where the workflow is defined and handles all the business logic.

### Start an order

```sh
cargo run --bin starter
```

This creates a sample order and starts the workflow. The workflow will:
1. Take payment
2. Set status to PENDING
3. Wait for restaurant to update status
4. Send notifications at each step

### Update order status (via Temporal UI or API)

You can update the order status using the Temporal Web UI or by calling the update
endpoint. Valid statuses are:
- `ACCEPTED` - Restaurant accepted the order
- `PREPARING` - Restaurant is preparing the food
- `READY` - Food is ready for pickup/delivery
- `COMPLETED` - Order completed successfully
- `REJECTED` - Restaurant rejected the order (triggers refund)

### Query order status

You can query the current order status using the Temporal Web UI or by calling the
query endpoint with the query type `GET_STATUS`.

## Environment Variables

- `TEMPORAL_ADDRESS`: The address of the Temporal server (default: `localhost:7233`)

## Testing

Run the tests with:

```sh
cargo test
```

The tests include:
- Complete order workflow test
- Order rejection workflow test
- Activity unit tests
- Order state management tests
- Status parsing tests

## Architecture

This Rust implementation demonstrates Temporal's advanced features:

- **Workflows**: Orchestrate the order lifecycle
- **Activities**: Handle external operations (payments, notifications)
- **Queries**: Allow customers to check order status
- **Updates**: Allow restaurants to update order status
- **Update Validators**: Ensure only valid status updates are accepted
- **Conditional Waiting**: Wait for specific conditions (order completion)

The implementation uses:
- `temporal-sdk-core` for Temporal integration
- `tokio` for async runtime
- `serde` for serialization
- `tracing` for logging
- `anyhow` for error handling

## Order Flow

1. **Order Creation**: Customer creates order with items and delivery details
2. **Payment**: System processes payment
3. **Pending**: Order is sent to restaurant for review
4. **Restaurant Updates**: Restaurant can accept, reject, or update status
5. **Notifications**: Customer receives SMS updates at each stage
6. **Completion**: Order is marked as completed or refunded if rejected

## Sample Products

The system includes sample products:
- Margherita Pizza (£12.99)
- Pepperoni Pizza (£14.99)
- Caesar Salad (£8.99)
- Chicken Wings (£9.99)
- Coca Cola (£2.99)
