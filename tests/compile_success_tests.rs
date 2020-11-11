use motokigo::{compiler, glsl, parser};

macro_rules! should_pass_compilation {
	($sn: ident, $source: expr) => {
		paste::item! {
			#[test]
			pub fn [<pass_compile_ $sn>]() {
				let mut program = parser::parse($source).unwrap();
				compiler::resolve_types::resolve(&mut program, &mut compiler::program_data::ProgramData::new()).unwrap();

				// check that glsl compiler atleast works
				glsl::generate_glsl(program.clone());

				compiler::compile(program);
			}
		}
	};
}

should_pass_compilation!(
	if_statements,
	r"
Float main(){
	if 1 {
		return 2.0
	} else if 0 {
		return 5.0
	} else {
		return 1.0
	}
}
"
);

should_pass_compilation!(
	for_loop,
	r"
Float main() {
	let mut a = 0.0
	for i=0 to 10 {
		a = a + 10.0 / float(i)
	}
	return a
}
"
);

should_pass_compilation!(
	struct_declaration,
	r"
struct Foo {
	Float x,
}

Float main() {
	return 1.0
}
"
);

should_pass_compilation!(
	struct_construction,
	r"
struct Foo {
	Float x
}

Float main() {
	let a = Foo { x: 1.0 }
	return a.x
}
"
);


should_pass_compilation!(mandelbrot, r"
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
    let max_steps = 5

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
");