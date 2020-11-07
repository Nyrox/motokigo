in vec3 normal;
out vec3 out_0;

vec3 __impl_main() {
	vec3 L = normalize(vec3(-0.5, 1, -1));
	vec3 C = vec3(1, 0.5, 0.5);
	float cos_a = dot(L, normal);
	float ambient = 0.3;
	return cos_a * C + ambient * C;
}

void main() {
	vec3 rt = __impl_main();
	out_0 = rt;
}

