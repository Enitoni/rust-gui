#version 410 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 _uv;

out vec2 uv;

uniform float fb_width;
uniform float fb_height;

void main() {
    vec2 space_pos = position / vec2(fb_width, fb_height);
    vec2 clip_pos = (space_pos - 0.5) * 2.0;
    clip_pos.y *= -1;
    gl_Position = vec4(clip_pos, 1.0, 1.0);
    uv = _uv;
}