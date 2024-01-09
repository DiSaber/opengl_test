#version 330 core
out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D texture0;
uniform vec4 text_color;

void main()
{
    FragColor = texture(texture0, TexCoord) * text_color;
}
