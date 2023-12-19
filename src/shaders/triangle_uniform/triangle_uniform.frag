#version 330 core
out vec4 FragColor;
  
uniform float colorScale;

void main()
{
    FragColor = vec4(1.0f * colorScale, 0.5f * colorScale, 0.2f * colorScale, 1.0f);
}
