#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoord;

out vec2 TexCoord;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_projection;

void main()
{
    gl_Position = camera_projection * camera_transform * transform * vec4(aPos, 1.0f);
    TexCoord = aTexCoord;
}
