use crate::builtins::*;

pub mod basics;
pub use basics::*;
pub mod stdlib;
pub use stdlib::*;

include!("functions.rs");
