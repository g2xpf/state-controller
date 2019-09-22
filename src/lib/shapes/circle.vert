#version 400 core

in vec2 coord;
uniform vec2 pos;
uniform vec2 camera_pos;
uniform float r;

void main() {
    vec2 circle = -camera_pos + pos * r;
    gl_Position = vec4(circle + coord, 0.0, 1.0);
}
