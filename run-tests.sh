#!/bin/bash

# Rust Temporal Demos Test Runner
# This script runs tests with ephemeral Temporal servers

set -e

echo "🚀 Rust Temporal Demos Test Runner"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

print_status "Rust/Cargo is installed"

# Check if temporal CLI is installed
if ! command -v temporal &> /dev/null; then
    print_warning "Temporal CLI is not installed. Tests will use embedded test environment."
    print_warning "For full functionality, install Temporal CLI: https://docs.temporal.io/cli"
fi

# Function to run tests for a demo
run_demo_tests() {
    local demo_name=$1
    local demo_dir=$2
    
    echo ""
    echo "🧪 Testing $demo_name..."
    echo "=========================="
    
    if [ ! -d "$demo_dir" ]; then
        print_error "Demo directory $demo_dir not found"
        return 1
    fi
    
    cd "$demo_dir"
    
    # Run tests
    if cargo test; then
        print_status "$demo_name tests passed"
    else
        print_error "$demo_name tests failed"
        return 1
    fi
    
    cd ..
}

# Run tests for both demos
echo ""
echo "Running tests with ephemeral Temporal servers..."
echo "This may take a few minutes as it starts temporary servers..."

# Test schedule payments
run_demo_tests "Schedule Payments" "schedule-payments-rust"

# Test food ordering
run_demo_tests "Food Ordering" "food-ordering-rust"

echo ""
echo "🎉 All tests completed successfully!"
echo ""
echo "Next steps:"
echo "1. Start Temporal server: make -f Makefile-rust temporal-server"
echo "2. Run schedule payments demo: make -f Makefile-rust schedule-payments"
echo "3. Run food ordering demo: make -f Makefile-rust food-ordering"
echo ""
echo "For more information, see README-RUST.md"
