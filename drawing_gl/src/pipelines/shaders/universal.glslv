#version 150 core

uniform mat4 transform;
uniform bool flipped_y;

in vec2 in_position;
in vec2 in_tex_coords;
in vec4 in_color;

out vec2 vert_tex_coords;
out vec4 vert_color;
out vec2 fpos;

void main() {
    vert_tex_coords = flipped_y ? vec2(in_tex_coords.s, 1.0 - in_tex_coords.t) : in_tex_coords;
    vert_color = in_color;
    fpos = in_position;
    gl_Position = transform * vec4(in_position, 0.0, 1.0);
}
