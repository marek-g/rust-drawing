#version 150 core

uniform mat4 transform;

in vec2 in_position;
in vec4 in_color;

out vec4 vert_color;

void main() {
    vert_color = in_color;
    gl_Position = transform * vec4(in_position, 0.0, 1.0);
}
