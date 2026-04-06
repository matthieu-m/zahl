//! Implementation of neg for numbers in 10..=12.

impl_z!(neg - 12i32 = 12i32);
impl_z!(neg - 11i32 = 11i32);
impl_z!(neg - 10i32 = 10i32);
impl_z!(neg 10i32 = -10i32);
impl_z!(neg 11i32 = -11i32);
impl_z!(neg 12i32 = -12i32);
