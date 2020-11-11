#version 330 core

in vec3 normal;
in float ux;
in float uy;

struct Col3 {
	float r;
	float g;
	float b;
};

out vec3 out_0;

Col3 rot(Col3 c) {
	return Col3(c.b, c.r, c.g);
}

vec3 __impl_main() {
	Col3 cout = Col3(1.0, 1.0, 0.5);
	cout = rot(cout);
	return vec3(cout.r, cout.g, cout.b);
}

void main() {
	vec3 rt = __impl_main();
	out_0 = rt;
}

