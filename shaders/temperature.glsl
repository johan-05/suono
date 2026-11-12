#version 430

in vec2 fragTexCoord;

layout(binding = 4) buffer FFT_history {
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
uniform int fft_offset;

out vec4 finalColor;

float temperature() {
    int freq_len = int(sample_count) / 2;
    int freq_idx = int(freq_len * (1 - fragTexCoord.y));
    float frag_pos = freq_len * (1 - fragTexCoord.y) - freq_idx;
    int time_idx = int(timeline_length * fragTexCoord.x) + fft_offset;
    if (time_idx >= int(timeline_length)) {
        time_idx -= int(timeline_length);
    }
    float fft_val = gain * fft_history[freq_len * time_idx + freq_idx];
    float next_fft_val = gain * fft_history[freq_len * time_idx + freq_idx + 1];
    fft_val = fft_val + (next_fft_val - fft_val) * frag_pos;
    if (fft_val < 0.2) {
        fft_val = 0;
    }
    return clamp(fft_val, 0.0, 1.0);
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

vec3 compute_color(float temperature) {
    if (color_count == 1) {
        int first_color = 4 * color_offset;
        return vec3(colors[first_color], colors[first_color + 1], colors[first_color + 2]);
    }
    float fade_count = float(color_count - 1);
    int color_index = int(fade_count * temperature);
    float true_t = fade_count * temperature - float(color_index);
    color_index = color_index + color_offset;
    return fade_rgb(color_index, color_index + 1, true_t);
}

void main() {
    float temperature = temperature();
    vec3 color = compute_color(temperature);
    finalColor = vec4(color, 1.2 * temperature);
}
