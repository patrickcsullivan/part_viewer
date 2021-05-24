#version 450

layout(set=1, binding=0)
uniform Uniforms {
    mat4 u_view_proj;
};

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;

layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_position;

void main() {
    v_normal = a_normal;
    v_position = a_position;

    gl_Position = u_view_proj * vec4(a_position, 1.0);
}