#version 430

in vec2 fragTexCoord;

layout(binding = 1) buffer FFT_buffer {
    float fft_buffer[];
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

float glow() {
    float sc = float(sample_count);
    int subline_idx = int(floor(sc * fragTexCoord[0]));
    vec2 p = vec2(fragTexCoord.x * Tex_dim[0], (1 - fragTexCoord.y) * Tex_dim[1]);

    float lowest_distance = 676767.0;
    for (int i = subline_idx - 5; i < subline_idx + 5 && i < sc - 1; ++i) {
        vec2 a = vec2(i * Tex_dim[0] / sc, gain * fft_buffer[i] * Tex_dim[1]);
        vec2 b = vec2((i + 1) * Tex_dim[0] / sc, gain * fft_buffer[i + 1] * Tex_dim[1]);

        vec2 ab = b - a;
        vec2 ap = p - a;
        vec2 bp = p - b;

        float t = dot(ab, ap) / dot(ab, ab);
        if (0 < t && t < 1) {
            float distance = length(p - a - t * ab);
            if (distance < lowest_distance) {
                lowest_distance = distance;
            }
        } else {
            float min_dist = min(length(ap), length(bp));
            if (min_dist < lowest_distance) {
                lowest_distance = min_dist;
            }
        }
    }

    if (lowest_distance <= line_width) {
        return 1.0;
    }

    //  Gaussian blur
    float sigma = 12.0;
    if (should_glow != 1) {
        sigma = 0.0;
    }

    float x = lowest_distance - line_width;
    float glow_factor = 0.2 * exp(-(x * x) / (2 * sigma * sigma));

    return clamp(glow_factor, 0, 1.0);
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
    float glow_factor = glow();
    vec3 color = compute_color();
    finalColor = vec4(color, glow_factor);
}
