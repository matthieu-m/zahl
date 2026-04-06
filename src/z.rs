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

impl<const N: i32> From<Z<N>> for i32 {
    fn from(_: Z<N>) -> Self {
        N
    }
}

//
//  Arithmetic for Z
//

macro_rules! impl_z {
    (abs $i:literal = $result:literal) => {
        impl $crate::Abs for $crate::Z<$i> {
            type Output = $crate::Z<$result>;

            fn abs(self) -> $crate::Z<$result> {
                const {
                    assert!($i.abs() == $result);
                }

                $crate::Z
            }
        }
    };
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
    ($left:literal pow $right:literal = $result:literal) => {
        impl $crate::Pow<$crate::Z<$right>> for $crate::Z<$left> {
            type Output = $crate::Z<$result>;

            fn pow(self, _other: $crate::Z<$right>) -> $crate::Z<$result> {
                const {
                    assert!($left.pow($right as u32) == $right);
                }

                $crate::Z
            }
        }
    };
    ($left:literal root $right:literal = $result:literal) => {
        impl $crate::Root<$crate::Z<$right>> for $crate::Z<$left> {
            type Output = $crate::Z<$result>;

            fn root(self, _other: $crate::Z<$right>) -> $crate::Z<$result> {
                const {
                    assert!($result.pow($right as u32) == $left);
                }

                $crate::Z
            }
        }
    };
}

impl_z!(abs 0i32 = 0);
impl_z!(neg 0i32 = 0);
impl_z!(0 + 0 = 0);
impl_z!(0 - 0 = 0);

impl_z!(0 * 0 = 0);

//
//  Script generated modules
//

#[cfg(feature = "abs-3")]
mod abs_1_3;

#[cfg(feature = "abs-6")]
mod abs_4_6;

#[cfg(feature = "abs-9")]
mod abs_7_9;

#[cfg(feature = "abs-12")]
mod abs_10_12;

#[cfg(feature = "abs-15")]
mod abs_13_15;

#[cfg(feature = "abs-18")]
mod abs_16_18;

#[cfg(feature = "abs-21")]
mod abs_19_21;

#[cfg(feature = "abs-24")]
mod abs_22_24;

#[cfg(feature = "abs-27")]
mod abs_25_27;

#[cfg(feature = "abs-30")]
mod abs_28_30;

#[cfg(feature = "abs-33")]
mod abs_31_33;

#[cfg(feature = "abs-36")]
mod abs_34_36;

#[cfg(feature = "abs-39")]
mod abs_37_39;

#[cfg(feature = "abs-42")]
mod abs_40_42;

#[cfg(feature = "abs-45")]
mod abs_43_45;

#[cfg(feature = "abs-48")]
mod abs_46_48;

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

#[cfg(feature = "add-39")]
mod add_37_39;

#[cfg(feature = "add-42")]
mod add_40_42;

#[cfg(feature = "add-45")]
mod add_43_45;

#[cfg(feature = "add-48")]
mod add_46_48;

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

#[cfg(feature = "div-39")]
mod div_37_39;

#[cfg(feature = "div-42")]
mod div_40_42;

#[cfg(feature = "div-45")]
mod div_43_45;

#[cfg(feature = "div-48")]
mod div_46_48;

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

#[cfg(feature = "mul-39")]
mod mul_37_39;

#[cfg(feature = "mul-42")]
mod mul_40_42;

#[cfg(feature = "mul-45")]
mod mul_43_45;

#[cfg(feature = "mul-48")]
mod mul_46_48;

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

#[cfg(feature = "neg-39")]
mod neg_37_39;

#[cfg(feature = "neg-42")]
mod neg_40_42;

#[cfg(feature = "neg-45")]
mod neg_43_45;

#[cfg(feature = "neg-48")]
mod neg_46_48;

#[cfg(feature = "pow-3")]
mod pow_1_3;

#[cfg(feature = "pow-6")]
mod pow_4_6;

#[cfg(feature = "pow-9")]
mod pow_7_9;

#[cfg(feature = "pow-12")]
mod pow_10_12;

#[cfg(feature = "pow-15")]
mod pow_13_15;

#[cfg(feature = "pow-18")]
mod pow_16_18;

#[cfg(feature = "pow-21")]
mod pow_19_21;

#[cfg(feature = "pow-24")]
mod pow_22_24;

#[cfg(feature = "pow-27")]
mod pow_25_27;

#[cfg(feature = "pow-30")]
mod pow_28_30;

#[cfg(feature = "pow-33")]
mod pow_31_33;

#[cfg(feature = "pow-36")]
mod pow_34_36;

#[cfg(feature = "pow-39")]
mod pow_37_39;

#[cfg(feature = "pow-42")]
mod pow_40_42;

#[cfg(feature = "pow-45")]
mod pow_43_45;

#[cfg(feature = "pow-48")]
mod pow_46_48;

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

#[cfg(feature = "rem-39")]
mod rem_37_39;

#[cfg(feature = "rem-42")]
mod rem_40_42;

#[cfg(feature = "rem-45")]
mod rem_43_45;

#[cfg(feature = "rem-48")]
mod rem_46_48;

#[cfg(feature = "root-3")]
mod root_1_3;

#[cfg(feature = "root-6")]
mod root_4_6;

#[cfg(feature = "root-9")]
mod root_7_9;

#[cfg(feature = "root-12")]
mod root_10_12;

#[cfg(feature = "root-15")]
mod root_13_15;

#[cfg(feature = "root-18")]
mod root_16_18;

#[cfg(feature = "root-21")]
mod root_19_21;

#[cfg(feature = "root-24")]
mod root_22_24;

#[cfg(feature = "root-27")]
mod root_25_27;

#[cfg(feature = "root-30")]
mod root_28_30;

#[cfg(feature = "root-33")]
mod root_31_33;

#[cfg(feature = "root-36")]
mod root_34_36;

#[cfg(feature = "root-39")]
mod root_37_39;

#[cfg(feature = "root-42")]
mod root_40_42;

#[cfg(feature = "root-45")]
mod root_43_45;

#[cfg(feature = "root-48")]
mod root_46_48;

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

#[cfg(feature = "sub-39")]
mod sub_37_39;

#[cfg(feature = "sub-42")]
mod sub_40_42;

#[cfg(feature = "sub-45")]
mod sub_43_45;

#[cfg(feature = "sub-48")]
mod sub_46_48;
