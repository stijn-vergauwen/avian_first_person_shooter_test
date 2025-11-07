use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::ops::{Add, AddAssign, Range};

/// An angle in radians.
///
/// - In the context of Bevy, this angle represents a clockwise rotation along an axis.
/// - Radians and degrees both go in the same direction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Angle(pub f32);

#[allow(unused)]
impl Angle {
    pub const ZERO: Angle = Angle(0.0);

    pub const QUARTER: Angle = Angle(FRAC_PI_2);

    pub const HALF: Angle = Angle(PI);

    pub const FULL: Angle = Angle(TAU);

    pub const PI: Angle = Angle(PI);

    pub const TAU: Angle = Angle(TAU);

    pub const fn from_degrees(degrees: f32) -> Self {
        const RADS_PER_DEG: f32 = PI / 180.0;
        Angle(degrees * RADS_PER_DEG)
    }

    pub const fn from_turns(turns: f32) -> Self {
        Angle(turns * TAU)
    }

    /// Returns the radians value this struct wraps around.
    pub fn radians(&self) -> f32 {
        self.0
    }

    pub fn as_degrees(&self) -> f32 {
        self.radians().to_degrees()
    }

    pub fn as_turns(&self) -> f32 {
        self.radians() / TAU
    }

    /// Returns the amount of full turns in this angle.
    pub fn full_turns(&self) -> f32 {
        self.as_turns().trunc()
    }

    /// Returns the remainder when dividing this angle by TAU.
    pub fn remainder_of_turns(&self) -> Angle {
        Angle::from_turns(self.as_turns().fract())
    }

    /// Returns the minimum of the two angles.
    pub fn min(&self, other: Angle) -> Angle {
        Angle(self.0.min(other.0))
    }

    /// Returns the maximum of the two angles.
    pub fn max(&self, other: Angle) -> Angle {
        Angle(self.0.max(other.0))
    }

    /// Returns this angle clamped to the given range.
    pub fn clamp(&self, range: &Range<Angle>) -> Angle {
        Angle(self.0.clamp(range.start.0, range.end.0))
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Angle(self.0 + rhs.0)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_angle_from_different_units() {
        let test_value = 5.0;

        let from_radians = Angle(test_value);
        let from_degrees = Angle::from_degrees(test_value);
        let from_turns = Angle::from_turns(test_value);

        assert_eq!(test_value, from_radians.radians());
        assert_eq!(test_value.to_radians(), from_degrees.radians());
        assert_eq!(test_value * TAU, from_turns.radians());
    }

    #[test]
    fn can_initialize_trough_consts() {
        assert_eq!(Angle(PI), Angle::PI);
        assert_eq!(Angle(TAU), Angle::TAU);
        assert_eq!(Angle(FRAC_PI_2), Angle::QUARTER);
        assert_eq!(Angle(PI), Angle::HALF);
        assert_eq!(Angle(TAU), Angle::FULL);
        assert_eq!(Angle(0.0), Angle::ZERO);
    }

    #[test]
    fn can_get_angle_as_different_units() {
        let angle = Angle::HALF;

        assert_eq!(180.0, angle.as_degrees());
        assert_eq!(PI, angle.radians());
        assert_eq!(0.5, angle.as_turns());
    }

    #[test]
    fn can_get_amount_of_full_turns_in_angle() {
        let half_turn = Angle::from_turns(0.5);
        let full_turn = Angle::from_turns(1.0);
        let many_turns = Angle::from_turns(74.95);
        let negative_turns = Angle::from_turns(-20.5);

        assert_eq!(0.0, half_turn.full_turns());
        assert_eq!(1.0, full_turn.full_turns());
        assert_eq!(74.0, many_turns.full_turns());
        assert_eq!(-20.0, negative_turns.full_turns());
    }

    #[test]
    fn can_get_remainder_of_turns_in_angle() {
        let half_turn = Angle::from_turns(0.5);
        let many_turns = Angle::from_turns(32.75);
        let negative_turns = Angle::from_turns(-2.3);

        assert_eq!(Angle::from_turns(0.5), half_turn.remainder_of_turns());
        assert_eq!(Angle::from_turns(0.75), many_turns.remainder_of_turns());
        assert_eq!(Angle::from_turns(-0.3), negative_turns.remainder_of_turns());
    }
}
