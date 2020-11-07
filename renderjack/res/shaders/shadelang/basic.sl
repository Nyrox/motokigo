
in Vec3 normal

Vec3 main() {
    let L = normalize(Vec3(-0.5, 1.0, -1.0))
    let C = Vec3(1.0, 0.5, 0.5)

    let cos_a = dot(L, normal)

    let mut ambient = 0.0

	if (1.0) {
		ambient = 1.0
	}


    return cos_a * C + ambient * C
}
