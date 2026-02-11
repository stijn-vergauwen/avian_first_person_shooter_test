use avian3d::prelude::{Forces, RigidBodyForces};
use bevy::{
    color::palettes::tailwind::{GREEN_300, PURPLE_400},
    prelude::*,
    window::{CursorGrabMode, CursorIcon, CursorOptions, PrimaryWindow, SystemCursorIcon},
};

use crate::{
    player::Player,
    utilities::{
        DrawGizmos,
        pd_controller::{PdController, config::PdControllerConfig},
        quaternion_pd_controller::QuaternionPdController,
        system_sets::{DataSystems, DisplaySystems, InputSystems},
    },
    world::{
        character::Character,
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
                update_grabbed_object_position,
                update_grabbed_object_rotation,
            )
                .in_set(DataSystems::UpdateEntities),
        )
        .add_observer(on_update_player_character_active)
        .add_observer(
            |event: On<Pointer<Over>>,
             grabbable_objects: Query<(), With<GrabbableObject>>,
             grabbed_object: Single<&GrabbedObject>,
             mut window_cursor: Single<&mut CursorIcon>| {
                if grabbed_object.is_inspecting && grabbable_objects.contains(event.entity) {
                    **window_cursor = CursorIcon::System(SystemCursorIcon::Pointer);
                }
            },
        )
        .add_observer(
            |event: On<Pointer<Out>>,
             grabbable_objects: Query<(), With<GrabbableObject>>,
             grabbed_object: Single<&GrabbedObject>,
             mut window_cursor: Single<&mut CursorIcon>| {
                if grabbed_object.is_inspecting && grabbable_objects.contains(event.entity) {
                    **window_cursor = CursorIcon::System(SystemCursorIcon::Default);
                }
            },
        )
        .add_observer(
            |event: On<Pointer<Press>>,
             mesh_materials: Query<&MeshMaterial3d<StandardMaterial>, With<GrabbableObject>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             grabbed_object: Single<&GrabbedObject>| {
                println!("Press event received");

                if !grabbed_object.is_inspecting {
                    println!("Not inspecting, return.");
                    return;
                }

                if let Ok(mesh_material) = mesh_materials.get(event.entity) {
                    println!("Press event mesh material found");

                    let material = materials.get_mut(mesh_material.clone()).unwrap();

                    material.base_color = Color::from(GREEN_300);
                }
            },
        )
        .add_observer(
            |event: On<Pointer<Drag>>,
             mut grab_orientations: Query<&mut GrabOrientation, With<GrabbableObject>>,
             grabbed_object: Single<&GrabbedObject>| {
                if !grabbed_object.is_inspecting {
                    return;
                }

                if let Ok(mut grab_orientation) = grab_orientations.get_mut(event.entity) {
                    const PIXELS_PER_RADIAN: f32 = 200f32;

                    let horizontal_rotation =
                        Quat::from_axis_angle(Vec3::Y, event.delta.x / PIXELS_PER_RADIAN);
                    let vertical_rotation =
                        Quat::from_axis_angle(Vec3::X, event.delta.y / PIXELS_PER_RADIAN);

                    grab_orientation.orientation =
                        horizontal_rotation * grab_orientation.orientation;
                    grab_orientation.orientation = vertical_rotation * grab_orientation.orientation;
                }
            },
        );
    }
}

/// Holds data on the object held by the player.
#[derive(Component, Clone)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
    position_force_controller: PdController<Vec3>,
    rotation_force_controller: QuaternionPdController,
    is_inspecting: bool,
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
            is_inspecting: false,
        }
    }
}

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

    character.is_active = !grabbed_object.is_inspecting;
}

fn grab_object_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grabbed_object: Single<(&mut GrabbedObject, &GlobalTransform)>,
    player_interaction_target: Res<PlayerInteractionTarget>,
    grabbable_query: Query<Option<&GrabOrientation>, With<GrabbableObject>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        grabbed_object.0.entity = None;

        if let Some(target) = player_interaction_target.current_target()
            && grabbable_query.contains(target.entity)
        {
            grabbed_object.0.entity = Some(target.entity);

            let global_transform = grabbed_object.1;
            let grab_orientation = grabbable_query
                .get(target.entity)
                .unwrap_or(None)
                .map_or(Quat::IDENTITY, |component| component.orientation);

            grabbed_object
                .0
                .position_force_controller
                .set_start_position(global_transform.translation());
            grabbed_object
                .0
                .rotation_force_controller
                .set_start_position(global_transform.rotation() * grab_orientation);
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_grabbed_object_position(
    mut grabbed_object: Single<(&mut GrabbedObject, &GlobalTransform)>,
    mut player: Single<Forces, With<Player>>,
    mut target_item_query: Query<
        (&GlobalTransform, Forces),
        (Without<GrabbedObject>, Without<Player>),
    >,
    time: Res<Time>,
) {
    let Some(target_item_entity) = grabbed_object.0.entity else {
        return;
    };

    let mut target_item = target_item_query.get_mut(target_item_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let target_position = grabbed_object.1.translation();
    let position_controller = &mut grabbed_object.0.position_force_controller;

    position_controller.set_target_position(target_position);
    position_controller.set_position(target_item.0.translation());
    position_controller.set_velocity(target_item.1.linear_velocity());
    position_controller.update(time.delta_secs());

    // Apply position force to grabbed object
    target_item
        .1
        .apply_force(position_controller.acceleration());

    // Apply opposite position force to player
    player.apply_force(-position_controller.acceleration());
}

#[allow(clippy::type_complexity)]
fn update_grabbed_object_rotation(
    mut grabbed_object: Single<(&mut GrabbedObject, &GlobalTransform)>,
    mut target_item_query: Query<(&GlobalTransform, Forces), Without<GrabbedObject>>,
    grab_orientation_query: Query<&GrabOrientation, With<GrabbableObject>>,
    time: Res<Time>,
) {
    let Some(target_item_entity) = grabbed_object.0.entity else {
        return;
    };

    let mut target_item = target_item_query.get_mut(target_item_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let player_rotation = grabbed_object.1.rotation();
    let rotation_controller = &mut grabbed_object.0.rotation_force_controller;

    let grab_orientation = grab_orientation_query
        .get(target_item_entity)
        .map_or(Quat::IDENTITY, |component| component.orientation);

    rotation_controller.set_target_position(player_rotation * grab_orientation);

    let new_acceleration = rotation_controller.update_from_physics_sim(
        target_item.0.rotation(),
        target_item.1.angular_velocity(),
        time.delta_secs(),
    );

    target_item.1.apply_angular_acceleration(new_acceleration);
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
