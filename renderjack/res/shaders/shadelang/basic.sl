
in Vec3 normal

Float foo(Float a, Float b) {
    return a + b
}

Vec3 main() {
    let mut ambient = 0.0

    ambient = foo(0.5, 0.0)

    return Vec3(ambient, ambient, ambient)
}
