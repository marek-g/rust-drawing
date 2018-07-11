#version 150 core

uniform sampler2D tex_sampler;

in vec2 vert_tex_coords;
in vec4 vert_color;

out vec4 frag_color;

void main() {
    frag_color = vec4(vert_color.rgb, vert_color.a * texture(tex_sampler, vert_tex_coords).r);
}
