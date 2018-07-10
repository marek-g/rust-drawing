#version 150 core

uniform sampler2D tex_sampler;

in vec2 vert_tex_coords;

out vec4 frag_color;

void main() {
    frag_color = texture(tex_sampler, vert_tex_coords);
}
