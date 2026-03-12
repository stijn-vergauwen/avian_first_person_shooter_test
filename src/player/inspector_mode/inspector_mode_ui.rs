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

#[allow(clippy::type_complexity)]
#[derive(Component)]
struct SliderForWeaponConfig {
    get_value: Box<dyn Fn(&WeaponConfig) -> f32 + Send + Sync>,
    set_value: Box<dyn Fn(&mut WeaponConfig, f32) + Send + Sync>,
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
                    build_config_slider(SliderForWeaponConfig {
                        get_value: Box::new(|weapon_config| weapon_config.shot_origin.x),
                        set_value: Box::new(|weapon_config, slider_value| weapon_config
                            .shot_origin
                            .x = slider_value)
                    }),
                    build_config_slider(SliderForWeaponConfig {
                        get_value: Box::new(|weapon_config| weapon_config.shot_origin.y),
                        set_value: Box::new(|weapon_config, slider_value| weapon_config
                            .shot_origin
                            .y = slider_value)
                    }),
                    build_config_slider(SliderForWeaponConfig {
                        get_value: Box::new(|weapon_config| weapon_config.shot_origin.z),
                        set_value: Box::new(|weapon_config, slider_value| weapon_config
                            .shot_origin
                            .z = slider_value)
                    }),
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
    grabbed_object: Single<&GrabbedObject>,
    sliders: Query<(Entity, &SliderForWeaponConfig)>,
    weapons: Query<&Weapon>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    mut commands: Commands,
) {
    **inspector_overlay_visibility = match event.set_inspecting {
        true => Visibility::Visible,
        false => Visibility::Hidden,
    };

    if let Some(grabbed_entity) = grabbed_object.entity
        && let Ok(weapon) = weapons.get(grabbed_entity)
    {
        let weapon_config = weapon_configs.get(weapon.config()).unwrap();

        for (slider_entity, slider_for_weapon_config) in sliders.iter() {
            let new_value = (slider_for_weapon_config.get_value)(weapon_config);

            commands
                .entity(slider_entity)
                .insert(SliderValue(new_value));
        }
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

fn build_config_slider(slider_for_weapon_config: SliderForWeaponConfig) -> impl Bundle {
    (
        slider(
            SliderProps {
                min: -1.0,
                max: 1.0,
                value: 0.0,
            },
            (
                SliderPrecision(2),
                SliderStep(0.01),
                slider_for_weapon_config,
            ),
        ),
        observe(build_on_slider_changed_observer()),
    )
}

#[allow(clippy::type_complexity)]
fn build_on_slider_changed_observer() -> impl Fn(
    On<ValueChange<f32>>,
    Query<&SliderForWeaponConfig>,
    Single<&GrabbedObject>,
    Query<&Weapon>,
    ResMut<Assets<WeaponConfig>>,
    Commands,
) {
    move |value_change: On<ValueChange<f32>>,
          query: Query<&SliderForWeaponConfig>,
          grabbed_object: Single<&GrabbedObject>,
          weapons: Query<&Weapon>,
          mut weapon_configs: ResMut<Assets<WeaponConfig>>,
          mut commands: Commands| {
        if let Some(grabbed_entity) = grabbed_object.entity
            && let Ok(weapon) = weapons.get(grabbed_entity)
        {
            let weapon_config = weapon_configs.get_mut(weapon.config()).unwrap();

            let slider_for_weapon_config = query.get(value_change.event_target()).unwrap();
            (slider_for_weapon_config.set_value)(weapon_config, value_change.value);
        };

        commands
            .entity(value_change.source)
            .insert(SliderValue(value_change.value));
    }
}
