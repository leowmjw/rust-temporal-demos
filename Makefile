# Makefile for Rust Temporal Demos
# Copyright 2025 Simon Emms <simon@simonemms.com>

.PHONY: help build test clean run-schedule-payments run-food-ordering run-workers

# Default target
help:
	@echo "Available targets:"
	@echo "  build                 - Build all projects"
	@echo "  test                  - Run all tests"
	@echo "  clean                 - Clean build artifacts"
	@echo "  run-schedule-payments - Run schedule-payments worker and starter"
	@echo "  run-food-ordering     - Run food-ordering worker and starter"
	@echo "  run-workers           - Run both workers in background"
	@echo "  stop-workers          - Stop all running workers"

# Build all projects
build:
	@echo "Building schedule-payments-rust..."
	cd schedule-payments-rust && cargo build
	@echo "Building food-ordering-rust..."
	cd food-ordering-rust && cargo build

# Run all tests
test:
	@echo "Testing schedule-payments-rust..."
	cd schedule-payments-rust && cargo test
	@echo "Testing food-ordering-rust..."
	cd food-ordering-rust && cargo test

# Clean build artifacts
clean:
	@echo "Cleaning schedule-payments-rust..."
	cd schedule-payments-rust && cargo clean
	@echo "Cleaning food-ordering-rust..."
	cd food-ordering-rust && cargo clean

# Run schedule-payments worker and starter
run-schedule-payments: build
	@echo "Starting schedule-payments worker..."
	cd schedule-payments-rust && cargo run --bin worker &
	@echo "Waiting for worker to start..."
	sleep 3
	@echo "Starting schedule-payments workflow..."
	cd schedule-payments-rust && cargo run --bin starter

# Run food-ordering worker and starter
run-food-ordering: build
	@echo "Starting food-ordering worker..."
	cd food-ordering-rust && cargo run --bin worker &
	@echo "Waiting for worker to start..."
	sleep 3
	@echo "Starting food-ordering workflow..."
	cd food-ordering-rust && cargo run --bin starter

# Run both workers in background
run-workers: build
	@echo "Starting schedule-payments worker..."
	cd schedule-payments-rust && cargo run --bin worker &
	@echo "Starting food-ordering worker..."
	cd food-ordering-rust && cargo run --bin worker &
	@echo "Both workers started. Use 'make stop-workers' to stop them."

# Stop all running workers
stop-workers:
	@echo "Stopping all workers..."
	pkill -f "cargo run --bin worker" || true
	@echo "Workers stopped."

# Run tests with verbose output
test-verbose:
	@echo "Testing schedule-payments-rust (verbose)..."
	cd schedule-payments-rust && cargo test -- --nocapture
	@echo "Testing food-ordering-rust (verbose)..."
	cd food-ordering-rust && cargo test -- --nocapture

# Check code without building
check:
	@echo "Checking schedule-payments-rust..."
	cd schedule-payments-rust && cargo check
	@echo "Checking food-ordering-rust..."
	cd food-ordering-rust && cargo check

# Format code
fmt:
	@echo "Formatting schedule-payments-rust..."
	cd schedule-payments-rust && cargo fmt
	@echo "Formatting food-ordering-rust..."
	cd food-ordering-rust && cargo fmt

# Run clippy linter
clippy:
	@echo "Running clippy on schedule-payments-rust..."
	cd schedule-payments-rust && cargo clippy
	@echo "Running clippy on food-ordering-rust..."
	cd food-ordering-rust && cargo clippy