use avian3d::prelude::{Forces, RigidBodyForces};
use bevy::{
    color::palettes::tailwind::PURPLE_400,
    prelude::*,
    window::{CursorGrabMode, CursorIcon, CursorOptions, PrimaryWindow, SystemCursorIcon},
};

use crate::{
    player::{Player, PlayerCamera},
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
        weapons::{ShootWeapon, Weapon},
    },
};

pub struct GrabbedObjectPlugin;

impl Plugin for GrabbedObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    grab_object_on_keypress,
                    shoot_held_weapon,
                    toggle_object_inspection_on_keypress,
                )
                    .in_set(InputSystems),
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
        .add_observer(show_pointer_when_over_grabbed_object)
        .add_observer(reset_cursor_when_leaving_grabbed_object)
        .add_observer(rotate_grabbed_object_on_drag);
    }
}

/// Holds data on the object held by the player.
#[derive(Component, Clone)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
    position_force_controller: PdController<Vec3>,
    rotation_force_controller: QuaternionPdController,
    is_inspecting: bool,
    offset_in_front_of_player_head: Vec3,
    position_in_front_of_player_head: Isometry3d,
    offset_in_right_hand: Vec3,
    position_in_right_hand: Isometry3d,
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
            is_inspecting: false,
            offset_in_front_of_player_head,
            offset_in_right_hand,
            position_in_front_of_player_head: Isometry3d::default(),
            position_in_right_hand: Isometry3d::default(),
        }
    }
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
    let player_head_rotation = player_head.rotation();
    grabbed_object.position_in_right_hand = Isometry3d {
        translation: (player_head.translation()
            + player_head_rotation * grabbed_object.offset_in_right_hand)
            .into(),
        rotation: player_head_rotation,
    };

    let player_camera_rotation = player_camera.rotation();
    grabbed_object.position_in_front_of_player_head = Isometry3d {
        translation: (player_camera.translation()
            + player_camera_rotation * grabbed_object.offset_in_front_of_player_head)
            .into(),
        rotation: player_camera_rotation,
    };
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
    mut grabbed_object: Single<&mut GrabbedObject>,
    player_interaction_target: Res<PlayerInteractionTarget>,
    grabbable_query: Query<Option<&GrabOrientation>, With<GrabbableObject>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        grabbed_object.entity = None;

        if let Some(target) = player_interaction_target.current_target()
            && grabbable_query.contains(target.entity)
        {
            grabbed_object.entity = Some(target.entity);

            let grab_orientation = grabbable_query
                .get(target.entity)
                .unwrap_or(None)
                .map_or(Quat::IDENTITY, |component| component.orientation);

            let target_isometry = grabbed_object.position_in_right_hand;
            grabbed_object
                .position_force_controller
                .set_start_position(target_isometry.translation.into());
            grabbed_object
                .rotation_force_controller
                .set_start_position(target_isometry.rotation * grab_orientation);
        }
    }
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
    } else {
        grabbed_object.position_in_right_hand.translation.to_vec3()
    };

    let position_controller = &mut grabbed_object.position_force_controller;

    position_controller.set_target_position(target_position);
    position_controller.update_from_physics_sim(
        target_item.0.translation(),
        target_item.1.linear_velocity(),
        time.delta_secs(),
    );

    // Apply position force to grabbed object
    target_item
        .1
        .apply_force(position_controller.acceleration());

    // Apply opposite position force to player
    player.apply_force(-position_controller.acceleration());
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
    let rotation_controller = &mut grabbed_object.rotation_force_controller;

    let grab_orientation = grabbable_object
        .2
        .map_or(Quat::IDENTITY, |component| component.orientation);

    rotation_controller.set_target_position(player_rotation * grab_orientation);

    let new_acceleration = rotation_controller.update_from_physics_sim(
        grabbable_object.0.rotation(),
        grabbable_object.1.angular_velocity(),
        time.delta_secs(),
    );

    grabbable_object
        .1
        .apply_angular_acceleration(new_acceleration);
}

fn shoot_held_weapon(
    mouse_input: Res<ButtonInput<MouseButton>>,
    grabbed_object: Single<&GrabbedObject>,
    weapons_query: Query<&Weapon>,
    mut commands: Commands,
) {
    if mouse_input.just_pressed(MouseButton::Left)
        && let Some(grabbed_entity) = grabbed_object.entity
        && weapons_query.contains(grabbed_entity)
    {
        commands.trigger(ShootWeapon {
            entity: grabbed_entity,
        });
    };
}

// Object inspecting

fn toggle_object_inspection_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut commands: Commands,
    player_entity: Single<Entity, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT)
        || grabbed_object.is_inspecting && keyboard_input.just_pressed(KeyCode::Escape)
    {
        grabbed_object.is_inspecting = !grabbed_object.is_inspecting;

        cursor_options.visible = grabbed_object.is_inspecting;
        cursor_options.grab_mode = if grabbed_object.is_inspecting {
            CursorGrabMode::None
        } else {
            CursorGrabMode::Locked
        };

        commands.trigger(UpdatePlayerCharacterActive {
            entity: *player_entity,
        });
    }
}

fn show_pointer_when_over_grabbed_object(
    event: On<Pointer<Over>>,
    grabbed_object: Single<&GrabbedObject>,
    mut window_cursor: Single<&mut CursorIcon>,
) {
    if grabbed_object.is_inspecting && grabbed_object.entity == Some(event.entity) {
        **window_cursor = CursorIcon::System(SystemCursorIcon::Pointer);
    }
}

fn reset_cursor_when_leaving_grabbed_object(
    event: On<Pointer<Out>>,
    grabbed_object: Single<&GrabbedObject>,
    mut window_cursor: Single<&mut CursorIcon>,
) {
    if grabbed_object.is_inspecting && grabbed_object.entity == Some(event.entity) {
        **window_cursor = CursorIcon::System(SystemCursorIcon::Default);
    }
}

fn rotate_grabbed_object_on_drag(
    event: On<Pointer<Drag>>,
    mut grab_orientations: Query<&mut GrabOrientation, With<GrabbableObject>>,
    grabbed_object: Single<&GrabbedObject>,
) {
    if !(grabbed_object.is_inspecting && grabbed_object.entity == Some(event.entity)) {
        return;
    }

    if let Ok(mut grab_orientation) = grab_orientations.get_mut(event.entity) {
        const PIXELS_PER_RADIAN: f32 = 150f32;

        let horizontal_rotation = Quat::from_axis_angle(Vec3::Y, event.delta.x / PIXELS_PER_RADIAN);
        let vertical_rotation = Quat::from_axis_angle(Vec3::X, event.delta.y / PIXELS_PER_RADIAN);

        grab_orientation.orientation = horizontal_rotation * grab_orientation.orientation;
        grab_orientation.orientation = vertical_rotation * grab_orientation.orientation;
    }
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
