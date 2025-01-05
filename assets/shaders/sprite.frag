#version 410 core

in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D textureSampler;

void main()
{
    // Sample texture
    vec4 color = texture(textureSampler, TexCoord);
    
    // Enhance colors for anime aesthetic
    vec3 enhanced = color.rgb;
    
    // Improve contrast
    enhanced = pow(enhanced, vec3(0.95));
    
    // Enhance saturation
    float luminance = dot(enhanced, vec3(0.299, 0.587, 0.114));
    enhanced = mix(vec3(luminance), enhanced, 1.2);
    
    // Add subtle color tinting for anime style
    enhanced *= vec3(1.02, 1.0, 1.02);
    
    // Output with original alpha
    FragColor = vec4(enhanced, color.a);
} 