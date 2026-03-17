mod object_movement;

use avian3d::prelude::TransformInterpolation;
use bevy::prelude::*;
use object_movement::GrabbedObjectMovementPlugin;

use crate::{
    utilities::{
        pd_controller::{PdController, config::PdControllerConfig},
        quaternion_pd_controller::QuaternionPdController,
        system_sets::InputSystems,
    },
    world::{
        character::CharacterHead, grabbable_object::GrabOrientation,
        interaction_target::PlayerInteractionTarget,
    },
};

const PRIMARY_HAND_OFFSET: Vec3 = Vec3::new(0.3, -0.15, -1.0);
const INSPECTING_OFFSET: Vec3 = Vec3::new(0.0, 0.0, -1.2);

pub struct GrabbedObjectPlugin;

impl Plugin for GrabbedObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GrabbedObjectMovementPlugin).insert_resource(GrabbedObject::new(
                PdControllerConfig::from_parameters(2.5, 1.0, 1.5),
                PdControllerConfig::from_parameters(2.0, 0.6, 1.0),
                PdControllerConfig::from_parameters(4.0, 1.2, 1.0),
            )).insert_resource(HoldPosition::PrimaryHand)
            .add_systems(Update, grab_object_on_keypress.in_set(InputSystems))
            .add_observer(on_grab_object)
            .add_observer(on_drop_object);
    }
}

/// Holds data on the object held by the player.
#[derive(Resource, Clone)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
    position_force_controller: PdController<Vec3>,
    rotation_force_controller: QuaternionPdController,
    position_controller_config: PdControllerConfig,
    rotation_controller_config: PdControllerConfig,
    ads_controller_config: PdControllerConfig,
}

impl GrabbedObject {
    pub fn new(
        position_force_controller_config: PdControllerConfig,
        rotation_force_controller_config: PdControllerConfig,
        ads_config: PdControllerConfig,
    ) -> Self {
        Self {
            entity: None,
            position_force_controller: PdController::new(position_force_controller_config),
            rotation_force_controller: QuaternionPdController::new(
                rotation_force_controller_config,
            ),
            position_controller_config: position_force_controller_config,
            rotation_controller_config: rotation_force_controller_config,
            ads_controller_config: ads_config,
        }
    }

    pub fn switch_controller_config(&mut self, use_ads_config: bool) {
        self.position_force_controller
            .set_config(match use_ads_config {
                true => self.ads_controller_config,
                false => self.position_controller_config,
            });

        self.rotation_force_controller
            .set_config(match use_ads_config {
                true => self.ads_controller_config,
                false => self.rotation_controller_config,
            });
    }
}

/// How the currently grabbed object should be held
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum HoldPosition {
    /// Hold object to right side of player.
    PrimaryHand,
    /// Hold object in front of player to inspect.
    Inspecting,
    /// Hold weapon in front of player to aim.
    AimDownSight,
}

#[derive(EntityEvent, Clone, Copy)]
struct GrabObject {
    entity: Entity,
}

#[derive(EntityEvent, Clone, Copy)]
struct DropObject {
    entity: Entity,
}

fn grab_object_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grabbed_object: Res<GrabbedObject>,
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
    grab_orientations: Query<&GrabOrientation>,
    player_head: Single<&GlobalTransform, With<CharacterHead>>,
    mut grabbed_object: ResMut<GrabbedObject>,
    mut hold_position: ResMut<HoldPosition>,
    mut commands: Commands,
) {
    let Ok(grab_orientation) = grab_orientations.get(event.entity) else {
        return;
    };

    grabbed_object.entity = Some(event.entity);

    // Add interpolation
    commands.entity(event.entity).insert(TransformInterpolation);

    // Reset hold position
    *hold_position = HoldPosition::PrimaryHand;

    // Set force controllers to new start values
    grabbed_object
        
        .position_force_controller
        .set_start_position(player_head.transform_point(PRIMARY_HAND_OFFSET));
    grabbed_object
        
        .rotation_force_controller
        .set_start_position(player_head.rotation() * grab_orientation.value());
}

fn on_drop_object(
    event: On<DropObject>,
    mut grabbed_object: ResMut<GrabbedObject>,
    mut commands: Commands,
) {
    grabbed_object.entity = None;

    // Remove interpolation
    commands
        .entity(event.entity)
        .remove::<TransformInterpolation>();
}
