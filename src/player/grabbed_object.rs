mod inspector_mode;

use avian3d::prelude::{Forces, RigidBodyForces};
use bevy::{color::palettes::tailwind::PURPLE_400, prelude::*};

use crate::{
    player::{Player, PlayerCamera, grabbed_object::inspector_mode::InspectorModePlugin},
    utilities::{
        DrawGizmos,
        pd_controller::{PdController, config::PdControllerConfig},
        quaternion_pd_controller::QuaternionPdController,
        system_sets::{DataSystems, DisplaySystems, InputSystems},
    },
    world::{
        character::{Character, CharacterHead},
        grabbable_object::{GrabOrientation, GrabbableObject},
        interaction_target::PlayerInteractionTarget,
    },
};

pub struct GrabbedObjectPlugin;

impl Plugin for GrabbedObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorModePlugin)
            .add_systems(
                Update,
                (
                    grab_object_on_keypress.in_set(InputSystems),
                    draw_grabbed_object_anchor_position.in_set(DisplaySystems),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    update_anchor_positions.in_set(DataSystems::PrepareData),
                    (
                        update_grabbed_object_position,
                        update_grabbed_object_rotation,
                    )
                        .in_set(DataSystems::UpdateEntities),
                ),
            )
            .add_observer(on_update_player_character_active)
            .add_observer(on_grab_object)
            .add_observer(on_drop_object);
    }
}

// TODO: store offsets in separate resource
// TODO: add anchor_positions field that is a struct with all the anchor positions
// TODO: add enum for each anchor position / offset
// TODO: add field of enum type that stores the current anchor position being used

/// Holds data on the object held by the player.
#[derive(Component, Clone)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
    position_force_controller: PdController<Vec3>,
    rotation_force_controller: QuaternionPdController,
    offset_in_front_of_player_head: Vec3,
    position_in_front_of_player_head: Isometry3d,
    offset_in_right_hand: Vec3,
    position_in_right_hand: Isometry3d,
    is_inspecting: bool,
    pub is_aiming: bool,
}

impl GrabbedObject {
    pub fn new(
        position_force_controller_config: PdControllerConfig,
        rotation_force_controller_config: PdControllerConfig,
        offset_in_front_of_player_head: Vec3,
        offset_in_right_hand: Vec3,
    ) -> Self {
        Self {
            entity: None,
            position_force_controller: PdController::new(position_force_controller_config),
            rotation_force_controller: QuaternionPdController::new(
                rotation_force_controller_config,
            ),
            offset_in_front_of_player_head,
            offset_in_right_hand,
            position_in_front_of_player_head: Isometry3d::default(),
            position_in_right_hand: Isometry3d::default(),
            is_inspecting: false,
            is_aiming: false,
        }
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

#[derive(EntityEvent, Copy, Clone)]
pub struct UpdatePlayerCharacterActive {
    pub entity: Entity,
}

fn update_anchor_positions(
    mut grabbed_object: Single<&mut GrabbedObject>,
    player_head: Single<&GlobalTransform, With<CharacterHead>>,
    player_camera: Single<&GlobalTransform, With<PlayerCamera>>,
) {
    grabbed_object.position_in_right_hand =
        calculate_anchor_position(&player_head, grabbed_object.offset_in_right_hand);

    grabbed_object.position_in_front_of_player_head = calculate_anchor_position(
        &player_camera,
        grabbed_object.offset_in_front_of_player_head,
    );
}

fn on_update_player_character_active(
    update_player_character_active: On<UpdatePlayerCharacterActive>,
    mut characters_query: Query<&mut Character>,
    grabbed_object: Single<&GrabbedObject>,
) {
    let Ok(mut character) = characters_query.get_mut(update_player_character_active.entity) else {
        return;
    };

    character.is_active = !grabbed_object.is_inspecting;
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
        }

        if let Some(target) = player_interaction_target.current_target() {
            commands.trigger(GrabObject {
                entity: target.entity,
            });
        }
    }
}

fn on_grab_object(
    event: On<GrabObject>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    grabbable_query: Query<&GrabOrientation, With<GrabbableObject>>,
) {
    if !grabbable_query.contains(event.entity) {
        return;
    }

    grabbed_object.entity = Some(event.entity);

    let grab_orientation = grabbable_query
        .get(event.entity)
        .map_or(Quat::IDENTITY, |component| component.orientation);

    let target_isometry = grabbed_object.position_in_right_hand;
    grabbed_object
        .position_force_controller
        .set_start_position(target_isometry.translation.into());
    grabbed_object
        .rotation_force_controller
        .set_start_position(target_isometry.rotation * grab_orientation);
}

fn on_drop_object(_: On<DropObject>, mut grabbed_object: Single<&mut GrabbedObject>) {
    grabbed_object.entity = None;
}

fn update_grabbed_object_position(
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut target_item_query: Query<(&GlobalTransform, Forces), Without<Player>>,
    time: Res<Time>,
    mut player: Single<Forces, With<Player>>,
) {
    let Some(target_item_entity) = grabbed_object.entity else {
        return;
    };

    let mut target_item = target_item_query.get_mut(target_item_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let target_position = if grabbed_object.is_inspecting {
        grabbed_object
            .position_in_front_of_player_head
            .translation
            .to_vec3()
    } else if grabbed_object.is_aiming {
        // TODO: replace this hardcoded thing with something more configurable
        grabbed_object
            .position_in_front_of_player_head
            .translation
            .to_vec3()
            + grabbed_object.position_in_front_of_player_head.rotation * Vec3::new(0.03, -0.05, 0.9)
    } else {
        grabbed_object.position_in_right_hand.translation.to_vec3()
    };

    grabbed_object
        .position_force_controller
        .set_target_position(target_position);

    let new_acceleration = grabbed_object
        .position_force_controller
        .update_from_physics_sim(
            target_item.0.translation(),
            target_item.1.linear_velocity(),
            time.delta_secs(),
        );

    // Apply position force to grabbed object
    target_item.1.apply_force(new_acceleration);

    // Apply opposite position force to player
    player.apply_force(-new_acceleration);
}

fn update_grabbed_object_rotation(
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut grabbable_object_query: Query<
        (&GlobalTransform, Forces, Option<&GrabOrientation>),
        With<GrabbableObject>,
    >,
    time: Res<Time>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let mut grabbable_object = grabbable_object_query.get_mut(grabbed_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let player_rotation = grabbed_object.position_in_right_hand.rotation;
    let grab_orientation = grabbable_object
        .2
        .map_or(Quat::IDENTITY, |component| component.orientation);

    grabbed_object
        .rotation_force_controller
        .set_target_position(player_rotation * grab_orientation);

    let new_acceleration = grabbed_object
        .rotation_force_controller
        .update_from_physics_sim(
            grabbable_object.0.rotation(),
            grabbable_object.1.angular_velocity(),
            time.delta_secs(),
        );

    grabbable_object
        .1
        .apply_angular_acceleration(new_acceleration);
}

// Gizmos

fn draw_grabbed_object_anchor_position(
    grabbed_object: Single<&GlobalTransform, (With<GrabbedObject>, With<DrawGizmos>)>,
    mut gizmos: Gizmos,
) {
    gizmos.sphere(
        grabbed_object.compute_transform().to_isometry(),
        0.2,
        PURPLE_400,
    );
}

// Utilities

fn calculate_anchor_position(
    global_transform: &GlobalTransform,
    grabbed_object_offset: Vec3,
) -> Isometry3d {
    Isometry3d {
        translation: (global_transform.translation()
            + global_transform.rotation() * grabbed_object_offset)
            .into(),
        rotation: global_transform.rotation(),
    }
}
