mod inspector_mode_ui;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorIcon, CursorOptions, PrimaryWindow, SystemCursorIcon},
};
use inspector_mode_ui::InspectorModeUiPlugin;

use crate::{
    player::{
        Player,
        grabbed_object::{GrabbedObject, UpdatePlayerCharacterActive, object_anchor::ObjectAnchor},
    },
    utilities::system_sets::InputSystems,
    world::grabbable_object::GrabOrientation,
};

pub struct InspectorModePlugin;

impl Plugin for InspectorModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorModeUiPlugin)
            .add_systems(
                Update,
                toggle_inspector_mode_on_keypress.in_set(InputSystems),
            )
            .add_observer(on_toggle_inspector_mode)
            .add_observer(set_cursor_icon_on_pointer_event::<Over>(
                SystemCursorIcon::Pointer,
            ))
            .add_observer(set_cursor_icon_on_pointer_event::<Out>(
                SystemCursorIcon::Default,
            ))
            .add_observer(rotate_grabbed_object_on_drag);
    }
}

#[derive(Event, Clone, Copy)]
struct ToggleInspectorMode {
    set_inspecting: bool,
}

fn toggle_inspector_mode_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grabbed_object: Single<&GrabbedObject>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT)
        || grabbed_object.current_object_anchor == ObjectAnchor::Inspecting
            && keyboard_input.just_pressed(KeyCode::Escape)
    {
        commands.trigger(ToggleInspectorMode {
            set_inspecting: grabbed_object.current_object_anchor != ObjectAnchor::Inspecting,
        });
    }
}

fn on_toggle_inspector_mode(
    event: On<ToggleInspectorMode>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut commands: Commands,
    player_entity: Single<Entity, With<Player>>,
) {
    let set_inspecting = event.set_inspecting;

    grabbed_object.current_object_anchor =
        match (grabbed_object.current_object_anchor, set_inspecting) {
            (ObjectAnchor::Default, true) => ObjectAnchor::Inspecting,
            (ObjectAnchor::Inspecting, false) => ObjectAnchor::Default,
            (ObjectAnchor::AimDownSight, true) => ObjectAnchor::Inspecting,
            _ => return,
        };

    cursor_options.visible = set_inspecting;
    cursor_options.grab_mode = match set_inspecting {
        true => CursorGrabMode::None,
        false => CursorGrabMode::Locked,
    };

    commands.trigger(UpdatePlayerCharacterActive {
        entity: *player_entity,
    });
}

fn set_cursor_icon_on_pointer_event<E: Clone + Reflect + std::fmt::Debug>(
    icon: SystemCursorIcon,
) -> impl Fn(On<Pointer<E>>, Single<&GrabbedObject>, Single<&mut CursorIcon>) {
    move |event, grabbed_object, mut window_cursor| {
        if grabbed_object.current_object_anchor == ObjectAnchor::Inspecting
            && grabbed_object.entity == Some(event.entity)
        {
            **window_cursor = CursorIcon::System(icon);
        }
    }
}

fn rotate_grabbed_object_on_drag(
    event: On<Pointer<Drag>>,
    mut grab_orientations: Query<&mut GrabOrientation>,
    grabbed_object: Single<&GrabbedObject>,
) {
    if !(grabbed_object.current_object_anchor == ObjectAnchor::Inspecting
        && grabbed_object.entity == Some(event.entity))
    {
        return;
    }

    if let Ok(mut grab_orientation) = grab_orientations.get_mut(event.entity) {
        const PIXELS_PER_RADIAN: f32 = 150f32;

        let horizontal_rotation = Quat::from_axis_angle(Vec3::Y, event.delta.x / PIXELS_PER_RADIAN);
        let vertical_rotation = Quat::from_axis_angle(Vec3::X, event.delta.y / PIXELS_PER_RADIAN);

        grab_orientation.0 = horizontal_rotation * grab_orientation.0;
        grab_orientation.0 = vertical_rotation * grab_orientation.0;
    }
}
