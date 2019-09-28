#version 400 core
in vec2 coord;
in vec2 tex_coord;
out vec2 uv;

void main() {
    uv = tex_coord;
    gl_Position = vec4(coord, 0., 1.);
}
