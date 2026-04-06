//! The (missing) Max trait.

/// The Max trait.
pub trait Max<Rhs> {
    /// The type of compute the maximum of self and the right hand side.
    type Output;

    /// Returns the maximum of self and other.
    fn max(self, other: Rhs) -> Self::Output;
}
