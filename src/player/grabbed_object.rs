pub mod object_anchor;
mod object_movement;

use avian3d::prelude::TransformInterpolation;
use bevy::prelude::*;
use object_anchor::{CalculatedAnchorValues, ObjectAnchor, ObjectAnchorPlugin};
use object_movement::GrabbedObjectMovementPlugin;

use crate::{
    utilities::{
        pd_controller::{PdController, config::PdControllerConfig},
        quaternion_pd_controller::QuaternionPdController,
        system_sets::InputSystems,
    },
    world::{
        character::Character, grabbable_object::GrabOrientation,
        interaction_target::PlayerInteractionTarget,
    },
};

pub struct GrabbedObjectPlugin;

impl Plugin for GrabbedObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GrabbedObjectMovementPlugin, ObjectAnchorPlugin))
            .add_systems(Update, grab_object_on_keypress.in_set(InputSystems))
            .add_observer(on_update_player_character_active)
            .add_observer(on_grab_object)
            .add_observer(on_drop_object);
    }
}

/// Holds data on the object held by the player.
#[derive(Component, Clone)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
    position_force_controller: PdController<Vec3>,
    rotation_force_controller: QuaternionPdController,
    // TODO: split anchor values to new component
    anchor_values: CalculatedAnchorValues,
    pub current_object_anchor: ObjectAnchor,
}

impl GrabbedObject {
    pub fn new(
        position_force_controller_config: PdControllerConfig,
        rotation_force_controller_config: PdControllerConfig,
    ) -> Self {
        Self {
            entity: None,
            position_force_controller: PdController::new(position_force_controller_config),
            rotation_force_controller: QuaternionPdController::new(
                rotation_force_controller_config,
            ),
            anchor_values: CalculatedAnchorValues::default(),
            current_object_anchor: ObjectAnchor::Default,
        }
    }

    fn current_anchor_value(&self) -> Isometry3d {
        self.anchor_values
            .get_from_object_anchor(self.current_object_anchor)
    }
}

#[derive(EntityEvent, Clone, Copy)]
struct GrabObject {
    entity: Entity,
}

#[derive(EntityEvent, Clone, Copy)]
struct DropObject {
    entity: Entity,
}

// TODO: either automatically update 'character active' value without needing an event, or rework event to 'set player active'. Current event is unclear in purpose and when to use
#[derive(EntityEvent, Copy, Clone)]
pub struct UpdatePlayerCharacterActive {
    pub entity: Entity,
}

fn on_update_player_character_active(
    update_player_character_active: On<UpdatePlayerCharacterActive>,
    mut characters_query: Query<&mut Character>,
    grabbed_object: Single<&GrabbedObject>,
) {
    let Ok(mut character) = characters_query.get_mut(update_player_character_active.entity) else {
        return;
    };

    character.is_active = grabbed_object.current_object_anchor != ObjectAnchor::Inspecting;
}

fn grab_object_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grabbed_object: Single<&GrabbedObject>,
    player_interaction_target: Res<PlayerInteractionTarget>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if let Some(entity) = grabbed_object.entity {
            commands.trigger(DropObject { entity });
        } else if let Some(target) = player_interaction_target.current_target() {
            commands.trigger(GrabObject {
                entity: target.entity,
            });
        }
    }
}

fn on_grab_object(
    event: On<GrabObject>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    grab_orientations: Query<&GrabOrientation>,
    mut commands: Commands,
) {
    let Ok(grab_orientation) = grab_orientations.get(event.entity) else {
        return;
    };

    grabbed_object.entity = Some(event.entity);

    // Add interpolation
    commands.entity(event.entity).insert(TransformInterpolation);

    // Set force controllers to new start values
    let target_isometry = grabbed_object.current_anchor_value();
    grabbed_object
        .position_force_controller
        .set_start_position(target_isometry.translation.into());
    grabbed_object
        .rotation_force_controller
        .set_start_position(target_isometry.rotation * grab_orientation.value());
}

fn on_drop_object(
    event: On<DropObject>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut commands: Commands,
) {
    grabbed_object.entity = None;

    // Remove interpolation
    commands
        .entity(event.entity)
        .remove::<TransformInterpolation>();
}
