use bevy::prelude::*;

pub struct GrabbableObjectPlugin;

impl Plugin for GrabbableObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(set_orientation_when_default_grab_orientation_inserted);
    }
}

/// Marker component for objects the player can grab.
#[derive(Component, Default, Clone, Copy)]
pub struct GrabbableObject {
    /// Relative orientation of object when grabbed.
    pub orientation: Quat,
}

impl GrabbableObject {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Stores the default orientation relative to the player that this object should have when grabbed.
#[derive(Component, Clone, Copy)]
#[require(GrabbableObject)]
pub struct DefaultGrabOrientation(pub Quat);

impl DefaultGrabOrientation {
    pub fn value(&self) -> Quat {
        self.0
    }
}

fn set_orientation_when_default_grab_orientation_inserted(
    event: On<Insert, DefaultGrabOrientation>,
    mut grabbable_objects: Query<(&mut GrabbableObject, &DefaultGrabOrientation)>,
) {
    let (mut grabbable_object, grab_orientation) = grabbable_objects
    .get_mut(event.entity)
    .expect("Entities with DefaultGrabOrientation component should always have GrabbableObject component.");

    grabbable_object.orientation = grab_orientation.0;
}
