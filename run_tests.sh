#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Error handling
set -e
trap 'handle_error $? $LINENO' ERR

handle_error() {
    local exit_code=$1
    local line_number=$2
    echo -e "${RED}Error: Command failed at line $line_number with exit code $exit_code${NC}"
}

# Set environment variables for macOS
setup_macos_env() {
    if [[ "$(uname)" == "Darwin" ]]; then
        echo -e "${YELLOW}Setting up macOS environment...${NC}"
        export MACOSX_DEPLOYMENT_TARGET=11.0
        export SDKROOT=$(xcrun --show-sdk-path)
        export CFLAGS="-arch arm64"
        export CXXFLAGS="-arch arm64"
        export RUSTFLAGS="-C target-cpu=apple-m1"
    fi
}

# Clean build artifacts
clean_build() {
    echo -e "${YELLOW}Cleaning build artifacts...${NC}"
    cargo clean
    rm -rf target/
}

# Run unit tests
run_unit_tests() {
    echo -e "${YELLOW}Running unit tests...${NC}"
    RUST_BACKTRACE=1 cargo test --lib -- --nocapture
}

# Run doc tests
run_doc_tests() {
    echo -e "${YELLOW}Running doc tests...${NC}"
    RUST_BACKTRACE=1 cargo test --doc
}

# Run example tests
run_example_tests() {
    echo -e "${YELLOW}Running example tests...${NC}"
    for example in examples/basic/*; do
        if [ -f "$example" ]; then
            echo -e "${YELLOW}Testing example: $(basename "$example")${NC}"
            RUST_BACKTRACE=1 cargo run --example "$(basename "$example" .rs)"
        fi
    done
}

# Run all tests
run_all_tests() {
    echo -e "${YELLOW}Starting test suite...${NC}"

    # Setup environment
    setup_macos_env

    # Clean build
    clean_build

    # Format code
    echo -e "${YELLOW}Formatting code...${NC}"
    cargo fmt

    # Run clippy with allowed warnings
    echo -e "${YELLOW}Running clippy...${NC}"
    cargo clippy -- -A warnings

    # Run tests
    run_unit_tests
    run_doc_tests
    run_example_tests

    echo -e "${GREEN}All tests completed successfully!${NC}"
}

# Show test results
show_test_results() {
    echo -e "\n${YELLOW}Test Summary:${NC}"
    echo -e "Unit Tests: ${GREEN}✓${NC}"
    echo -e "Doc Tests: ${GREEN}✓${NC}"
    echo -e "Example Tests: ${GREEN}✓${NC}"
}

# Main execution
main() {
    case "$1" in
        "unit")
            setup_macos_env
            run_unit_tests
            ;;
        "doc")
            setup_macos_env
            run_doc_tests
            ;;
        "examples")
            setup_macos_env
            run_example_tests
            ;;
        "all" | "")
            run_all_tests
            show_test_results
            ;;
        *)
            echo "Usage: $0 [unit|doc|examples|all]"
            exit 1
            ;;
    esac
}

# Run main function with arguments
main "$@"
