#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 uv;

uniform mat4 mvp;

uniform vec3 pos;
uniform vec3 lastpos;

uniform float scale;




uniform float xrot;
uniform float yrot;
uniform float zrot;

uniform vec3 lastrot;


uniform float interp_time;

uniform float walkbob;
uniform float time;

uniform float buttonactive;

out vec2 TexCoord;

out float brightadd;

uniform float istitle;
uniform float isbutton;

uniform float issky;


mat4 getRotationMatrix(float xrot, float yrot, float zrot) {
    mat4 Rx = mat4(1.0, 0.0, 0.0, 0.0,
                   0.0, cos(xrot), -sin(xrot), 0.0,
                   0.0, sin(xrot), cos(xrot), 0.0,
                   0.0, 0.0, 0.0, 1.0);
                   
    mat4 Ry = mat4(cos(yrot), 0.0, sin(yrot), 0.0,
                   0.0, 1.0, 0.0, 0.0,
                   -sin(yrot), 0.0, cos(yrot), 0.0,
                   0.0, 0.0, 0.0, 1.0);
                   
    mat4 Rz = mat4(cos(zrot), -sin(zrot), 0.0, 0.0,
                   sin(zrot), cos(zrot), 0.0, 0.0,
                   0.0, 0.0, 1.0, 0.0,
                   0.0, 0.0, 0.0, 1.0);
    
    return Rz * Ry * Rx; // Note: The order might need to be adjusted based on your specific needs
}


void main() {



    TexCoord = uv;

    vec3 bob = vec3(0.0, ((sin(walkbob) )/20.0), 0.0) + vec3(0.0, 0.3, 0.0);

    vec3 adjtopos = mix(lastpos, pos, min(interp_time * 4.0, 1.0));


    vec3 mixedrots = mix(vec3(lastrot.x, -1.0 * lastrot.y, lastrot.z), vec3(xrot, -1.0 * yrot, zrot), min(interp_time * 4.0, 1.0));



    if(istitle != 0.0) {

        mixedrots = vec3(sin(time) * 0.1, yrot - sin(time)*0.02, zrot);
    }

    if(isbutton != 0.0) {
        mixedrots = vec3(xrot + sin(time)*0.5, yrot, zrot);
        // Adjust this to control how fast it decays
        float decay_speed = 2.0;

        // Exponential decay function
        adjtopos += vec3((exp(-decay_speed * time) * 3.0), 0.0, 0.0);
    }

    if(issky != 0.0) {
        mixedrots = vec3(xrot,yrot + time*0.3,zrot);
    }

    mat4 rotationMatrix = getRotationMatrix(mixedrots.x, mixedrots.y, mixedrots.z);
    vec4 rotatedPosition = rotationMatrix * vec4(aPos * scale, 1.0);



    if(buttonactive != 0.0) {
        //adjtopos.y += 0.05;
        adjtopos.y += sin(time*4.0)*0.05;
        brightadd = 0.13;
    } else {
        brightadd = 0.0;
    }
    

    
    gl_Position = mvp * (rotatedPosition + vec4(adjtopos, 0.0) + vec4(bob * -1.0, 0.0));
}