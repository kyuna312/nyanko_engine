#version 410 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;
in vec3 NeonEffect;

uniform sampler2D diffuseTexture;
uniform vec3 lightPos;
uniform vec3 viewPos;
uniform float time;

void main() {
    // Sample the texture
    vec4 texColor = texture(diffuseTexture, TexCoord);
    
    // Enhance the colors slightly
    vec3 color = texColor.rgb * 1.1;
    
    // Add ambient light
    vec3 ambient = vec3(0.3, 0.2, 0.3) * color;
    
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * vec3(1.0, 0.8, 0.9) * color;
    
    // Specular lighting
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = spec * vec3(1.0, 0.8, 1.0);
    
    // Neon effect
    vec3 neon = NeonEffect * (sin(time * 3.0 + TexCoord.x * 10.0) * 0.5 + 0.5);
    
    // Combine everything
    vec3 result = ambient + diffuse + specular + neon * 0.3;
    
    // Add subtle pulsing glow
    float glow = (sin(time * 2.0) + 1.0) * 0.15;
    result += vec3(1.0, 0.4, 0.8) * glow;
    
    FragColor = vec4(result, texColor.a);
} 