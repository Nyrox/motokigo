#version 330 core

in vec3 normal;
out vec3 out_0;

vec3 __impl_main() {
	float ambient = 0.5;
	for (int i=0; i < 5; i++) {
		ambient = ambient + 0.1;
	}
	return vec3(ambient, ambient, ambient);
}

void main() {
	vec3 rt = __impl_main();
	out_0 = rt;
}

