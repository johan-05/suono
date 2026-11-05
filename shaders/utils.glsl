#version 430

// This is kind of a shell for a fragment shader
// it contains usefull concepts that can be modifed

in vec2 fragTexCoord;
out vec4 finalColor;

layout(binding = 0) buffer AudioBuffer {
    float audio_data[];
};

layout(binding = 1) buffer ColorBuffer {
    float colors[]; // stored as a float array of [r1, g1, b1, r2, g2, b2 ...]
};

uniform vec2 Tex_dim;
uniform int Should_glow;
uniform int Color_offset;
uniform int Color_count;

vec3 fade_rgb(int from_idx_i, int to_idx_i, float t_0) {
    float t = clamp(t_0 * t_0 * (3.0 - 2.0 * t_0), 0.0, 1.0);
    int from_idx = 4 * from_idx_i;
    int to_idx = 4 * to_idx_i;
    float r = colors[from_idx + 0] + t * (colors[to_idx + 0] - colors[from_idx + 0]);
    float g = colors[from_idx + 1] + t * (colors[to_idx + 1] - colors[from_idx + 1]);
    float b = colors[from_idx + 2] + t * (colors[to_idx + 2] - colors[from_idx + 2]);
    return vec3(r, g, b);
}

vec3 compute_color() {
    if (Color_count == 1) {
        int first_color = 4 * Color_offset;
        return vec3(colors[first_color], colors[first_color + 1], colors[first_color + 2]);
    }
    float fade_count = float(Color_count - 1);
    int color_index = int(fade_count * fragTexCoord.x);
    float true_t = fade_count * fragTexCoord.x - float(color_index);
    color_index = color_index + Color_offset;
    return fade_rgb(color_index, color_index + 1, true_t);
}
