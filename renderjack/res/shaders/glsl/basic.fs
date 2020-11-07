#version 330 core

in vec3 normal;
out vec3 out_0;

vec3 __impl_main() {
	vec3 L = normalize(vec3(-0.5, 1, -1));
	vec3 C = vec3(1, 0.5, 0.5);
	float cos_a = dot(L, normal);
	if (bool(1)) {
		float ambient = 0;
	}
	if (bool(1)) {
		ambient = 1;
	}
	return cos_a * C + ambient * C;
}

void main() {
	vec3 rt = __impl_main();
	out_0 = rt;
}

