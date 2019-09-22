#version 400 core

in vec2 coord;

uniform vec2 pos;
uniform float width;
uniform float height;
uniform float angle;
uniform vec2 camera_pos;

vec2 rotate(vec2 v) {
    float c = cos(angle);
    float s = sin(angle);
    return mat2(c, -s, s, c) * v;
}

void main() {
    vec2 v_coord = coord * vec2(width, height);
    v_coord = rotate(v_coord) + pos - camera_pos;
    
    gl_Position = vec4(v_coord, 0.0, 1.0);
}
