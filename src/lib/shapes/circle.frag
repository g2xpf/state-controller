#version 400 core

uniform vec2 pos;
uniform vec2 camera_pos;
uniform vec3 color;
uniform float r;
uniform ivec2 iResolution;

out vec4 f_color;

float distFunc(vec2 p) {
    return length(p - pos + camera_pos) - r;
}

void main() {
    vec2 uv = (gl_FragCoord.xy * 2.0 - iResolution.xy) / min(iResolution.x, iResolution.y);
    if (distFunc(uv) < 0.) {
        f_color = vec4(color, 1.);
    } else {
        discard;
    }
}
