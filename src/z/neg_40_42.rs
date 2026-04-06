//! Implementation of neg for numbers in 40..=42.

impl_z!(neg - 42i32 = 42i32);
impl_z!(neg - 41i32 = 41i32);
impl_z!(neg - 40i32 = 40i32);
impl_z!(neg 40i32 = -40i32);
impl_z!(neg 41i32 = -41i32);
impl_z!(neg 42i32 = -42i32);
