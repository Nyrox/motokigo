use crate::builtins::*;

pub mod basics;
pub use basics::*;

//TODO: Reintroduce proc-macro?
//include!(concat!(env!("OUT_DIR"), "/functions.rs"));
pub const FUNCTIONS: &[&dyn BuiltInCallable] = &[
    &BinMulFloatVec2,
    &BinMulFloatVec3,
    &BinMulFloatVec4,
    &BinMulVec2Float,
    &BinMulVec3Float,
    &BinMulVec4Float,
    &BinAddVec2Vec2,
    &BinAddVec3Vec3,
    &BinAddVec4Vec4,
    &Vec2Constructor,
    &Vec3Constructor,
    &Vec4Constructor,
    &Vec2Normalize,
    &Vec3Normalize,
    &Vec4Normalize,
    &Vec2Dot,
    &Vec3Dot,
    &Vec4Dot,
    &UnNegFloat,
];
