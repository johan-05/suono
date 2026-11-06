#version 430

in vec2 fragTexCoord;

layout(binding = 3) buffer FFT_history {
    float fft_history[];
};

layout(binding = 2) buffer ColorBuffer {
    float colors[]; // stored as a float array of [r1, g1, b1, a1, r2, g2, b2, a2 ...]
};

uniform vec2 Tex_dim;
uniform int should_glow;
uniform int color_offset;
uniform int color_count;
uniform uint sample_count;
uniform uint timeline_length;
uniform float line_width;
uniform float gain;

out vec4 finalColor;

void main() {
    float glow_factor = glow();
    vec3 color = compute_color();
    finalColor = vec4(color, glow_factor);
}
