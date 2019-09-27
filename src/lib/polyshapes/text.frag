#version 400 core

const float EPS = 1e-3;

uniform sampler2D tex;
uniform vec3 color;

in vec2 v_coord;

out vec4 f_color;

void main() {
    float t = texture(tex, v_coord).r;
    if(t > EPS){
        f_color = vec4(color * t, 1.);
    } else {
        discard;
    }

}
