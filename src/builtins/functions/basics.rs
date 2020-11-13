use crate::{ast::TypeKind, builtins::*, vm::VirtualMachine};
use macros::{generate_builtin_fn, generate_glsl_impl_inline, generate_matrix_ctor, generate_vector_ctor};

use crate::glsl::{compiler::GenerateGLSL, BuiltInCallableGLSL};

macro_rules! implement_vec_op {
	( $name:ident, $comp:ident ) => {
		paste::item! {
            implement_func!(BinMul, __op_binary_mul, a * b, "{} * {}", $name, $comp, $name);
            implement_func!(BinMul, __op_binary_mul, b * a, "{} * {}", $comp, $name, $name);
            implement_func!(BinDiv, __op_binary_div, a / b, "{} / {}", $name, $comp, $name);
            implement_func!(BinAdd, __op_binary_add, a + b, "{} + {}", $name, $name, $name);
            implement_func!(BinSub, __op_binary_sub, a - b, "{} - {}", $name, $name, $name);

            implement_func!(BinNeg, __op_unary_neg, a * (-1.0 as $comp), "-{}", $name, $name);
            implement_func!(BinEquality, __op_binary_equality, if a == b { 1 } else { 0 }, "{} == {}", $name, $name, Int);
            implement_func!(BinNotEqual, __op_binary_not_equal, if a != b { 1 } else { 0 }, "{} != {}", $name, $name, Int);
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
            implement_func!(UnNeg, __op_unary_neg, -a, "-{}", $name, $name);
            implement_func!(BinEquality, __op_binary_equality, if a == b { 1 } else { 0 }, "{} == {}", $name, $name, Int);
            implement_func!(BinNotEqual, __op_binary_not_equal, if a != b { 1 } else { 0 }, "{} != {}", $name, $name, Int);

            implement_func!(BinLess, __op_binary_less, if a < b { 1 } else { 0 }, "{} < {}", $name, $name, Int);
            implement_func!(BinLessEq, __op_binary_less_equal, if a <= b { 1 } else { 0 }, "{} <= {}", $name, $name, Int);       
            implement_func!(BinGreater, __op_binary_greater, if a >= b { 1 } else { 0 }, "{} > {}", $name, $name, Int);
            implement_func!(BinGreaterEq, __op_binary_greater_equal, if a >= b { 1 } else { 0 }, "{} >= {}", $name, $name, Int);
		}
	};
}

macro_rules! implement_float_num_ops {
	( $name:ident ) => {
		paste::item! {
            implement_func!(BinMul, __op_binary_mul, a * b, "{} * {}", $name, $name, $name);
            implement_func!(BinDiv, __op_binary_div, a / b, "{} / {}", $name, $name, $name);
            implement_func!(BinAdd, __op_binary_add, a + b, "{} + {}", $name, $name, $name);
            implement_func!(BinSub, __op_binary_sub, a - b, "{} - {}", $name, $name, $name);
		}
	};
}

macro_rules! implement_integer_num_ops {
	( $name:ident ) => {
		paste::item! {
            implement_func!(UnNot, __op_unary_not, if a == 0 { 1 as $name } else { 0 as $name }, "!{}", $name, $name);

            implement_func!(BinMul, __op_binary_mul, a.wrapping_mul(b), "{} * {}", $name, $name, $name);
            implement_func!(BinDiv, __op_binary_div, a.wrapping_div(b), "{} / {}", $name, $name, $name);
            implement_func!(BinAdd, __op_binary_add, a.wrapping_add(b), "{} + {}", $name, $name, $name);
            implement_func!(BinSub, __op_binary_sub, a.wrapping_sub(b), "{} - {}", $name, $name, $name);

            implement_func!(BinAnd, __op_binary_and, if a != 0 && b != 0 { 1 } else { 0 }, "bool({}) && bool({})", $name, $name, $name);
            implement_func!(BinOr, __op_binary_or, if a != 0 || b != 0 { 1 } else { 0 }, "bool({}) || bool({})", $name, $name, $name);
            implement_func!(BinXor, __op_binary_or, unimplemented!("{}, {}", a, b), "bool({}) ^^ bool({})", $name, $name, $name);
		}
	};
}

implement_common_num_ops!(Float);
implement_common_num_ops!(Int);
implement_float_num_ops!(Float);
implement_integer_num_ops!(Int);

implement_func!(Cast, int, a as i32, "int({})", Float, Int);
implement_func!(Cast, float, a as f32, "float({})", Int, Float);
