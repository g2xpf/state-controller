#version 400 core
in vec2 uv;
uniform sampler2D tex;
uniform vec3 color;
out vec4 f_color;

void main() {
    f_color = vec4(color * texture(tex, uv).r, 1.);
}
