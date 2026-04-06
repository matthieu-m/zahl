//! The (missing) Pow trait.

/// The Pow trait.
pub trait Pow<Rhs> {
    /// The type of elevating self to the n-th power.
    type Output;

    /// Returns self to the n-th power.
    fn pow(self, n: Rhs) -> Self::Output;
}
