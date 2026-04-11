//! Root with fallback type when dividing by zero.

/// A trait for root with fallback on root by zero.
pub trait RootOr<Rhs, F> {
    /// Type of the result.
    type Output;

    /// Returns the `n`-th root of `self`, or fallback if `Rhs` is a zero.
    fn root_or(self, n: Rhs, fallback: F) -> Self::Output;
}
