/// Describes the fraction of something.
///
/// Valid fraction values are always between 0.0 - 1.0 (inclusive).
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Fraction(f32);

impl Fraction {
    pub fn new(value: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&value),
            "Invalid Fraction, tried to initialize with a value of: {}",
            value
        );

        Self(value)
    }

    pub const fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    pub fn value(&self) -> f32 {
        self.0
    }

    /// Returns the inverse of this fraction.
    pub fn inverse(&self) -> Self {
        Self(1.0 - self.0)
    }
}
