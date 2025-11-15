# Rust Temporal Demos Makefile

.PHONY: help install test clean schedule-payments food-ordering temporal-server demo

# Default target
help:
	@echo "Rust Temporal Demos"
	@echo "=================="
	@echo ""
	@echo "Available targets:"
	@echo "  install          - Install Rust dependencies"
	@echo "  test             - Run all tests"
	@echo "  clean            - Clean build artifacts"
	@echo "  temporal-server  - Start Temporal server (requires temporal CLI)"
	@echo "  schedule-payments - Run schedule payments demo"
	@echo "  food-ordering    - Run food ordering demo"
	@echo ""
	@echo "Schedule Payments:"
	@echo "  schedule-payments-worker   - Start schedule payments worker"
	@echo "  schedule-payments-schedule - Create schedule payments schedule"
	@echo "  schedule-payments-starter  - Trigger schedule payments workflow"
	@echo ""
	@echo "Food Ordering:"
	@echo "  food-ordering-worker  - Start food ordering worker"
	@echo "  food-ordering-starter - Start food ordering workflow"

# Install dependencies
install:
	@echo "Installing Rust dependencies..."
	cd schedule-payments-rust && cargo build
	cd food-ordering-rust && cargo build

# Run all tests
test:
	@echo "Running schedule payments tests..."
	cd schedule-payments-rust && cargo test
	@echo "Running food ordering tests..."
	cd food-ordering-rust && cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cd schedule-payments-rust && cargo clean
	cd food-ordering-rust && cargo clean

# Start Temporal server
temporal-server:
	@echo "Starting Temporal server..."
	@echo "Make sure you have the Temporal CLI installed: https://docs.temporal.io/cli"
	temporal server start-dev

# Schedule Payments Demo
schedule-payments: schedule-payments-worker schedule-payments-schedule schedule-payments-starter

schedule-payments-worker:
	@echo "Starting schedule payments worker..."
	cd schedule-payments-rust && cargo run --bin worker

schedule-payments-schedule:
	@echo "Creating schedule payments schedule..."
	cd schedule-payments-rust && cargo run --bin schedule

schedule-payments-starter:
	@echo "Triggering schedule payments workflow..."
	cd schedule-payments-rust && cargo run --bin starter

# Food Ordering Demo
food-ordering: food-ordering-worker food-ordering-starter

food-ordering-worker:
	@echo "Starting food ordering worker..."
	cd food-ordering-rust && cargo run --bin worker

food-ordering-starter:
	@echo "Starting food ordering workflow..."
	cd food-ordering-rust && cargo run --bin starter

# Development helpers
dev-setup:
	@echo "Setting up development environment..."
	@echo "1. Install Rust: https://rustup.rs/"
	@echo "2. Install Temporal CLI: https://docs.temporal.io/cli"
	@echo "3. Run 'make temporal-server' to start Temporal"
	@echo "4. Run 'make install' to build dependencies"
	@echo "5. Run 'make test' to verify everything works"

# Format code
fmt:
	@echo "Formatting code..."
	cd schedule-payments-rust && cargo fmt
	cd food-ordering-rust && cargo fmt

# Lint code
lint:
	@echo "Linting code..."
	cd schedule-payments-rust && cargo clippy
	cd food-ordering-rust && cargo clippy

demo:
	@echo "Starting Temporal + Food App"
	@pkgx overmind s

disrupt:
	pkgx overmind stop worker

continue:
	pkgx overmind r worker
