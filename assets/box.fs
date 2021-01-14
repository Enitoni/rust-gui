#version 410 core

in vec2 uv;

out vec4 color;

uniform int is_border;

void main() {
    if (is_border == 1) {
        color = vec4(0.2, 0.2, 1.0, 0.5);
    } else {
        color = vec4(1.0, 1.0, 1.0, 0.04);
    }
}
