




// "
// in Float a;
// in Float b;

// Float main() {
//     return a + b
// }
// "

const TEST_ITERATIONS: usize = 50;

use motokigo::{parser, compiler, glsl, vm::VMState};

macro_rules! generate_basic_op_test {
    ($name: ident, $tl: expr, $tr: ty, $op: expr, $opr:expr, $epsilon: expr) => {
        paste::item! {
            #[test]
            pub fn [<test_basic_op_ $name>]() {
                let test_source = format!(r"
                    in {t} a
                    in {t} b

                    {t} main() {{
                        return a {o} b
                    }}
                ", t=$tl, o=$op);

                let mut program = parser::parse(test_source).unwrap();
                compiler::resolve_types::resolve(&mut program, &mut compiler::program_data::ProgramData::new()).unwrap();

                // check that glsl compiler atleast works
                glsl::generate_glsl(program.clone());

                let program = compiler::compile(program);

                let mut rng = rand::thread_rng();
                for i in 0..TEST_ITERATIONS {
                    let mut vm = motokigo::vm::VirtualMachine::new(&program);
                    let a = rand::random::<$tr>();
                    let b = rand::random::<$tr>();

                    vm.set_global("a", a);
                    vm.set_global("b", b);

                    if let VMState::VMRunFinished(mut s) = vm.run_fn("main", vec![]) {
                        let r: $tr = unsafe { s.0.pop_stack() };
                        let expected: $tr = $opr(a, b);

                        assert!((expected - r).abs() <= $epsilon);
                    } else{
                        panic!("Encountered a breakpoint in a test. Cursed.");
                    }
                }
            }
        }
    }
}

const EPSILON_F32: f32 = 0.000001;
const EPSILON_I32: i32 = 0;

generate_basic_op_test!(add_float, "Float", f32, "+", std::ops::Add::add, EPSILON_F32);
generate_basic_op_test!(sub_float, "Float", f32, "-", std::ops::Sub::sub, EPSILON_F32);
generate_basic_op_test!(mul_float, "Float", f32, "*", std::ops::Mul::mul, EPSILON_F32);
generate_basic_op_test!(div_float, "Float", f32, "/", std::ops::Div::div, EPSILON_F32);

generate_basic_op_test!(add_int, "Int", i32, "+", i32::wrapping_add, EPSILON_I32);
generate_basic_op_test!(sub_int, "Int", i32, "-", i32::wrapping_sub, EPSILON_I32);
generate_basic_op_test!(mul_int, "Int", i32, "*", i32::wrapping_mul, EPSILON_I32);
generate_basic_op_test!(div_int, "Int", i32, "/", i32::wrapping_div, EPSILON_I32);