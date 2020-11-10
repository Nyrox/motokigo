use motokigo::{compiler, glsl, parser};

macro_rules! should_fail_compilation {
	($sn: ident, $source: expr) => {
		paste::item! {
			#[test]
			#[should_panic]
			pub fn [<fail_compile_ $sn>]() {
				let mut program = parser::parse($source).unwrap();
                compiler::resolve_types::resolve(&mut program, &mut compiler::program_data::ProgramData::new()).unwrap();

                // check that glsl compiler atleast works
                glsl::generate_glsl(program.clone());

                compiler::compile(program);
			}
		}
	}
}

should_fail_compilation!(
    wrong_return_type,
    r"
Float main() {
	return 5
}"
);

should_fail_compilation!(
    wrong_param_types,
    r"
Float foo(Float a) {
	return a
}

Float main() {
	return foo(5)
}
"
);

should_fail_compilation!(assignment_to_immutable, r"
Float main() {
	let a = 0.0
	a = 1.0
	return 1.0
}");

should_fail_compilation!(assignment_type_mismatch, r"
Float main() {
	let mut a = 1
	a = 1.0
	return 1.0
}
");
