#version 410 core
out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

uniform Material material;
uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 lightColor;

void main() {
    // Ambient
    vec3 ambient = 0.2 * lightColor * texture(material.diffuse, TexCoord).rgb;

    // Diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor * texture(material.diffuse, TexCoord).rgb;

    // Specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = spec * lightColor * texture(material.specular, TexCoord).rgb;

    // Rim lighting for anime style
    float rim = 1.0 - max(dot(viewDir, norm), 0.0);
    rim = smoothstep(0.6, 1.0, rim);
    vec3 rimLight = rim * lightColor;

    vec3 result = ambient + diffuse + specular + rimLight;
    FragColor = vec4(result, 1.0);
} 