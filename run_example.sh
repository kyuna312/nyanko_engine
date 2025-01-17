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

# Create example directory structure and example files
setup_examples() {
    echo -e "${YELLOW}Setting up example directory structure...${NC}"

    # Create directories
    mkdir -p examples/{minimal,drag_drop,menu,tree_view}

    # Create minimal example files if they don't exist
    for example in window renderer input; do
        if [ ! -f "examples/minimal/$example.rs" ]; then
            echo "Creating minimal/$example.rs..."
            echo 'fn main() { println!("Example: minimal/'$example'"); }' > "examples/minimal/$example.rs"
        fi
    done

    # Create other example files
    for category in drag_drop menu tree_view; do
        for type in basic advanced; do
            if [ ! -f "examples/$category/$type.rs" ]; then
                echo "Creating $category/$type.rs..."
                echo 'fn main() { println!("Example: '$category'/'$type'"); }' > "examples/$category/$type.rs"
            fi
        done
    done
}

# Clean and update dependencies
update_deps() {
    echo -e "${YELLOW}Updating dependencies...${NC}"
    cargo update
    cargo clean
}

# Format code
format_code() {
    echo -e "${YELLOW}Formatting code...${NC}"
    cargo fmt
}

# Build and test
build_and_test() {
    echo -e "${YELLOW}Building project...${NC}"
    cargo build --all-targets

    if [ $? -eq 0 ]; then
        echo -e "${YELLOW}Running tests...${NC}"
        cargo test
    fi
}

# Main execution
main() {
    setup_examples
    update_deps
    format_code
    build_and_test
    echo -e "${GREEN}All tasks completed successfully!${NC}"
}

# Show help
show_help() {
    echo "Usage: $0 [--help]"
    echo "Sets up example directory structure, formats code, and runs tests"
}

# Parse arguments
case "$1" in
    -h|--help)
        show_help
        exit 0
        ;;
    *)
        main
        ;;
esac
