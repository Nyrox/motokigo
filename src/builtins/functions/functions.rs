pub const FUNCTIONS: &[&dyn BuiltInCallable] = &[
	&Vec4FloatBinMul,
	&FloatVec4BinMul,
	&Vec4FloatBinDiv,
	&Vec4Vec4BinAdd,
	&Vec4Vec4BinSub,
	&Vec4BinNeg,
	&Vec4Vec4BinEquality,
	&Vec4Vec4BinNotEqual,
	&Vec3FloatBinMul,
	&FloatVec3BinMul,
	&Vec3FloatBinDiv,
	&Vec3Vec3BinAdd,
	&Vec3Vec3BinSub,
	&Vec3BinNeg,
	&Vec3Vec3BinEquality,
	&Vec3Vec3BinNotEqual,
	&Vec2FloatBinMul,
	&FloatVec2BinMul,
	&Vec2FloatBinDiv,
	&Vec2Vec2BinAdd,
	&Vec2Vec2BinSub,
	&Vec2BinNeg,
	&Vec2Vec2BinEquality,
	&Vec2Vec2BinNotEqual,
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
	&FloatUnNeg,
	&FloatFloatBinEquality,
	&FloatFloatBinNotEqual,
	&FloatFloatBinLess,
	&FloatFloatBinLessEq,
	&FloatFloatBinGreater,
	&FloatFloatBinGreaterEq,
	&IntUnNeg,
	&IntIntBinEquality,
	&IntIntBinNotEqual,
	&IntIntBinLess,
	&IntIntBinLessEq,
	&IntIntBinGreater,
	&IntIntBinGreaterEq,
	&FloatFloatBinMul,
	&FloatFloatBinDiv,
	&FloatFloatBinAdd,
	&FloatFloatBinSub,
	&IntUnNot,
	&IntIntBinMul,
	&IntIntBinDiv,
	&IntIntBinAdd,
	&IntIntBinSub,
	&IntIntBinAnd,
	&IntIntBinOr,
	&IntIntBinXor,
	&FloatCast,
	&IntCast,
	&Vec2IntElem,
	&Vec2Length,
	&Vec2Normalize,
	&Vec2Vec2Dot,
	&Vec2Vec2Distance,
	&Vec3IntElem,
	&Vec3Length,
	&Vec3Normalize,
	&Vec3Vec3Dot,
	&Vec3Vec3Distance,
	&Vec4IntElem,
	&Vec4Length,
	&Vec4Normalize,
	&Vec4Vec4Dot,
	&Vec4Vec4Distance,
	&FloatAbs,
	&FloatSign,
	&IntAbs,
	&IntSign,
	&FloatSin,
	&FloatCos,
	&FloatTan,
	&FloatAsin,
	&FloatAcos,
	&FloatAtan,
	&FloatRadians,
	&FloatDegrees,
	&FloatExp,
	&FloatLog,
	&FloatExp2,
	&FloatLog2,
	&FloatSqrt,
	&FloatFloor,
	&FloatCeil,
	&FloatFract,
	&FloatFloatAtan2,
	&FloatFloatPow,
	&FloatFloatLogn,
	&FloatFloatMin,
	&FloatFloatMax,
];