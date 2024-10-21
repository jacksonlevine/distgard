#version 330 core
out vec4 FragColor;


uniform float special;
void main()
{
    if(special == 0.0) {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
    
}