//! The (missing) Min trait.

/// The Min trait.
pub trait Min<Rhs> {
    /// The type of compute the minimum of self and the right hand side.
    type Output;

    /// Returns the minimum of self and other.
    fn min(self, other: Rhs) -> Self::Output;
}
