#version 400 core

in vec2 coord;
in vec2 tex_coord;

uniform vec2 camera_pos;
uniform vec2 pos;
uniform float theta;

out vec2 v_coord;

vec2 rotate(vec2 p) {
    float c = cos(theta);
    float s = sin(theta);
    return vec2(c * p.x - s * p.y, s * p.x + c * p.y);
}

void main() {
    v_coord = tex_coord;
    gl_Position = vec4(pos + rotate(coord) - camera_pos, 0.0, 1.0);
}
