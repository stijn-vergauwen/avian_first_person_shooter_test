use bevy::prelude::*;

pub trait ConstrainValue {
    fn constrain(self, max: f32) -> Self;
}

impl ConstrainValue for f32 {
    fn constrain(self, max: f32) -> Self {
        self.clamp(-max, max)
    }
}

impl ConstrainValue for Vec2 {
    fn constrain(self, max: f32) -> Self {
        self.clamp_length_max(max)
    }
}

impl ConstrainValue for Vec3 {
    fn constrain(self, max: f32) -> Self {
        self.clamp_length_max(max)
    }
}
