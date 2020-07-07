use crate::builtins::*;

pub mod basics;
pub use basics::*;

//TODO: Reintroduce proc-macro?
//include!(concat!(env!("OUT_DIR"), "/functions.rs"));
pub const FUNCTIONS: &[&dyn BuiltInCallable] = &[
    &BinMulFloatVec3,
    &BinMulVec3Float,
    &BinAddVec3Vec3,
    &Vec3Constructor,
    &Vec3Normalize,
    &Vec3Dot,
    &UnNegFloat,
];
