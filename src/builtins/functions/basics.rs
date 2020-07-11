use crate::ast::TypeKind;
use crate::builtins::*;
use crate::vm::VirtualMachine;
use builtins::generate_builtin_fn;
use builtins::generate_glsl_impl_inline;

use crate::glsl::compiler::GenerateGLSL;
use crate::glsl::BuiltInCallableGLSL;

macro_rules! implement_vec_op {
    ( $name:ident, $s:ident<$t:ty, $n:literal> ) => {
        paste::item! {

            #[generate_builtin_fn("__op_binary_mul")]
            fn [<BinMul $name Float>](v: $name, a: $t) -> $name {
                v * a
            }

            #[generate_glsl_impl_inline("BinMul{}Float", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("{} * {}", a, b)
            }

            #[generate_builtin_fn("__op_binary_mul")]
            fn [<BinMulFloat $name>](a: $t, v: $name) -> $name {
                v * a
            }

            #[generate_glsl_impl_inline("BinMulFloat{}", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("{} * {}", a, b)
            }

            #[generate_builtin_fn("__op_binary_add")]
            fn [<BinAdd $name $name>](a: $name, b: $name) -> $name {
                a + b
            }

            #[generate_glsl_impl_inline("BinAdd{}{}", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("{} + {}", a, b)
            }

            #[generate_builtin_fn("normalize")]
            fn [<$name Normalize>](a: $name) -> $name {
                a.normalize()
            }

            #[generate_glsl_impl_inline("{}Normalize", $name)]
            fn generate(a: &str) -> String {
                format!("normalize({})", a)
            }

            #[generate_builtin_fn("dot")]
            fn [<$name Dot>](a: $name, b: $name) -> $t {
                a.dot(b)
            }

            #[generate_glsl_impl_inline("{}Dot", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("dot({}, {})", a, b)
            }
        }
    };
}

implement_vec_op!(Vec4, Vector<f32, 4>);
implement_vec_op!(Vec3, Vector<f32, 3>);
implement_vec_op!(Vec2, Vector<f32, 2>);

#[generate_builtin_fn("Vec2")]
fn Vec2Constructor(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

#[generate_glsl_impl_inline("Vec2Constructor")]
fn generate(a: &str, b: &str) -> String {
    format!("vec2({}, {})", a, b)
}

#[generate_builtin_fn("Vec3")]
fn Vec3Constructor(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

#[generate_glsl_impl_inline("Vec3Constructor")]
fn generate(a: &str, b: &str, c: &str) -> String {
    format!("vec3({}, {}, {})", a, b, c)
}

#[generate_builtin_fn("Vec4")]
fn Vec4Constructor(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

#[generate_glsl_impl_inline("Vec4Constructor")]
fn generate(a: &str, b: &str, c: &str, d: &str) -> String {
    format!("vec4({}, {}, {}, {})", a, b, c, d)
}

#[generate_builtin_fn("__op_unary_neg")]
fn UnNegFloat(a: f32) -> f32 {
    -a
}

#[generate_glsl_impl_inline("UnNegFloat")]
fn generate(a: &str) -> String {
    format!("-{}", a)
}
