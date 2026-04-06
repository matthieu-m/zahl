//! Implementation of neg for numbers in 31..=33.

impl_z!(neg - 33i32 = 33i32);
impl_z!(neg - 32i32 = 32i32);
impl_z!(neg - 31i32 = 31i32);
impl_z!(neg 31i32 = -31i32);
impl_z!(neg 32i32 = -32i32);
impl_z!(neg 33i32 = -33i32);
