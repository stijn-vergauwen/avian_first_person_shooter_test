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
    world::{grabbable_object::GrabOrientation, weapons::Weapon},
};

pub struct InspectorModePlugin;

impl Plugin for InspectorModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorModeUiPlugin)
            .insert_state(InspectorModeState::Disabled)
            .add_systems(
                OnEnter(InspectorModeState::Enabled),
                on_inspector_mode_enabled,
            )
            .add_systems(
                OnEnter(InspectorModeState::Disabled),
                on_inspector_mode_disabled,
            )
            .add_systems(
                Update,
                toggle_inspector_mode_on_keypress.in_set(InputSystems),
            )
            .add_observer(set_cursor_icon_on_pointer_event::<Over>(
                SystemCursorIcon::Pointer,
            ))
            .add_observer(set_cursor_icon_on_pointer_event::<Out>(
                SystemCursorIcon::Default,
            ))
            .add_observer(rotate_grabbed_object_on_drag);
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum InspectorModeState {
    #[default]
    Disabled,
    Enabled,
}

fn toggle_inspector_mode_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grabbed_object: Single<&GrabbedObject>,
    inspector_state: Res<State<InspectorModeState>>,
    mut next_inspector_state: ResMut<NextState<InspectorModeState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT)
        || grabbed_object.current_object_anchor == ObjectAnchor::Inspecting
            && keyboard_input.just_pressed(KeyCode::Escape)
    {
        let next_state = match inspector_state.get() {
            InspectorModeState::Disabled => InspectorModeState::Enabled,
            InspectorModeState::Enabled => InspectorModeState::Disabled,
        };

        next_inspector_state.set(next_state);
    }
}

fn on_inspector_mode_enabled(
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut commands: Commands,
    player_entity: Single<Entity, With<Player>>,
) {
    grabbed_object.current_object_anchor = ObjectAnchor::Inspecting;
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;

    commands.trigger(UpdatePlayerCharacterActive {
        entity: *player_entity,
    });
}

fn on_inspector_mode_disabled(
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut commands: Commands,
    player_entity: Single<Entity, With<Player>>,
    weapons: Query<&Weapon>,
    asset_server: Res<AssetServer>,
) {
    grabbed_object.current_object_anchor = ObjectAnchor::Default;
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;

    commands.trigger(UpdatePlayerCharacterActive {
        entity: *player_entity,
    });

    if let Some(grabbed_entity) = grabbed_object.entity
        && let Ok(weapon) = weapons.get(grabbed_entity)
    {
        let path = asset_server.get_path(weapon.config()).unwrap();
        asset_server.reload(path);
    };
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
    grabbed_object: Single<&GrabbedObject>,
    inspector_state: Res<State<InspectorModeState>>,
    mut grab_orientations: Query<&mut GrabOrientation>,
) {
    if grabbed_object.entity != Some(event.entity)
        || *inspector_state == InspectorModeState::Disabled
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
