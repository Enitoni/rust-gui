#version 410 core

in vec2 uv;

out vec4 color;

uniform int is_border;
uniform vec4 fill_color;
uniform vec4 border_color;

void main() {
    if (is_border == 1) {
        color = border_color;
    } else {
        color = fill_color;
    }
}
