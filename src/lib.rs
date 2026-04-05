//! A pragmatic type-level integer library, focused on human-friendliness.

//  Support core.
#![cfg_attr(not(test), no_std)]

mod z;

pub use z::Z;
