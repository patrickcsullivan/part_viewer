#version 450

layout(location=0) in vec3 a_position;

// const vec2 positions[3] = vec2[3](
//     vec2(0.0, 0.5),
//     vec2(-0.5, -0.5),
//     vec2(0.5, -0.5)
// );

void main() {
    gl_Position = vec4(a_position, 1.0);
}