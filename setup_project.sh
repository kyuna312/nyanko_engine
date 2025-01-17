#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create project structure
create_structure() {
    echo -e "${YELLOW}Creating project structure...${NC}"

    # Create main source directories
    mkdir -p src/{core,graphics,input,utils}
    mkdir -p examples/basic
    mkdir -p shaders/{vertex,fragment}

    # Create basic shader files
    echo "Creating shader files..."
    create_shader_files
}

# Create shader files
create_shader_files() {
    # Vertex shader
    cat > shaders/vertex/basic.vert << 'EOF'
#version 450
layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoord;

out vec2 v_texcoord;

void main() {
    v_texcoord = texcoord;
    gl_Position = vec4(position, 1.0);
}
EOF

    # Fragment shader
    cat > shaders/fragment/basic.frag << 'EOF'
#version 450
in vec2 v_texcoord;
out vec4 color;

void main() {
    color = vec4(v_texcoord, 0.0, 1.0);
}
EOF
}

# Main execution
main() {
    create_structure
    cargo build
    echo -e "${GREEN}Project setup complete!${NC}"
}

main
