#version 400 core

in vec2 coord;

uniform vec2 pos;
uniform float width;
uniform float height;
uniform float angle;
uniform vec2 camera_pos;

out vec2 texUV;

vec2 rotate(vec2 v){
    float c = cos(angle);
    float s = sin(angle);
    return mat2(c, -s, s, c) * v;
}

void main(){
    gl_Position = vec4(-camera_pos + pos + rotate(vec2(width, height) * coord), 0.0, 1.0);
    texUV = (coord + 1.0) / 2.0;
}
