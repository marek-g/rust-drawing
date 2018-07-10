#version 150 core

uniform sampler2D tex_sampler;

in vec2 vert_tex_coords;
in vec4 vert_color;

out vec4 frag_color;

void main() {
    frag_color = vert_color * vec4(1.0, 1.0, 1.0, texture(tex_sampler, vert_tex_coords).r);
}
