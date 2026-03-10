use bevy::{color::palettes::tailwind::SKY_600, prelude::*};

use crate::{
    player::grabbed_object::GrabbedObject,
    world::grabbable_object::{DefaultGrabOrientation, GrabOrientation},
};

use super::ToggleInspectorMode;

pub struct InspectorModeUiPlugin;

impl Plugin for InspectorModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inspector_overlay)
            .add_observer(on_toggle_inspector_mode);
    }
}

#[derive(Component)]
struct InspectorOverlay;

fn spawn_inspector_overlay(mut commands: Commands) {
    let overlay_entity = commands
        .spawn((
            InspectorOverlay,
            Visibility::Hidden,
            Pickable::IGNORE,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::from(SKY_600)),
            ChildOf(overlay_entity),
        ))
        .with_child(Text::new("Reset orientation"))
        .observe(on_default_orientation_button_click);
}

fn on_toggle_inspector_mode(
    event: On<ToggleInspectorMode>,
    mut inspector_overlay_visibility: Single<&mut Visibility, With<InspectorOverlay>>,
) {
    **inspector_overlay_visibility = match event.set_inspecting {
        true => Visibility::Visible,
        false => Visibility::Hidden,
    };
}

fn on_default_orientation_button_click(
    _: On<Pointer<Click>>,
    mut grab_orientations: Query<(&mut GrabOrientation, Option<&DefaultGrabOrientation>)>,
    grabbed_object: Single<&GrabbedObject>,
) {
    let (mut orientation, default) = grab_orientations
        .get_mut(grabbed_object.entity.unwrap())
        .unwrap();

    orientation.0 = default.map_or(Quat::IDENTITY, |orientation| orientation.value());
}
