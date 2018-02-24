#version 150 core

in vec2 a_Pos;
in vec4 a_Color;
out vec4 v_Color;

layout (std140)
uniform Locals {
	mat4 u_Transform;
};

void main() {
    v_Color = a_Color;
    gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
}