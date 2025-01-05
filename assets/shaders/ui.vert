#version 410 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform vec2 position;
uniform vec2 size;
uniform vec2 screenSize;

void main() {
    vec2 pos = position + (aPos * size);
    vec2 normalizedPos = (pos / screenSize) * 2.0 - 1.0;
    gl_Position = vec4(normalizedPos, 0.0, 1.0);
    TexCoord = aTexCoord;
} 