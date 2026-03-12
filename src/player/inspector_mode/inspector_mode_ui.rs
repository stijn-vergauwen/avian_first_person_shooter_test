use bevy::{
    color::palettes::tailwind::{NEUTRAL_600, SKY_600},
    feathers::controls::{SliderProps, slider},
    prelude::*,
    ui_widgets::{SliderPrecision, SliderStep, SliderValue, ValueChange, observe},
};

use crate::{
    player::grabbed_object::GrabbedObject,
    utilities::system_sets::DisplaySystems,
    world::grabbable_object::{DefaultGrabOrientation, GrabOrientation, GrabbableObject},
};

use super::ToggleInspectorMode;

pub struct InspectorModeUiPlugin;

impl Plugin for InspectorModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponShotOrigin>()
            .add_systems(Startup, spawn_inspector_overlay)
            .add_systems(Update, draw_test_gizmo.in_set(DisplaySystems))
            .add_observer(on_toggle_inspector_mode);
    }
}

#[derive(Component)]
struct InspectorOverlay;

#[derive(Resource, Default)]
struct WeaponShotOrigin(Vec3);

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

            overlay
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
                ))
                .with_child(Text::new("Reset orientation"))
                .observe(on_default_orientation_button_click);
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
    grabbable_objects: Query<&GlobalTransform, With<GrabbableObject>>,
    shot_origin: Res<WeaponShotOrigin>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let global_transform = grabbable_objects.get(grabbed_entity).unwrap();

    let transform =
        global_transform.compute_transform() * Transform::from_translation(shot_origin.0);

    gizmos.axes(transform, 0.2);
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
            Axis::X => |shot_origin: &mut WeaponShotOrigin, value| shot_origin.0.x = value,
            Axis::Y => |shot_origin: &mut WeaponShotOrigin, value| shot_origin.0.y = value,
            Axis::Z => |shot_origin: &mut WeaponShotOrigin, value| shot_origin.0.z = value,
        })),
    )
}

fn build_on_slider_changed_observer<F>(
    set_field: F,
) -> impl Fn(On<ValueChange<f32>>, ResMut<WeaponShotOrigin>, Commands)
where
    F: Fn(&mut WeaponShotOrigin, f32),
{
    move |value_change: On<ValueChange<f32>>,
          mut shot_origin: ResMut<WeaponShotOrigin>,
          mut commands: Commands| {
        set_field(&mut shot_origin, value_change.value);

        commands
            .entity(value_change.source)
            .insert(SliderValue(value_change.value));
    }
}
