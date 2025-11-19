mod cursor_lock;
mod item_anchor;
mod spawner;

use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::*, ecs::relationship::RelationshipSourceCollection,
    input::mouse::AccumulatedMouseMotion, prelude::*,
};

use crate::{
    player::{
        cursor_lock::CursorLockPlugin,
        item_anchor::{ItemAnchor, ItemAnchorPlugin},
        spawner::PlayerSpawnerPlugin,
    },
    utilities::{
        euler_angle::EulerAngle,
        fraction::Fraction,
        system_sets::{DisplaySystems, InputSystems},
    },
    world::{
        desired_movement::{DesiredMovement, SetDesiredMovement},
        desired_rotation::{DesiredRotation, RotationType, SetDesiredRotation},
        grabbable_object::GrabbableObject,
    },
};

const MOVEMENT_KEYBINDS: MovementKeybinds = MovementKeybinds {
    forward_key: KeyCode::KeyW,
    back_key: KeyCode::KeyS,
    left_key: KeyCode::KeyA,
    right_key: KeyCode::KeyD,
};

/// Upper threshold for delta mouse motion in a single update, this is to ignore motion spikes caused by input through Parsec.
const UPPER_MOUSE_MOTION_THRESHOLD: f32 = 1000.0;

/// Mouse sensitivity calculated as: how many pixels the mouse needs to move in a direction to rotate by 1 radian in that direction.
/// - Higher value = less sensitive.
const PIXELS_PER_RADIAN: f32 = 600f32;

const MAX_GRAB_DISTANCE: f32 = 2.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CursorLockPlugin, PlayerSpawnerPlugin, ItemAnchorPlugin))
            .add_systems(Startup, spawn_crosshair)
            .add_systems(
                Update,
                (
                    (
                        handle_movement_input,
                        handle_rotation_input,
                        set_item_anchor_target_on_keypress,
                    )
                        .in_set(InputSystems),
                    (draw_player_gizmos, update_crosshair_color).in_set(DisplaySystems),
                ),
            );
    }
}

/// Marker component for the player. Only 1 player should be spawned.
#[derive(Component, Clone, Copy)]
pub struct Player;

/// Marker component for the player body mesh & collider.
#[derive(Component, Clone, Copy)]
pub struct PlayerBody;

/// Marker component for the player camera.
#[derive(Component, Clone, Copy)]
struct PlayerCamera;

#[derive(Copy, Clone)]
pub struct MovementKeybinds {
    pub forward_key: KeyCode,
    pub back_key: KeyCode,
    pub left_key: KeyCode,
    pub right_key: KeyCode,
}

fn handle_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_entity: Single<Entity, With<Player>>,
    mut previous_input: Local<Option<DesiredMovement>>,
    mut commands: Commands,
) {
    let move_direction = move_direction_from_input(MOVEMENT_KEYBINDS, &keyboard_input);

    let desired_movement = move_direction.map(|direction| DesiredMovement {
        direction,
        fraction_of_max_strength: Fraction::new_unchecked(1.0),
    });

    if desired_movement != *previous_input {
        *previous_input = desired_movement;

        commands.trigger(SetDesiredMovement {
            entity: *player_entity,
            desired_movement,
        });
    }
}

fn handle_rotation_input(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player_entity: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let Some(desired_rotation) = calculate_desired_rotation(accumulated_mouse_motion.delta) else {
        return;
    };

    commands.trigger(SetDesiredRotation {
        entity: *player_entity,
        desired_rotation,
    });
}

fn calculate_desired_rotation(delta_motion: Vec2) -> Option<DesiredRotation> {
    if delta_motion.length() > UPPER_MOUSE_MOTION_THRESHOLD {
        println!("Mouse motion above threshold!");
    }

    (delta_motion.length() > 0.0 && delta_motion.length() < UPPER_MOUSE_MOTION_THRESHOLD).then(
        || {
            let delta_rotation = EulerAngle::from_radians(
                -delta_motion.y / PIXELS_PER_RADIAN,
                -delta_motion.x / PIXELS_PER_RADIAN,
                0.0,
                EulerRot::default(),
            );

            DesiredRotation {
                rotation: delta_rotation,
                rotation_type: RotationType::DeltaRotation,
            }
        },
    )
}

