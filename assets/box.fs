#version 420 core

in vec2 uv;

out vec4 color;

uniform int is_border;
uniform vec4 fill_color;
uniform vec4 border_color;

uniform int has_texture;
layout(binding = 0) uniform sampler2D fill_texture;

void main() {
    if (is_border == 1) {
        color = border_color;
    } else {
        if (has_texture == 1) {
            color = fill_color * texture(fill_texture, uv).rgba;
        } else {
            color = fill_color;
        }
    }
}
