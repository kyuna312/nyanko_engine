#version 410 core

in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D inputTexture;
uniform float threshold;
uniform float intensity;

void main()
{
    vec4 color = texture(inputTexture, TexCoord);
    float brightness = dot(color.rgb, vec3(0.2126, 0.7152, 0.0722));
    
    if(brightness > threshold) {
        FragColor = color * intensity;
    } else {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}
