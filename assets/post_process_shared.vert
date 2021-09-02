#version 330 core
precision lowp float;

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoord;
layout(location = 2) in vec4 color0;

out vec2 uv;
out vec4 color;
uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * (Model * (vec4(position, 1)));
    color = color0 / 255.0;
    uv = texcoord;
}