#version 430

in vec2 fragTexCoord;

layout(binding = 1) buffer FFT_buffer {
    float fft_buffer[];
};

layout(binding = 2) buffer ColorBuffer {
    float colors[]; // stored as a float array of [r1, g1, b1, r2, g2, b2 ...]
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

float alpha() {
    float sc = float(sample_count);
    int subline_idx = int(sc * fragTexCoord.x);
    vec2 p = vec2(fragTexCoord.x * Tex_dim.x, (1 - fragTexCoord.y) * Tex_dim.y);

    float line_pos_x = Tex_dim.x * (subline_idx + 0.5) / sc;
    float frag_y = (1 - fragTexCoord.y);
    float wf_sample = gain * fft_buffer[subline_idx];

    float dist_from_line = abs(line_pos_x - p.x);
    bool frag_within_range = 0 < frag_y / wf_sample && frag_y / wf_sample < 1;

    if (dist_from_line < line_width / 2 && frag_within_range) {
        return 1.0;
    } else if (frag_within_range) {
        float sigma = 4.0;
        float line_width = 1.5;
        if (should_glow != 1) {
            sigma = 0.0;
        }
        float x = dist_from_line;
        float glow_factor = 0.2 * exp(-(x * x) / (2 * pow(sigma, 2)));

        return clamp(glow_factor, 0, 1.0);
    } else {
        return 0.0;
    }
}

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
    if (color_count == 1) {
        int first_color = 4 * color_offset;
        return vec3(colors[first_color], colors[first_color + 1], colors[first_color + 2]);
    }
    float fade_count = float(color_count - 1);
    int color_index = int(fade_count * fragTexCoord.x);
    float true_t = fade_count * fragTexCoord.x - float(color_index);
    color_index = color_index + color_offset;
    return fade_rgb(color_index, color_index + 1, true_t);
}

void main() {
    float alpha = alpha();
    vec3 color = compute_color();
    finalColor = vec4(color, alpha);
}
