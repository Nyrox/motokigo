use crate::ast::TypeKind;
use crate::builtins::*;
use crate::vm::VirtualMachine;
use builtins::{
    generate_builtin_fn, generate_glsl_impl_inline, generate_matrix_ctor, generate_vector_ctor,
};

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

            #[generate_builtin_fn("__op_binary_div")]
            fn [<BinDiv $name Float>](v: $name, a: $t) -> $name {
                v / a
            }

            #[generate_glsl_impl_inline("BinDiv{}Float", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("{} / {}", a, b)
            }

            #[generate_builtin_fn("__op_binary_add")]
            fn [<BinAdd $name $name>](a: $name, b: $name) -> $name {
                a + b
            }

            #[generate_glsl_impl_inline("BinAdd{}{}", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("{} + {}", a, b)
            }

            #[generate_builtin_fn("__op_binary_sub")]
            fn [<BinSub $name $name>](a: $name, b: $name) -> $name {
                a - b
            }

            #[generate_glsl_impl_inline("BinSub{}{}", $name)]
            fn generate(a: &str, b: &str) -> String {
                format!("{} - {}", a, b)
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

generate_vector_ctor!(2);
generate_vector_ctor!(3);
generate_vector_ctor!(4);

generate_matrix_ctor!(2, 2);
generate_matrix_ctor!(2, 3);
generate_matrix_ctor!(2, 4);
generate_matrix_ctor!(3, 2);
generate_matrix_ctor!(3, 3);
generate_matrix_ctor!(3, 4);
generate_matrix_ctor!(4, 2);
generate_matrix_ctor!(4, 3);
generate_matrix_ctor!(4, 4);

#[generate_builtin_fn("__op_unary_neg")]
fn UnNegFloat(a: f32) -> f32 {
    -a
}

#[generate_glsl_impl_inline("UnNegFloat")]
fn generate(a: &str) -> String {
    format!("-{}", a)
}

#[generate_builtin_fn("__op_binary_less")]
fn BinLessIntInt(a: i32, b: i32) -> i32 {
    if a < b { 1 } else { 0 }
}

#[generate_glsl_impl_inline("BinLessIntInt")]
fn generate(a: &str, b: &str) -> String {
    format!("{} < {}", a, b)
}

#[generate_builtin_fn("__op_binary_add")]
fn BinAddFloatFloat(a: f32, b: f32) -> f32 {
    a + b
}

#[generate_glsl_impl_inline("BinAddFloatFloat")]
fn generate(a: &str, b: &str) -> String {
    format!("{} + {}", a, b)
}

#[generate_builtin_fn("__op_binary_add")]
fn BinAddIntInt(a: i32, b: i32) -> i32 {
    a + b
}

#[generate_glsl_impl_inline("BinAddIntInt")]
fn generate(a: &str, b: &str) -> String {
    format!("{} + {}", a, b)
}