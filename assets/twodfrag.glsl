#version 430 core
out vec4 FragColor;
in vec2 TexCoord;


uniform float time;

uniform float deathtype; 

float rand(vec2 c){
	return fract(sin(dot(c.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

float noise(vec2 p, float freq ){
    float PI = 3.14159265358979323846;
	float unit = 0.05;
	vec2 ij = floor(p/unit);
	vec2 xy = mod(p,unit)/unit;
	//xy = 3.*xy*xy-2.*xy*xy*xy;
	xy = .5*(1.-cos(PI*xy));
	float a = rand((ij+vec2(0.,0.)));
	float b = rand((ij+vec2(1.,0.)));
	float c = rand((ij+vec2(0.,1.)));
	float d = rand((ij+vec2(1.,1.)));
	float x1 = mix(a, b, xy.x);
	float x2 = mix(c, d, xy.x);
	return mix(x1, x2, xy.y);
}

float pNoise(vec2 p, int res){
	float persistance = .5;
	float n = 0.;
	float normK = 0.;
	float f = 4.;
	float amp = 1.;
	int iCount = 0;
	for (int i = 0; i<50; i++){
		n+=amp*noise(p, f);
		f*=2.;
		normK+=amp;
		amp*=persistance;
		if (iCount == res) break;
		iCount++;
	}
	float nf = n/normK;
	return nf*nf*nf*nf;
}

void main() {
    
    

	if(deathtype == 0.0) {
		float noisevalue = pNoise((TexCoord * 0.3) + vec2(time * 100.0, time * 100.0), 1);
		FragColor = vec4(1.0, 1.0, 1.0, noisevalue);
	}

    if(deathtype == 1.0) {

		vec3 colors[4] = vec3[4](
			vec3(1.0, 0.0, 0.0),
			vec3(0.0, 1.0, 0.0),
			
			vec3(0.0, 0.0, 1.0),
			vec3(1.0, 1.0, 0.0)
		);

		float noisevalue = pNoise((TexCoord * 0.2) + vec2(time * 20.0, time * 20.0), 1);

		int colorIndex = int(time) % 4;
		
		FragColor = vec4(colors[colorIndex], noisevalue);
	}
}