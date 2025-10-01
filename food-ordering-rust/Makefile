# Makefile for Food Ordering Rust
# Copyright 2025 Simon Emms <simon@simonemms.com>

.PHONY: help build test clean run-worker run-starter run-all

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the project"
	@echo "  test       - Run tests"
	@echo "  clean      - Clean build artifacts"
	@echo "  run-worker - Run the worker"
	@echo "  run-starter - Run the starter (triggers workflow)"
	@echo "  run-all    - Run worker and starter together"

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test --test integration_tests

# Clean build artifacts
clean:
	cargo clean

# Run the worker
run-worker: build
	cargo run --bin worker

# Run the starter (triggers workflow)
run-starter: build
	cargo run --bin starter

# Run worker and starter together
run-all: build
	@echo "Starting worker..."
	cargo run --bin worker &
	@echo "Waiting for worker to start..."
	sleep 3
	@echo "Starting workflow..."
	cargo run --bin starter

# Run tests with verbose output
test-verbose:
	cargo test -- --nocapture

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy
