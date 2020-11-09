#version 330 core

in vec3 normal;
out vec3 out_0;

float foo(float a, float b) {
	return a + b;
}

vec3 __impl_main() {
	float ambient = 0.0;
	ambient = foo(0.5, 0.0);
	return vec3(ambient, ambient, ambient);
}

void main() {
	vec3 rt = __impl_main();
	out_0 = rt;
}

