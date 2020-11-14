use crate::{ast::TypeKind, builtins::*, vm::VirtualMachine};
use macros::{generate_builtin_fn, generate_glsl_impl_inline};
use macros::bingbong;
use crate::glsl::{compiler::GenerateGLSL, BuiltInCallableGLSL};

macro_rules! implement_vec_funcs {
    ( $name:ident ) => {
        bingbong!(Elem, elem, |a: $name, b: Int| -> Float { a.get_elem(b as usize) }, "{}[{}]");
        bingbong!(Length, length, |a: $name| -> Float { a.length() }, "length({})");
        bingbong!(Normalize, normalize, |a: $name| -> $name { a.normalize() }, "normalize({})");
        bingbong!(Dot, dot, |a: $name, b: $name| -> Float { a.dot(b) }, "dot({}, {})");
        bingbong!(Distance, distance, |a: $name, b: $name| -> Float { (a - b).length() }, "distance({}, {})");
    }
}
implement_vec_funcs!(Vec2);
implement_vec_funcs!(Vec3);
implement_vec_funcs!(Vec4);

macro_rules! implement_common_num_funcs {
    ( $name:ident ) => {
        bingbong!(Abs, abs, |a: $name| -> $name { a.abs() }, "abs({})");
        bingbong!(Sign, sign, |a: $name| -> $name { a.signum() }, "sign({})");
    }
}
implement_common_num_funcs!(Float);
implement_common_num_funcs!(Int);

macro_rules! implement_float_func { 
    ( $func:ident, $impl:expr, $glimpl:literal, 1) => {
        paste::item! {
            bingbong!($func, [<$func:lower>], |a: Float| -> Float { $impl(a) }, $glimpl);
        }
    };
    ( $func:ident, $impl:expr, $glimpl:literal, 2) => {
        paste::item! {
            bingbong!($func, [<$func:lower>], |a: Float, b: Float| -> Float { $impl(a, b) }, $glimpl);
        }
    };
}

implement_float_func!(Sin, f32::sin, "sin({})", 1);
implement_float_func!(Cos, f32::cos, "cos({})", 1);
implement_float_func!(Tan, f32::tan, "tan({})", 1);
implement_float_func!(Asin, f32::asin, "asin({})", 1);
implement_float_func!(Acos, f32::acos, "acos({})", 1);
implement_float_func!(Atan, f32::atan, "atan({})", 1);
implement_float_func!(Radians, f32::to_radians, "radians({})", 1);
implement_float_func!(Degrees, f32::to_degrees, "degrees({})", 1);
implement_float_func!(Exp, f32::exp, "exp({})", 1);
implement_float_func!(Log, f32::ln, "log({})", 1);
implement_float_func!(Exp2, f32::exp2, "exp2({})", 1);
implement_float_func!(Log2, f32::log2, "log2({})", 1);
implement_float_func!(Sqrt, f32::sqrt, "sqrt({})", 1);
implement_float_func!(Floor, f32::floor, "floor({})", 1);
implement_float_func!(Ceil, f32::ceil, "ceil({})", 1);
implement_float_func!(Fract, f32::fract, "fract({})", 1);
implement_float_func!(Atan2, f32::atan2, "atan2({}, {})", 2);
implement_float_func!(Pow, f32::pow, "pow({}, {})", 2);
implement_float_func!(Logn, f32::log, "logn({}, {})", 2);
implement_float_func!(Min, f32::min, "min({}, {})", 2);
implement_float_func!(Max, f32::max, "max({}, {})", 2);