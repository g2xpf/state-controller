#version 430

in vec2 coord;

uniform vec2 pos;
uniform float width;
uniform float height;
uniform float angle;

vec2 rotate(vec2 v) {
    float c = cos(angle);
    float s = sin(angle);
    return v * mat2(c, -s, s, c);
}

void main() {
    vec2 v_coord = coord * vec2(width, height);
    v_coord = rotate(v_coord) + pos;
    
    gl_Position = vec4(v_coord, 0.0, 1.0);
}
