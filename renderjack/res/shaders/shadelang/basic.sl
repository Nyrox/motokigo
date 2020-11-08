
in Vec3 normal

Vec3 main() {
    let mut ambient = 0.5

	for i=0 to 5 {
		ambient = ambient + 0.1
	}

    return Vec3(ambient, ambient, ambient)
}
