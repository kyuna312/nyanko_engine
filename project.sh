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
    if [ -d "target" ]; then
        # Use sudo only if necessary
        if ! rm -rf target/ 2>/dev/null; then
            echo -e "${YELLOW}Requesting permission to remove target directory...${NC}"
            sudo rm -rf target/
        fi
    fi

    if [ -d "examples" ]; then
        rm -rf examples/
    fi

    # Use cargo clean without removing target directory
    cargo clean
}

# Create project structure
create_structure() {
    echo -e "${YELLOW}Creating project structure...${NC}"

    # Create directories with proper permissions
    mkdir -p src/{core,graphics,input,utils,platform,math}
    mkdir -p examples/basic
    mkdir -p tests

    # Set proper permissions
    chmod -R 755 src examples tests

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

    # Set proper permissions for created files
    chmod 644 src/lib.rs
}

# Create example files
create_example_files() {
    echo -e "${YELLOW}Creating example files...${NC}"

    # Ensure the directory exists
    mkdir -p examples/basic

    # Create basic window example
    cat > examples/basic/window.rs << 'EOF'
use winit::event_loop::EventLoop;
use nyanko_engine::platform::PlatformWindow;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let _platform_window = PlatformWindow::new(window);

    println!("Basic window example initialized");
}
EOF

    # Create basic graphics example
    cat > examples/basic/graphics.rs << 'EOF'
use nyanko_engine::graphics::{Renderer, RendererConfig};
use nyanko_engine::platform::PlatformWindow;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let platform_window = PlatformWindow::new(window);
    let config = RendererConfig::default();
    let _renderer = Renderer::new(platform_window, config);

    println!("Basic graphics example initialized");
}
EOF

    # Create basic input example
    cat > examples/basic/input.rs << 'EOF'
use nyanko_engine::input::PlatformInput;

fn main() {
    let mut input = PlatformInput::new();
    input.update();

    println!("Basic input example initialized");
}
EOF

    # Set proper permissions
    chmod 644 examples/basic/*.rs
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

    # Set proper permissions
    chmod 644 tests/lib.rs
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
    # Ensure script has execute permissions
    if [ ! -x "$0" ]; then
        chmod +x "$0"
    fi

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
