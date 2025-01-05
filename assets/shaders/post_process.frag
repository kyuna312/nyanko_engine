#version 410 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D screenTexture;
uniform sampler2D bloomTexture;
uniform float time;
uniform bool enableBloom;
uniform bool enableChromatic;
uniform bool enableVignette;
uniform float bloomIntensity;
uniform float chromaticStrength;
uniform float vignetteIntensity;
uniform float vignetteRoundness;
uniform float vignetteSmoothness;

vec3 applyBloom(vec3 color, vec2 uv) {
    vec3 bloomColor = texture(bloomTexture, uv).rgb;
    return color + bloomColor * bloomIntensity;
}

vec3 applyChromaticAberration(vec2 uv) {
    vec2 direction = normalize(uv - 0.5);
    vec3 color;
    color.r = texture(screenTexture, uv - direction * chromaticStrength).r;
    color.g = texture(screenTexture, uv).g;
    color.b = texture(screenTexture, uv + direction * chromaticStrength).b;
    return color;
}

float applyVignette(vec2 uv) {
    vec2 coord = (uv - 0.5) * 2.0;
    float len = length(coord);
    float vignette = smoothstep(0.8, 0.2, len * vignetteIntensity);
    return pow(vignette, vignetteRoundness);
}

void main() {
    vec2 uv = TexCoord;
    vec3 color;

    if (enableChromatic) {
        color = applyChromaticAberration(uv);
    } else {
        color = texture(screenTexture, uv).rgb;
    }

    if (enableBloom) {
        color = applyBloom(color, uv);
    }

    if (enableVignette) {
        color *= applyVignette(uv);
    }

    // Add subtle color grading
    color = pow(color, vec3(1.0 / 2.2)); // Gamma correction
    color *= 1.1; // Brightness boost
    color = mix(color, color * vec3(1.1, 1.0, 1.2), 0.2); // Subtle tint

    FragColor = vec4(color, 1.0);
} 