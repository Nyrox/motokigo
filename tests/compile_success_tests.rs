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
	let a = Foo { x=1.0 }
	return a.x
}
"
);
