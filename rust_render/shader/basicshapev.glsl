#version 150

in vec2 a_Pos;
//in vec3 a_Color;
out vec4 v_Color;

layout (std140)
uniform b_Locals{
    vec4 l_Color;
    mat4 l_Pro;
    mat4 l_Mod;
    mat4 l_View;
};

void main() {
    v_Color = l_Color;
    gl_Position = l_Pro* l_View * l_Mod * vec4(a_Pos, 0.0, 1.0);
}