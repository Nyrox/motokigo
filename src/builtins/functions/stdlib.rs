use crate::{ast::TypeKind, builtins::*, vm::VirtualMachine};
use macros::{generate_builtin_fn, generate_glsl_impl_inline};

use crate::glsl::{compiler::GenerateGLSL, BuiltInCallableGLSL};

macro_rules! implement_vec_funcs {
	( $name:ident, $s:ident<$t:ty, $n:literal> ) => {
		paste::item! {
            #[generate_builtin_fn("dot")]
            fn [<$name Dot>](a: $name, b: $name) -> $t {
                a.dot(b)
            }

            #[generate_glsl_impl_inline("{}Dot", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("dot({}, {})", a, b)
            }

            #[generate_builtin_fn("elem")]
            fn [<Get $name Elem>](a: $name, b: i32) -> f32 {
                a.get_elem(b as usize)
            }

            #[generate_glsl_impl_inline("Get{}Elem", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("{}[{}]", a, b)
            }
        }
    };
}

implement_vec_funcs!(Vec4, Vector<f32, 4>);
implement_vec_funcs!(Vec3, Vector<f32, 3>);
implement_vec_funcs!(Vec2, Vector<f32, 2>);

macro_rules! implement_func_1 {
	( $func:ident, $impl:expr, $name:ident, $ret:ident ) => {
        paste::item! {
            #[generate_builtin_fn("{}", [<$func:lower>])]
            fn [<$name $func>](a: $name) -> $ret {
                $impl
            }

            #[generate_glsl_impl_inline("{}{}", $name, $func)]
            fn generate(a: &str) -> String {
                format!(concat!(stringify!([<$func:lower>]), "({})"), a)
            }
        }
    }
}

macro_rules! implement_vec_func_1 {
    ( $func:ident, $impl:expr, $ret:ident ) => {
        implement_func_1!($func, $impl, Vec2, $ret);
        implement_func_1!($func, $impl, Vec3, $ret);
        implement_func_1!($func, $impl, Vec4, $ret);
    }
}

implement_vec_func_1!(Length, a.length(), f32);
implement_vec_func_1!(Normalize, a.length(), f32);
implement_func_1!(Abs, a.abs(), Float, Float);
