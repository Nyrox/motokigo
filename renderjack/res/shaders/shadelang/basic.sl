in Vec3 normal
in Float ux
in Float uy

Vec2 square_complex(Vec2 z){
    return Vec2(
        elem(z,0)*elem(z,0) - elem(z,1)*elem(z,1),
        elem(z,0)*elem(z,1) + elem(z,1)*elem(z,0)
    )
}

Float square_length(Vec2 a) {
    return elem(a,0)*elem(a,0) + elem(a,1)*elem(a,1)
}

Vec3 main() {
    let max_steps = 1000

    let uv = Vec2(-2.5 + (1.0 - (-2.5)) * ux, -1.0 + (1.0 - (-1.0)) * uy)
    let mut z = uv

    let mut steps = 0
    for i=0 to max_steps {
        if square_length(z) < 4.0 {
            z = square_complex(z) + uv
            steps = steps + 1
        }
    }

    if (steps == max_steps) {
        return Vec3(1.0, 0.0, 0.0)
    }

    return Vec3(float(steps) / 15.0, 0.0, 0.0)
}
