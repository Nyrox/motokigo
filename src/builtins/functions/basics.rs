use crate::{ast::TypeKind, builtins::*, vm::VirtualMachine};
use macros::{generate_builtin_fn, generate_glsl_impl_inline, generate_matrix_ctor, generate_vector_ctor};
use macros::bingbong;
use crate::glsl::{compiler::GenerateGLSL, BuiltInCallableGLSL};

macro_rules! implement_vec_op {
	( $name:ident, $comp:ident ) => {
		paste::item! {
            bingbong!(BinMul, __op_binary_mul, |a: $name, b: $comp| -> $name { a * b }, "{} * {}");
            bingbong!(BinMul, __op_binary_mul, |a: $comp, b: $name| -> $name { b * a }, "{} * {}");
            bingbong!(BinDiv, __op_binary_div, |a: $name, b: $comp| -> $name { a / b }, "{} / {}");
            bingbong!(BinAdd, __op_binary_add, |a: $name, b: $name| -> $name { a + b }, "{} + {}");
            bingbong!(BinSub, __op_binary_sub, |a: $name, b: $name| -> $name { a - b }, "{} - {}");

            bingbong!(BinNeg, __op_unary_neg, |a: $name| -> $name { a * (-1.0 as $comp) }, "-{}");
            bingbong!(BinEquality, __op_binary_equality, |a: $name, b: $name| -> Int { if a == b { 1 } else { 0 } }, "{} == {}");
            bingbong!(BinNotEqual, __op_binary_not_equal, |a: $name, b: $name| -> Int { if a != b { 1 } else { 0 } }, "{} != {}");
		}
	};
}

implement_vec_op!(Vec4, Float);
implement_vec_op!(Vec3, Float);
implement_vec_op!(Vec2, Float);

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

macro_rules! implement_common_num_ops {
	( $name:ident ) => {
		paste::item! {
            bingbong!(UnNeg, __op_unary_neg, |a: $name| -> $name { -a }, "-{}");
            bingbong!(BinEquality, __op_binary_equality, |a: $name, b: $name| -> Int { if a == b { 1 } else { 0 } }, "{} == {}");
            bingbong!(BinNotEqual, __op_binary_not_equal, |a: $name, b: $name| -> Int { if a != b { 1 } else { 0 } }, "{} != {}");

            bingbong!(BinLess, __op_binary_less, |a: $name, b: $name| -> Int { if a < b { 1 } else { 0 } }, "{} < {}");
            bingbong!(BinLessEq, __op_binary_less_equal, |a: $name, b: $name| -> Int { if a <= b { 1 } else { 0 } }, "{} <= {}");       
            bingbong!(BinGreater, __op_binary_greater, |a: $name, b: $name| -> Int { if a >= b { 1 } else { 0 } }, "{} > {}");
            bingbong!(BinGreaterEq, __op_binary_greater_equal, |a: $name, b: $name| -> Int { if a >= b { 1 } else { 0 } }, "{} >= {}");
		}
	};
}

macro_rules! implement_float_num_ops {
	( $name:ident ) => {
		paste::item! {
            bingbong!(BinMul, __op_binary_mul, |a: $name, b: $name| -> $name { a * b }, "{} * {}");
            bingbong!(BinDiv, __op_binary_div, |a: $name, b: $name| -> $name { a / b }, "{} / {}");
            bingbong!(BinAdd, __op_binary_add, |a: $name, b: $name| -> $name { a + b }, "{} + {}");
            bingbong!(BinSub, __op_binary_sub, |a: $name, b: $name| -> $name { a - b }, "{} - {}");
		}
	};
}

macro_rules! implement_integer_num_ops {
	( $name:ident ) => {
		paste::item! {
            bingbong!(UnNot, __op_unary_not, |a: $name| -> $name { if a == 0 { 1 as $name } else { 0 as $name } }, "!{}");
            bingbong!(BinMul, __op_binary_mul, |a: $name, b: $name| -> $name { a.wrapping_mul(b) }, "{} * {}");
            bingbong!(BinDiv, __op_binary_div, |a: $name, b: $name| -> $name { a.wrapping_div(b) }, "{} / {}");
            bingbong!(BinAdd, __op_binary_add, |a: $name, b: $name| -> $name { a.wrapping_add(b) }, "{} + {}");
            bingbong!(BinSub, __op_binary_sub, |a: $name, b: $name| -> $name { a.wrapping_sub(b) }, "{} - {}");
            bingbong!(BinAnd, __op_binary_and, |a: $name, b: $name| -> $name { if a != 0 && b != 0 { 1 } else { 0 } }, "bool({}) && bool({})");
            bingbong!(BinOr, __op_binary_or,   |a: $name, b: $name| -> $name { if a != 0 || b != 0 { 1 } else { 0 } }, "bool({}) || bool({})");
            bingbong!(BinXor, __op_binary_or,  |a: $name, b: $name| -> $name { unimplemented!("{}, {}", a, b) }, "bool({}) ^^ bool({})");
		}
	};
}

implement_common_num_ops!(Float);
implement_common_num_ops!(Int);
implement_float_num_ops!(Float);
implement_integer_num_ops!(Int);

bingbong!(Cast, int, |a: Float| -> Int { a as i32 }, "int({})");
bingbong!(Cast, float, |a: Int| -> Float { a as f32 }, "float({})");
