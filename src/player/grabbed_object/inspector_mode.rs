use bevy::{
    color::palettes::tailwind::SKY_600,
    prelude::*,
    window::{CursorGrabMode, CursorIcon, CursorOptions, PrimaryWindow, SystemCursorIcon},
};

use crate::{
    player::{
        Player,
        grabbed_object::{GrabbedObject, UpdatePlayerCharacterActive},
    },
    utilities::system_sets::InputSystems,
    world::grabbable_object::{GrabOrientation, GrabbableObject},
};

pub struct InspectorModePlugin;

impl Plugin for InspectorModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_reset_to_default_orientation_button)
            .add_systems(
                Update,
                toggle_object_inspection_on_keypress.in_set(InputSystems),
            )
            .add_observer(show_pointer_when_over_grabbed_object)
            .add_observer(reset_cursor_when_leaving_grabbed_object)
            .add_observer(rotate_grabbed_object_on_drag);
    }
}

// TODO: make event for switching inspection mode
fn toggle_object_inspection_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut reset_orientation_button_visibility: Single<&mut Visibility, With<ResetOrientationButton>>,
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

        **reset_orientation_button_visibility = if grabbed_object.is_inspecting {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        commands.trigger(UpdatePlayerCharacterActive {
            entity: *player_entity,
        });
    }
}
// TODO: find how to handle 'pointer over' & 'pointer out' in 1 function
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

// UI

/// Marker component.
#[derive(Component, Clone, Copy)]
struct ResetOrientationButton;

fn spawn_reset_to_default_orientation_button(mut commands: Commands) {
    commands
        .spawn((
            ResetOrientationButton,
            Button,
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::from(SKY_600)),
        ))
        .with_child(Text::new("Reset orientation"))
        // TODO: split to fn
        .observe(
            |_: On<Pointer<Click>>,
             mut grab_orientations: Query<&mut GrabOrientation, With<GrabbableObject>>,
             grabbed_object: Single<&GrabbedObject>| {
                let mut grab_orientation = grab_orientations
                    .get_mut(grabbed_object.entity.unwrap())
                    .unwrap();
                grab_orientation.orientation = grab_orientation.default_orientation;
            },
        );
}
