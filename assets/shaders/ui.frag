#version 410 core
out vec4 FragColor;
in vec2 TexCoord;

uniform vec4 color;
uniform bool hasTexture;
uniform sampler2D textureSampler;

void main() {
    if (hasTexture) {
        vec4 texColor = texture(textureSampler, TexCoord);
        FragColor = texColor * color;
    } else {
        FragColor = color;
    }
} 