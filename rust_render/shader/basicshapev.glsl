#version 150

in vec2 a_Pos;
//in vec3 a_Color;
out vec4 v_Color;

uniform b_Locals{
    vec4 l_Color;
};

void main() {
    v_Color = l_Color;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}