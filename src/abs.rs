//! The (missing) Abs trait.

/// The Abs trait.
pub trait Abs {
    /// The type of the absolute value of self.
    type Output;

    /// Returns the absolute value of self.
    fn abs(self) -> Self::Output;
}
