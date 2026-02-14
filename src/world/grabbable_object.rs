use bevy::prelude::*;

/// Marker component for objects the player can grab.
#[derive(Component, Clone, Copy)]
pub struct GrabbableObject;

/// Stores the orientation relative to the player that this object should have when grabbed.
#[derive(Component, Clone, Copy)]
pub struct GrabOrientation {
    /// The current orientation.
    pub orientation: Quat,
    /// The default orientation to reset to.
    pub default_orientation: Quat,
}

impl GrabOrientation {
    pub const IDENTITY: Self = Self {
        orientation: Quat::IDENTITY,
        default_orientation: Quat::IDENTITY,
    };

    pub fn with_default_orientation(default_orientation: Quat) -> Self {
        Self {
            orientation: default_orientation,
            default_orientation,
        }
    }
}