fn set_item_anchor_target_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut item_anchor: Single<&mut ItemAnchor>,
    player_camera: Single<&GlobalTransform, With<PlayerCamera>>,
    spatial_query: SpatialQuery,
    player_body_entity: Single<Entity, With<PlayerBody>>,
    grabbable_query: Query<&GrabbableObject>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        item_anchor.target_item_entity = None;

        let origin = player_camera.translation();
        let direction = player_camera.forward();

        if let Some(hit_data) = spatial_query.cast_ray(
            origin,
            direction,
            MAX_GRAB_DISTANCE,
            false,
            &SpatialQueryFilter::from_excluded_entities(player_body_entity.iter()),
        ) && grabbable_query.contains(hit_data.entity)
        {
            item_anchor.target_item_entity = Some(hit_data.entity)
        };
    }
}

#[allow(unused)]
fn draw_player_gizmos(
    tool_anchor: Single<&GlobalTransform, With<ItemAnchor>>,
    player_camera: Single<&GlobalTransform, (With<PlayerCamera>, Without<ItemAnchor>)>,
    mut gizmos: Gizmos,
) {
    // Item anchor
    gizmos.sphere(
        tool_anchor.compute_transform().to_isometry(),
        0.2,
        PURPLE_400,
    );

    // Player camera
    gizmos.ray(
        player_camera.translation(),
        player_camera.forward() * 10.0,
        LIME_400,
    );
}

// Crosshair for grabbable objects
// TODO: move to module

/// Marker component grabbable object crosshair.
#[derive(Component)]
struct Crosshair;

fn spawn_crosshair(mut commands: Commands) {
    const RADIUS: f32 = 16.0;
    const THICKNESS: f32 = 3.0;

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Crosshair,
                Node {
                    width: Val::Px(RADIUS),
                    height: Val::Px(RADIUS),
                    border: UiRect::all(Val::Px(THICKNESS)),
                    ..default()
                },
                BorderRadius::all(Val::Percent(50.0)),
                BackgroundColor(Color::NONE),
                BorderColor::all(NEUTRAL_400),
            ));
        });
}

fn update_crosshair_color(
    player_camera: Single<&GlobalTransform, With<PlayerCamera>>,
    player_body_entity: Single<Entity, With<PlayerBody>>,
    spatial_query: SpatialQuery,
    grabbable_query: Query<&GrabbableObject>,
    mut crosshair_color: Single<&mut BorderColor, With<Crosshair>>,
) {
    let origin = player_camera.translation();
    let direction = player_camera.forward();

    let new_color = match spatial_query.cast_ray(
        origin,
        direction,
        MAX_GRAB_DISTANCE,
        false,
        &SpatialQueryFilter::from_excluded_entities(player_body_entity.iter()),
    ) {
        Some(hit_data) if grabbable_query.contains(hit_data.entity) => Color::Srgba(TEAL_400),
        _ => Color::Srgba(NEUTRAL_400),
    };

    if crosshair_color.top != new_color {
        crosshair_color.set_all(new_color);
    }
}

// Utilities

fn move_direction_from_input(
    keybinds: MovementKeybinds,
    input: &ButtonInput<KeyCode>,
) -> Option<Dir3> {
    let mut direction = Vec3::ZERO;

    if input.pressed(keybinds.forward_key) {
        direction.z -= 1.0;
    }

    if input.pressed(keybinds.back_key) {
        direction.z += 1.0;
    }

    if input.pressed(keybinds.left_key) {
        direction.x -= 1.0;
    }

    if input.pressed(keybinds.right_key) {
        direction.x += 1.0;
    }

    Dir3::new(direction).ok()
}
