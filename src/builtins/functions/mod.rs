use crate::builtins::*;

pub mod basics;
use basics::*;

include!(concat!(env!("OUT_DIR"), "/functions.rs"));
