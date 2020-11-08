#version 330 core

in float ux;
in float uy;
out vec3 out_0;

vec3 __impl_main() {
	float u = -2.5 + 1 - -2.5 * ux;
	float v = -1 + 1 - -1 * uy;
	float zu = u;
	float zv = v;
	int steps = 0;
	for (int i=0; i < 10; i++) {
		if (bool(zu * zu + zv * zv < 4)) {
			float nzu = zu * zu - zv * zv + u;
			float nzv = 2 * zu * zv + v;
			zu = nzu;
			zv = nzv;
			steps = steps + 1;
		}
	}
	if (bool(steps == 10)) {
		return vec3(1, 0, 0);
	}
	return vec3(float(steps) / 15, 0, 0);
}

void main() {
	vec3 rt = __impl_main();
	out_0 = rt;
}

