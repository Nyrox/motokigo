use crate::builtins::*;

pub mod basics;
pub use basics::*;

include!(concat!(env!("OUT_DIR"), "/functions.rs"));
