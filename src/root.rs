//! The (missing) Root trait.

/// The Root trait.
pub trait Root<Rhs> {
    /// The type of taking the root of self by Rhs.
    type Output;

    /// Returns the n-th root of self.
    fn root(self, n: Rhs) -> Self::Output;
}
