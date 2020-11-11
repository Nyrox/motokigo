in Vec3 normal
in Float ux
in Float uy


struct Col3 {
	Float r,
	Float g,
	Float b,
}

Col3 rot(Col3 c) {
	return Col3 { r=c.b, g=c.r, b=c.g }
}

Vec3 main() {
	let mut cout = Col3 { r=1.0, b=0.5, g=1.0 }
	cout = rot(cout)

	return Vec3(cout.r, cout.g, cout.b)
}
