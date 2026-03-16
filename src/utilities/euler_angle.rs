use crate::utilities::angle::Angle;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Mul, Sub};

// TODO: add conversions to & from Quaterions

#[derive(Copy, Clone, Default, Debug, PartialEq, Reflect, Deserialize, Serialize)]
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

    pub fn to_quat(&self) -> Quat {
        let as_tuple = match self.order {
            EulerRot::ZYX => (self.z, self.y, self.x),
            EulerRot::ZXY => (self.z, self.x, self.y),
            EulerRot::YXZ => (self.y, self.x, self.z),
            EulerRot::YZX => (self.y, self.z, self.x),
            EulerRot::XYZ => (self.x, self.y, self.z),
            EulerRot::XZY => (self.x, self.z, self.y),
            _ => panic!("EulerRot type not supported!"),
        };

        Quat::from_euler(
            self.order,
            as_tuple.0.radians(),
            as_tuple.1.radians(),
            as_tuple.2.radians(),
        )
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

impl Sub for EulerAngle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        EulerAngle::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.order)
    }
}

impl Mul<f32> for EulerAngle {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        EulerAngle::new(self.x * rhs, self.y * rhs, self.z * rhs, self.order)
    }
}

impl Div<f32> for EulerAngle {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        EulerAngle::new(self.x / rhs, self.y / rhs, self.z / rhs, self.order)
    }
}
