use crate::builtins::*;

#[macro_use] mod utils;
pub mod basics;
pub use basics::*;
pub mod stdlib;
pub use stdlib::*;

include!("functions.rs");
