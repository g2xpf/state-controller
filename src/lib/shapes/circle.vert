#version 430

in vec2 coord;
uniform vec2 pos;
uniform float r;

void main() {
    vec2 circle = pos * r;
    gl_Position = vec4(circle + coord, 0.0, 1.0);
}
