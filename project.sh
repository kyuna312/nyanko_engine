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
    fi
}

# Clean project
clean_project() {
    echo -e "${YELLOW}Cleaning project...${NC}"
    rm -rf target/
    rm -rf examples/
    cargo clean
}

# Create project structure
create_structure() {
    echo -e "${YELLOW}Creating project structure...${NC}"

    # Create directories
    mkdir -p src/{core,graphics,input,utils,platform,math}
    mkdir -p examples/basic
    mkdir -p tests

    # Create source files
    create_source_files

    # Create example files
    create_example_files

    # Create test files
    create_test_files
}

# Create source files
create_source_files() {
    echo -e "${YELLOW}Creating source files...${NC}"

    # Create lib.rs
    cat > src/lib.rs << 'EOF'
pub mod core;
pub mod graphics;
pub mod input;
pub mod utils;
pub mod platform;
pub mod math;

pub use graphics::*;
pub use input::*;
pub use platform::*;
pub use math::*;
EOF

    # Create other module files
    # ... (Add other file creation commands)
}

# Create example files
create_example_files() {
    echo -e "${YELLOW}Creating example files...${NC}"

    # Create basic window example
    cat > examples/basic/window.rs << 'EOF'
use nyanko_engine::*;

fn main() {
    println!("Basic window example");
}
EOF
}

# Create test files
create_test_files() {
    echo -e "${YELLOW}Creating test files...${NC}"

    # Create basic tests
    cat > tests/lib.rs << 'EOF'
use nyanko_engine::*;

#[test]
fn test_basic() {
    assert!(true);
}
EOF
}

# Run tests
run_tests() {
    echo -e "${YELLOW}Running tests...${NC}"
    RUST_BACKTRACE=1 cargo test --all-features -- --nocapture
}

# Run examples
run_examples() {
    echo -e "${YELLOW}Running examples...${NC}"
    for example in examples/basic/*; do
        if [ -f "$example" ]; then
            echo -e "${YELLOW}Running example: $(basename "$example")${NC}"
            RUST_BACKTRACE=1 cargo run --example "$(basename "$example" .rs)"
        fi
    done
}

# Setup project
setup_project() {
    echo -e "${YELLOW}Setting up project...${NC}"
    clean_project
    create_structure
    cargo build
}

# Main execution
main() {
    case "$1" in
        "setup")
            setup_macos_env
            setup_project
            ;;
        "test")
            setup_macos_env
            run_tests
            ;;
        "examples")
            setup_macos_env
            run_examples
            ;;
        "all" | "")
            setup_macos_env
            setup_project
            run_tests
            run_examples
            ;;
        *)
            echo "Usage: $0 [setup|test|examples|all]"
            exit 1
            ;;
    esac
}

# Run main function with arguments
main "$@"
