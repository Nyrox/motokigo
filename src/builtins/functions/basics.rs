use crate::{ast::TypeKind, builtins::*, vm::VirtualMachine};
use macros::{generate_builtin_fn, generate_glsl_impl_inline, generate_matrix_ctor, generate_vector_ctor};

use crate::glsl::{compiler::GenerateGLSL, BuiltInCallableGLSL};

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

			#[generate_glsl_impl_inline("BinAdd{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} + {}", a, b)
			}

			#[generate_builtin_fn("__op_binary_sub")]
			fn [<BinSub $name $name>](a: $name, b: $name) -> $name {
				a - b
			}

			#[generate_glsl_impl_inline("BinSub{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} - {}", a, b)
            }
            
            // Negation
			#[generate_builtin_fn("__op_unary_neg")]
			fn [<UnNeg $name>](a: $t) -> $t {
				-a
			}

			#[generate_glsl_impl_inline("UnNeg{}", $name)]
			fn generate(a: &str) -> String {
				format!("-{}", a)
            }
            
            // Equality
			#[generate_builtin_fn("__op_binary_equality")]
			fn [<BinEquality $name $name>](a: $t, b: $t) -> $t {
				if a == b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinEquality{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} == {}", a, b)
            }
            
            // Not equal
			#[generate_builtin_fn("__op_binary_not_equal")]
			fn [<BinNotEqual $name $name>](a: $t, b: $t) -> $t {
				if a != b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinNotEqual{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} != {}", a, b)
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

macro_rules! implement_common_num_ops {
	( $name:ident, $t:ident ) => {
		paste::item! {
			// Negation
			#[generate_builtin_fn("__op_unary_neg")]
			fn [<UnNeg $name>](a: $t) -> $t {
				-a
			}

			#[generate_glsl_impl_inline("UnNeg{}", $name)]
			fn generate(a: &str) -> String {
				format!("-{}", a)
			}

			// Less
			#[generate_builtin_fn("__op_binary_less")]
			fn [<BinLess $name $name>](a: $t, b: $t) -> $t {
				if a < b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinLess{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} < {}", a, b)
			}

			// Less equal
			#[generate_builtin_fn("__op_binary_less_equal")]
			fn [<BinLessEqual $name $name>](a: $t, b: $t) -> $t {
				if a <= b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinLessEqual{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} <= {}", a, b)
			}

			// Greater
			#[generate_builtin_fn("__op_binary_greater")]
			fn [<BinGreater $name $name>](a: $t, b: $t) -> $t {
				if a > b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinGreater{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} > {}", a, b)
			}

			// Greater equal
			#[generate_builtin_fn("__op_binary_greater_equal")]
			fn [<BinGreaterEqual $name $name>](a: $t, b: $t) -> $t {
				if a >= b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinGreaterEqual{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} >= {}", a, b)
			}

			// Equality
			#[generate_builtin_fn("__op_binary_equality")]
			fn [<BinEquality $name $name>](a: $t, b: $t) -> $t {
				if a == b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinEquality{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} == {}", a, b)
            }
            
            // Not equal
			#[generate_builtin_fn("__op_binary_not_equal")]
			fn [<BinNotEqual $name $name>](a: $t, b: $t) -> $t {
				if a != b { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("BinNotEqual{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} != {}", a, b)
            }
		}
	};
}

macro_rules! implement_float_num_ops {
	( $name:ident, $t:ident ) => {
		paste::item! {
			// Add
			#[generate_builtin_fn("__op_binary_add")]
			fn [<BinAdd $name $name>](a: $t, b: $t) -> $t {
				a + b
			}

			#[generate_glsl_impl_inline("BinAdd{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} + {}", a, b)
			}

			// Mul
			#[generate_builtin_fn("__op_binary_mul")]
			fn [<BinMul $name $name>](a: $t, b: $t) -> $t {
				a * b
			}

			#[generate_glsl_impl_inline("BinMul{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} * {}", a, b)
			}

			// Sub
			#[generate_builtin_fn("__op_binary_sub")]
			fn [<BinSub $name $name>](a: $t, b: $t) -> $t {
				a - b
			}

			#[generate_glsl_impl_inline("BinSub{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} - {}", a, b)
			}

			// Div
			#[generate_builtin_fn("__op_binary_div")]
			fn [<BinDiv $name $name>](a: $t, b: $t) -> $t {
				a / b
			}

			#[generate_glsl_impl_inline("BinDiv{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} / {}", a, b)
			}
		}
	};
}

