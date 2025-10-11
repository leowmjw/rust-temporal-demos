.PHONY: dev worker kickoff build test clean check

# Development target using cargo-watch
dev:
	# Starts Overmind to mintor Procfile ..
	@pkgx overmind s

unit:
	@echo "Starting unit test in loop .."
	@cargo watch -x test

worker:
	@cargo watch -x 'run --bin worker'

kickoff:
	@cargo watch -x 'run --bin starter'

# Build the project
build:
	cargo build

# Build for release
release:
	cargo build --release

# Run tests
test:
	cargo test

# Check the project (faster than build)
check:
	cargo check

# Clean build artifacts
clean:
	cargo clean

# Run clippy for linting
clippy:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt -- --check
