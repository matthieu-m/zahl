//! Division with fallback type when dividing by zero.

/// A trait for division with fallback on division by zero.
pub trait DivOr<Rhs, F> {
    /// Type of the result.
    type Output;

    /// Division or fallback.
    fn div_or(self, other: Rhs, fallback: F) -> Self::Output;
}
