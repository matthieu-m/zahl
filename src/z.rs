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

#[cfg(feature = "add-3")]
mod add_1_3;

#[cfg(feature = "add-6")]
mod add_4_6;

#[cfg(feature = "add-9")]
mod add_7_9;

#[cfg(feature = "add-12")]
mod add_10_12;

#[cfg(feature = "add-15")]
mod add_13_15;

#[cfg(feature = "add-18")]
mod add_16_18;

#[cfg(feature = "add-21")]
mod add_19_21;

#[cfg(feature = "add-24")]
mod add_22_24;

#[cfg(feature = "add-27")]
mod add_25_27;

#[cfg(feature = "add-30")]
mod add_28_30;

#[cfg(feature = "add-33")]
mod add_31_33;

#[cfg(feature = "add-36")]
mod add_34_36;

#[cfg(feature = "div-3")]
mod div_1_3;

#[cfg(feature = "div-6")]
mod div_4_6;

#[cfg(feature = "div-9")]
mod div_7_9;

#[cfg(feature = "div-12")]
mod div_10_12;

#[cfg(feature = "div-15")]
mod div_13_15;

#[cfg(feature = "div-18")]
mod div_16_18;

#[cfg(feature = "div-21")]
mod div_19_21;

#[cfg(feature = "div-24")]
mod div_22_24;

#[cfg(feature = "div-27")]
mod div_25_27;

#[cfg(feature = "div-30")]
mod div_28_30;

#[cfg(feature = "div-33")]
mod div_31_33;

#[cfg(feature = "div-36")]
mod div_34_36;

#[cfg(feature = "mul-3")]
mod mul_1_3;

#[cfg(feature = "mul-6")]
mod mul_4_6;

#[cfg(feature = "mul-9")]
mod mul_7_9;

#[cfg(feature = "mul-12")]
mod mul_10_12;

#[cfg(feature = "mul-15")]
mod mul_13_15;

#[cfg(feature = "mul-18")]
mod mul_16_18;

#[cfg(feature = "mul-21")]
mod mul_19_21;

#[cfg(feature = "mul-24")]
mod mul_22_24;

#[cfg(feature = "mul-27")]
mod mul_25_27;

#[cfg(feature = "mul-30")]
mod mul_28_30;

#[cfg(feature = "mul-33")]
mod mul_31_33;

#[cfg(feature = "mul-36")]
mod mul_34_36;

#[cfg(feature = "neg-3")]
mod neg_1_3;

#[cfg(feature = "neg-6")]
mod neg_4_6;

#[cfg(feature = "neg-9")]
mod neg_7_9;

#[cfg(feature = "neg-12")]
mod neg_10_12;

#[cfg(feature = "neg-15")]
mod neg_13_15;

#[cfg(feature = "neg-18")]
mod neg_16_18;

#[cfg(feature = "neg-21")]
mod neg_19_21;

#[cfg(feature = "neg-24")]
mod neg_22_24;

#[cfg(feature = "neg-27")]
mod neg_25_27;

#[cfg(feature = "neg-30")]
mod neg_28_30;

#[cfg(feature = "neg-33")]
mod neg_31_33;

#[cfg(feature = "neg-36")]
mod neg_34_36;

#[cfg(feature = "rem-3")]
mod rem_1_3;

#[cfg(feature = "rem-6")]
mod rem_4_6;

#[cfg(feature = "rem-9")]
mod rem_7_9;

#[cfg(feature = "rem-12")]
mod rem_10_12;

#[cfg(feature = "rem-15")]
mod rem_13_15;

#[cfg(feature = "rem-18")]
mod rem_16_18;

#[cfg(feature = "rem-21")]
mod rem_19_21;

#[cfg(feature = "rem-24")]
mod rem_22_24;

#[cfg(feature = "rem-27")]
mod rem_25_27;

#[cfg(feature = "rem-30")]
mod rem_28_30;

#[cfg(feature = "rem-33")]
mod rem_31_33;

#[cfg(feature = "rem-36")]
mod rem_34_36;

#[cfg(feature = "sub-3")]
mod sub_1_3;

#[cfg(feature = "sub-6")]
mod sub_4_6;

#[cfg(feature = "sub-9")]
mod sub_7_9;

#[cfg(feature = "sub-12")]
mod sub_10_12;

#[cfg(feature = "sub-15")]
mod sub_13_15;

#[cfg(feature = "sub-18")]
mod sub_16_18;

#[cfg(feature = "sub-21")]
mod sub_19_21;

#[cfg(feature = "sub-24")]
mod sub_22_24;

#[cfg(feature = "sub-27")]
mod sub_25_27;

#[cfg(feature = "sub-30")]
mod sub_28_30;

#[cfg(feature = "sub-33")]
mod sub_31_33;

#[cfg(feature = "sub-36")]
mod sub_34_36;
