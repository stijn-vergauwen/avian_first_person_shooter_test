use crate::utilities::angle::Angle;
use bevy::prelude::*;
use std::ops::{Add, AddAssign};

// TODO: add conversions to & from Quaterions

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct EulerAngle {
    /// The rotation around the X axis.
    pub x: Angle,
    /// The rotation around the Y axis.
    pub y: Angle,
    /// The rotation around the Z axis.
    pub z: Angle,
    pub order: EulerRot,
}

impl EulerAngle {
    pub fn new(x: Angle, y: Angle, z: Angle, order: EulerRot) -> Self {
        Self { x, y, z, order }
    }

    pub fn from_radians(x: f32, y: f32, z: f32, order: EulerRot) -> Self {
        Self::new(Angle(x), Angle(y), Angle(z), order)
    }
}

impl Add for EulerAngle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        EulerAngle::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.order)
    }
}

impl AddAssign for EulerAngle {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}
