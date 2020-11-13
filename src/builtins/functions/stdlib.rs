use crate::{ast::TypeKind, builtins::*, vm::VirtualMachine};
use macros::{generate_builtin_fn, generate_glsl_impl_inline};

use crate::glsl::{compiler::GenerateGLSL, BuiltInCallableGLSL};

macro_rules! implement_vec_func {
    ( $func:ident, $impl:expr, $ret:ident, 1 ) => {
        implement_func!($func, $impl, Vec2, $ret);
        implement_func!($func, $impl, Vec3, $ret);
        implement_func!($func, $impl, Vec4, $ret);
    };
    ( $func:ident, $impl:expr, $ret:ident , 2) => {
        implement_func!($func, $impl, Vec2, Vec2, $ret);
        implement_func!($func, $impl, Vec3, Vec3, $ret);
        implement_func!($func, $impl, Vec4, Vec4, $ret);
    };
}

macro_rules! implement_float_func { 
    ( $func:ident, $impl:expr, 1) => {
        implement_func!($func, $impl, Float, Float);
    };
    ( $func:ident, $impl:expr, 2) => {
        implement_func!($func, $impl, Float, Float, Float);
    };
}

macro_rules! implement_num_func {
    ( $func:ident, $impl:expr, 1 ) => {
        implement_float_func!($func, $impl, 1);
        implement_func!($func, $impl, Int, Int);
    };
    ( $func:ident, $impl:expr, 2 ) => {
        implement_float_func!($func, $impl, 2);
        implement_func!($func, $impl, Int, Int, Int);
    };
}

implement_func!(Elem, elem, a.get_elem(b as usize), "{}[{}]", Vec2, Int, Float);
implement_func!(Elem, elem, a.get_elem(b as usize), "{}[{}]", Vec3, Int, Float);
implement_func!(Elem, elem, a.get_elem(b as usize), "{}[{}]", Vec4, Int, Float);

implement_vec_func!(Length, a.length(), Float, 1);
implement_vec_func!(Normalize, a.length(), Float, 1);
implement_vec_func!(Dot, a.dot(b), Float, 2);
implement_vec_func!(Distance, (a - b).length(), Float, 2);

implement_num_func!(Abs, a.abs(), 1);
implement_num_func!(Sign, a.signum(), 1);
implement_num_func!(Mod, a % b, 2);

implement_float_func!(Sin, a.sin(), 1);
implement_float_func!(Cos, a.cos(), 1);
implement_float_func!(Tan, a.tan(), 1);
implement_float_func!(Asin, a.asin(), 1);
implement_float_func!(Acos, a.acos(), 1);
implement_float_func!(Atan, a.atan(), 1);
implement_float_func!(Radians, a.to_radians(), 1);
implement_float_func!(Degrees, a.to_degrees(), 1);
implement_float_func!(Exp, a.exp(), 1);
implement_float_func!(Log, a.ln(), 1);
implement_float_func!(Exp2, a.exp2(), 1);
implement_float_func!(Log2, a.log2(), 1);
implement_float_func!(Sqrt, a.sqrt(), 1);
implement_float_func!(Floor, a.floor(), 1);
implement_float_func!(Ceil, a.ceil(), 1);
implement_float_func!(Fract, a.fract(), 1);
implement_float_func!(Atan2, a.atan2(b), 2);
implement_float_func!(Pow, a.pow(b), 2);
implement_float_func!(Logn, a.log(b), 2);
implement_float_func!(Min, a.min(b), 2);
implement_float_func!(Max, a.max(b), 2);



