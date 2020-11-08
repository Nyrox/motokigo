in Float ux
in Float uy

Vec3 main() {
    let mut z = 0.5

    for i=0 to 10 {
        if ux * i > 1.0 {
            z = z + 0.1
        } else {
            z = z - 0.1
        }
    }

    return Vec3(ux, uy, z)
}
