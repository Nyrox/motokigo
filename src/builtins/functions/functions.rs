pub const FUNCTIONS: &[&dyn BuiltInCallable] = &[
	&BinMulVec4Float,
	&BinMulFloatVec4,
	&BinDivVec4Float,
	&BinAddVec4Vec4,
	&BinSubVec4Vec4,
	&Vec4Normalize,
	&Vec4Dot,
	&BinMulVec3Float,
	&BinMulFloatVec3,
	&BinDivVec3Float,
	&BinAddVec3Vec3,
	&BinSubVec3Vec3,
	&Vec3Normalize,
	&Vec3Dot,
	&BinMulVec2Float,
	&BinMulFloatVec2,
	&BinDivVec2Float,
	&BinAddVec2Vec2,
	&BinSubVec2Vec2,
	&Vec2Normalize,
	&Vec2Dot,
	&Vec2Constructor,
	&Vec3Constructor,
	&Vec4Constructor,
	&Mat2Constructor,
	&Mat2VectorConstructor,
	&Mat2x3Constructor,
	&Mat2x3VectorConstructor,
	&Mat2x4Constructor,
	&Mat2x4VectorConstructor,
	&Mat3x2Constructor,
	&Mat3x2VectorConstructor,
	&Mat3Constructor,
	&Mat3VectorConstructor,
	&Mat3x4Constructor,
	&Mat3x4VectorConstructor,
	&Mat4x2Constructor,
	&Mat4x2VectorConstructor,
	&Mat4x3Constructor,
	&Mat4x3VectorConstructor,
	&Mat4Constructor,
	&Mat4VectorConstructor,
    &UnNegFloat,
    &BinLessIntInt,
    &BinAddFloatFloat,
    &BinAddIntInt,
];