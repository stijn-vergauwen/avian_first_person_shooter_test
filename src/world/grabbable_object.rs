use bevy::prelude::*;

/// Marker component for objects the player can grab.
#[derive(Component, Clone, Copy)]
pub struct GrabbableObject;

/// Stores the orientation relative to the player that this object should have when grabbed.
#[derive(Component, Clone, Copy)]
pub struct GrabOrientation {
    pub orientation: Quat,
}

impl GrabOrientation {
    pub fn new(orientation: Quat) -> Self {
        Self { orientation }
    }
}