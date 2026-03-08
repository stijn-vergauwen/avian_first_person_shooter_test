use bevy::prelude::*;

pub struct GrabbableObjectPlugin;

impl Plugin for GrabbableObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(set_orientation_when_default_grab_orientation_inserted);
    }
}

/// Marker component for objects the player can grab.
#[derive(Component, Default, Clone, Copy)]
#[require(GrabOrientation)]
pub struct GrabbableObject;

/// Stores the orientation relative to the player that this object should have when grabbed.
#[derive(Component, Default, Clone, Copy)]
pub struct GrabOrientation(pub Quat);

impl GrabOrientation {
    pub fn value(&self) -> Quat {
        self.0
    }
}

/// Stores the default grab orientation that this object should reset to.
#[derive(Component, Clone, Copy)]
#[require(GrabbableObject, GrabOrientation)]
pub struct DefaultGrabOrientation(pub Quat);

impl DefaultGrabOrientation {
    pub fn value(&self) -> Quat {
        self.0
    }
}

fn set_orientation_when_default_grab_orientation_inserted(
    event: On<Insert, DefaultGrabOrientation>,
    mut grab_orientations: Query<(&mut GrabOrientation, &DefaultGrabOrientation)>,
) {
    let (mut orientation, default) = grab_orientations
    .get_mut(event.entity)
    .expect("Entities with DefaultGrabOrientation component should always have GrabbableObject component.");

    orientation.0 = default.0;
}
