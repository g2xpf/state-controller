#version 430

in vec2 texUV;

uniform sampler2D tex;

out vec4 color;

void main(){
    color = texture(tex, texUV);
}