macro_rules! implement_integer_num_ops {
	( $name:ident, $t:ident ) => {
		paste::item! {
			// Not
			#[generate_builtin_fn("__op_unary_not")]
			fn [<UnNot $name>](a: $t) -> $t {
				if a == 0 { 1 as $t } else { 0 as $t }
			}

			#[generate_glsl_impl_inline("UnNot{}", $name)]
			fn generate(a: &str) -> String {
				format!("!{}", a)
			}

			// Add
			#[generate_builtin_fn("__op_binary_add")]
			fn [<BinAdd $name $name>](a: $t, b: $t) -> $t {
				a.wrapping_add(b)
			}

			#[generate_glsl_impl_inline("BinAdd{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} + {}", a, b)
			}

			// Mul
			#[generate_builtin_fn("__op_binary_mul")]
			fn [<BinMul $name $name>](a: $t, b: $t) -> $t {
				a.wrapping_mul(b)
			}

			#[generate_glsl_impl_inline("BinMul{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} * {}", a, b)
			}

			// Sub
			#[generate_builtin_fn("__op_binary_sub")]
			fn [<BinSub $name $name>](a: $t, b: $t) -> $t {
				a.wrapping_sub(b)
			}

			#[generate_glsl_impl_inline("BinSub{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} - {}", a, b)
			}

			// Div
			#[generate_builtin_fn("__op_binary_div")]
			fn [<BinDiv $name $name>](a: $t, b: $t) -> $t {
				a.wrapping_div(b)
			}

			#[generate_glsl_impl_inline("BinDiv{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("{} / {}", a, b)
            }
            
            // And
			#[generate_builtin_fn("__op_binary_and")]
			fn [<BinAnd $name $name>](a: $t, b: $t) -> $t {
				if a != 0 && b != 0 { 1 } else { 0 }
			}

			#[generate_glsl_impl_inline("BinAnd{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("bool({}) && bool({})", a, b)
            }

            // Or
			#[generate_builtin_fn("__op_binary_or")]
			fn [<BinOr $name $name>](a: $t, b: $t) -> $t {
                if a != 0 || b != 0 { 1 } else { 0 }
			}

			#[generate_glsl_impl_inline("BinOr{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("bool({}) || bool({})", a, b)
            }

            // Xor
			#[generate_builtin_fn("__op_binary_xor")]
			fn [<BinXor $name $name>](_a: $t, _b: $t) -> $t {
				unimplemented!("XOR not yet implemented");
			}

			#[generate_glsl_impl_inline("BinXor{}{}", $name, $name)]
			fn generate(a: &str, b: &str) -> String {
				format!("bool({}) ^^ bool({})", a, b)
            }
		}
	};
}

implement_common_num_ops!(Float, f32);
implement_common_num_ops!(Int, i32);
implement_float_num_ops!(Float, f32);
implement_integer_num_ops!(Int, i32);

#[generate_builtin_fn("int")]
fn CastFloatInt(a: f32) -> i32 {
	a as i32
}

#[generate_glsl_impl_inline("CastFloatInt")]
fn generate(a: &str) -> String {
	format!("int({})", a)
}

#[generate_builtin_fn("float")]
fn CastIntFloat(a: i32) -> f32 {
	a as f32
}

#[generate_glsl_impl_inline("CastIntFloat")]
fn generate(a: &str) -> String {
	format!("float({})", a)
}
