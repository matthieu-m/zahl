//! A pragmatic type-level integer library, focused on human-friendliness.

//  Support core.
#![cfg_attr(not(test), no_std)]

mod abs;
mod div_or;
mod max;
mod min;
mod pow;
mod root;
mod root_or;
mod z;

pub use abs::Abs;
pub use div_or::DivOr;
pub use max::Max;
pub use min::Min;
pub use pow::Pow;
pub use root::Root;
pub use root_or::RootOr;
pub use z::Z;
