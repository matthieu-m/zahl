//! A pragmatic type-level integer library, focused on human-friendliness.

//  Support core.
#![cfg_attr(not(test), no_std)]

mod pow;
mod root;
mod z;

pub use pow::Pow;
pub use root::Root;
pub use z::Z;
