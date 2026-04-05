//! The core type-level integer type: Z.

/// Type-level integer.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Z<const N: i32>;

impl<const N: i32> Z<N> {
    /// Constructs a new (stateless) instance.
    pub const fn new() -> Self {
        Self
    }
}

macro_rules! impl_z {
    (neg $i:literal = $result:literal) => {
        impl core::ops::Neg for $crate::Z<$i> {
            type Output = $crate::Z<$result>;

            fn neg(self) -> $crate::Z<$result> {
                const {
                    assert!(-($i) == $result);
                }

                $crate::Z
            }
        }
    };
    ($left:literal + $right:literal = $result:literal) => {
        impl core::ops::Add<$crate::Z<$right>> for $crate::Z<$left> {
            type Output = $crate::Z<$result>;

            fn add(self, _other: $crate::Z<$right>) -> $crate::Z<$result> {
                const {
                    assert!($left + $right == $result);
                }

                $crate::Z
            }
        }
    };
    ($left:literal - $right:literal = $result:literal) => {
        impl core::ops::Sub<$crate::Z<$right>> for $crate::Z<$left> {
            type Output = $crate::Z<$result>;

            fn sub(self, _other: $crate::Z<$right>) -> $crate::Z<$result> {
                const {
                    assert!($left - $right == $result);
                }

                $crate::Z
            }
        }
    };
    ($left:literal * $right:literal = $result:literal) => {
        impl core::ops::Mul<$crate::Z<$right>> for $crate::Z<$left> {
            type Output = $crate::Z<$result>;

            fn mul(self, _other: $crate::Z<$right>) -> $crate::Z<$result> {
                #![allow(clippy::neg_multiply)]

                const {
                    assert!($left * $right == $result);
                }

                $crate::Z
            }
        }
    };
    ($left:literal / $right:literal = $result:literal) => {
        impl core::ops::Div<$crate::Z<$right>> for $crate::Z<$left> {
            type Output = $crate::Z<$result>;

            fn div(self, _other: $crate::Z<$right>) -> $crate::Z<$result> {
                const {
                    assert!($left / $right == $result);
                }

                $crate::Z
            }
        }
    };
    ($left:literal % $right:literal = $result:literal) => {
        impl core::ops::Rem<$crate::Z<$right>> for $crate::Z<$left> {
            type Output = $crate::Z<$result>;

            fn rem(self, _other: $crate::Z<$right>) -> $crate::Z<$result> {
                const {
                    assert!($left % $right == $result);
                }

                $crate::Z
            }
        }
    };
}

impl_z!(0 + 0 = 0);
impl_z!(0 - 0 = 0);
impl_z!(neg 0 = 0);

impl_z!(0 * 0 = 0);

//
//  Script generated modules
//

#[cfg(feature = "add-1-3")]
mod add_1_3;

#[cfg(feature = "add-4-6")]
mod add_4_6;

#[cfg(feature = "add-7-9")]
mod add_7_9;

#[cfg(feature = "add-10-12")]
mod add_10_12;

#[cfg(feature = "add-13-15")]
mod add_13_15;

#[cfg(feature = "add-16-18")]
mod add_16_18;

#[cfg(feature = "add-19-21")]
mod add_19_21;

#[cfg(feature = "add-22-24")]
mod add_22_24;

#[cfg(feature = "div-1-3")]
mod div_1_3;

#[cfg(feature = "div-4-6")]
mod div_4_6;

#[cfg(feature = "div-7-9")]
mod div_7_9;

#[cfg(feature = "div-10-12")]
mod div_10_12;

#[cfg(feature = "div-13-15")]
mod div_13_15;

#[cfg(feature = "div-16-18")]
mod div_16_18;

#[cfg(feature = "div-19-21")]
mod div_19_21;

#[cfg(feature = "div-22-24")]
mod div_22_24;

#[cfg(feature = "mul-1-3")]
mod mul_1_3;

#[cfg(feature = "mul-4-6")]
mod mul_4_6;

#[cfg(feature = "mul-7-9")]
mod mul_7_9;

#[cfg(feature = "mul-10-12")]
mod mul_10_12;

#[cfg(feature = "mul-13-15")]
mod mul_13_15;

#[cfg(feature = "mul-16-18")]
mod mul_16_18;

#[cfg(feature = "mul-19-21")]
mod mul_19_21;

#[cfg(feature = "mul-22-24")]
mod mul_22_24;

#[cfg(feature = "neg-1-3")]
mod neg_1_3;

#[cfg(feature = "neg-4-6")]
mod neg_4_6;

#[cfg(feature = "neg-7-9")]
mod neg_7_9;

#[cfg(feature = "neg-10-12")]
mod neg_10_12;

#[cfg(feature = "neg-13-15")]
mod neg_13_15;

#[cfg(feature = "neg-16-18")]
mod neg_16_18;

#[cfg(feature = "neg-19-21")]
mod neg_19_21;

#[cfg(feature = "neg-22-24")]
mod neg_22_24;

#[cfg(feature = "rem-1-3")]
mod rem_1_3;

#[cfg(feature = "rem-4-6")]
mod rem_4_6;

#[cfg(feature = "rem-7-9")]
mod rem_7_9;

#[cfg(feature = "rem-10-12")]
mod rem_10_12;

#[cfg(feature = "rem-13-15")]
mod rem_13_15;

#[cfg(feature = "rem-16-18")]
mod rem_16_18;

#[cfg(feature = "rem-19-21")]
mod rem_19_21;

#[cfg(feature = "rem-22-24")]
mod rem_22_24;

#[cfg(feature = "sub-1-3")]
mod sub_1_3;

#[cfg(feature = "sub-4-6")]
mod sub_4_6;

#[cfg(feature = "sub-7-9")]
mod sub_7_9;

#[cfg(feature = "sub-10-12")]
mod sub_10_12;

#[cfg(feature = "sub-13-15")]
mod sub_13_15;

#[cfg(feature = "sub-16-18")]
mod sub_16_18;

#[cfg(feature = "sub-19-21")]
mod sub_19_21;

#[cfg(feature = "sub-22-24")]
mod sub_22_24;
