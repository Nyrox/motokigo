#version 330 core

in vec3 normal;
in float ux;
in float uy;
out vec3 out_0;

vec2 square_complex(vec2 z) {
	return vec2(z[0] * z[0] - z[1] * z[1], z[0] * z[1] + z[1] * z[0]);
}

float square_length(vec2 a) {
	return a[0] * a[0] + a[1] * a[1];
}

vec3 __impl_main() {
	int max_steps = 1000;
	vec2 uv = vec2(-2.5 + (1.0 - (-2.5)) * ux, -1.0 + (1.0 - (-1.0)) * uy);
	vec2 z = uv;
	int steps = 0;
	for (int i=0; i < max_steps; i++) {
		if (bool(square_length(z) < 4.0)) {
			z = square_complex(z) + uv;
			steps = steps + 1;
		}
	}
	if (bool((steps == max_steps))) {
		return vec3(1.0, 0.0, 0.0);
	}
	return vec3(float(steps) / 15.0, 0.0, 0.0);
}

void main() {
	vec3 rt = __impl_main();
	out_0 = rt;
}

