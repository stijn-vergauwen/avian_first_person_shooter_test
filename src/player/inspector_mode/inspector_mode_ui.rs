use bevy::{
    color::palettes::{
        css::{BLUE, GREEN, RED},
        tailwind::{NEUTRAL_600, SKY_600},
    },
    feathers::controls::{SliderProps, slider},
    prelude::*,
    ui_widgets::{SliderPrecision, SliderStep, SliderValue, ValueChange, observe},
};

use crate::{
    player::grabbed_object::GrabbedObject,
    utilities::system_sets::DisplaySystems,
    world::{
        grabbable_object::{DefaultGrabOrientation, GrabOrientation},
        weapons::{Weapon, weapon_config::WeaponConfig},
    },
};

use super::ToggleInspectorMode;

pub struct InspectorModeUiPlugin;

impl Plugin for InspectorModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inspector_overlay)
            .add_systems(Update, draw_test_gizmo.in_set(DisplaySystems))
            .add_observer(on_toggle_inspector_mode);
    }
}

#[derive(Component)]
struct InspectorOverlay;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

fn spawn_inspector_overlay(mut commands: Commands) {
    commands
        .spawn((
            InspectorOverlay,
            Visibility::Hidden,
            Pickable::IGNORE,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay.spawn((
                Node {
                    margin: UiRect::all(px(20.0)),
                    padding: UiRect::all(px(10.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6.0),
                    ..default()
                },
                BackgroundColor(Color::from(NEUTRAL_600)),
                children![
                    Text::new("Shot origin"),
                    build_config_slider(Axis::X),
                    build_config_slider(Axis::Y),
                    build_config_slider(Axis::Z),
                ],
            ));

            overlay.spawn((
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::from(SKY_600)),
                observe(on_default_orientation_button_click),
                children![Text::new("Reset orientation")],
            ));
        });
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

fn draw_test_gizmo(
    mut gizmos: Gizmos,
    grabbed_object: Single<&GrabbedObject>,
    weapons: Query<(&Weapon, &GlobalTransform)>,
    mut weapon_configs: ResMut<Assets<WeaponConfig>>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let Ok((weapon, global_transform)) = weapons.get(grabbed_entity) else {
        return;
    };

    let weapon_config = weapon_configs.get_mut(weapon.config()).unwrap();

    let transform = global_transform.compute_transform()
        * Transform::from_translation(weapon_config.shot_origin);

    gizmos.arrow(
        transform.translation,
        transform.transform_point(Vec3::NEG_Z * 0.5),
        BLUE,
    );

    gizmos.line(
        transform.transform_point(Vec3::NEG_X * 0.1),
        transform.transform_point(Vec3::X * 0.1),
        RED,
    );

    gizmos.line(
        transform.transform_point(Vec3::NEG_Y * 0.1),
        transform.transform_point(Vec3::Y * 0.1),
        GREEN,
    );
}

fn build_config_slider(axis: Axis) -> impl Bundle {
    (
        slider(
            SliderProps {
                min: -1.0,
                max: 1.0,
                value: 0.0,
            },
            (SliderPrecision(2), SliderStep(0.01)),
        ),
        observe(build_on_slider_changed_observer(match axis {
            Axis::X => |weapon_config: &mut WeaponConfig, slider_value| {
                weapon_config.shot_origin.x = slider_value
            },
            Axis::Y => |weapon_config: &mut WeaponConfig, slider_value| {
                weapon_config.shot_origin.y = slider_value
            },
            Axis::Z => |weapon_config: &mut WeaponConfig, slider_value| {
                weapon_config.shot_origin.z = slider_value
            },
        })),
    )
}

#[allow(clippy::type_complexity)]
fn build_on_slider_changed_observer<F>(
    set_field: F,
) -> impl Fn(
    On<ValueChange<f32>>,
    Single<&GrabbedObject>,
    Query<&Weapon>,
    ResMut<Assets<WeaponConfig>>,
    Commands,
)
where
    F: Fn(&mut WeaponConfig, f32),
{
    move |value_change: On<ValueChange<f32>>,
          grabbed_object: Single<&GrabbedObject>,
          weapons: Query<&Weapon>,
          mut weapon_configs: ResMut<Assets<WeaponConfig>>,
          mut commands: Commands| {
        if let Some(grabbed_entity) = grabbed_object.entity
            && let Ok(weapon) = weapons.get(grabbed_entity)
        {
            let weapon_config = weapon_configs.get_mut(weapon.config()).unwrap();
            set_field(weapon_config, value_change.value);
        };

        commands
            .entity(value_change.source)
            .insert(SliderValue(value_change.value));
    }
}
