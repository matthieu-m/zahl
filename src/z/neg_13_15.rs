//! Implementation of neg for numbers in 13..=15.

impl_z!(neg - 15i32 = 15i32);
impl_z!(neg - 14i32 = 14i32);
impl_z!(neg - 13i32 = 13i32);
impl_z!(neg 13i32 = -13i32);
impl_z!(neg 14i32 = -14i32);
impl_z!(neg 15i32 = -15i32);
