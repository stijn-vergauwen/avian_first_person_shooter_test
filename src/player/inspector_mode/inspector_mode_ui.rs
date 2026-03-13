use bevy::{
    color::palettes::{
        css::{BLUE, GREEN, RED},
        tailwind::{NEUTRAL_700, SKY_600},
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
        weapons::{
            Weapon, weapon_config::WeaponConfig, weapon_config_modified::WeaponConfigModified,
            weapon_config_save::SaveWeaponConfig,
        },
    },
};

use super::ToggleInspectorMode;

pub struct InspectorModeUiPlugin;

impl Plugin for InspectorModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inspector_overlay)
            .add_systems(Update, draw_test_gizmo.in_set(DisplaySystems))
            .add_observer(on_toggle_inspector_mode)
            .add_observer(on_weapon_config_modified)
            .add_observer(on_update_inspector_overlay);
    }
}

#[derive(Component)]
struct InspectorOverlay;

#[derive(Component)]
struct SliderForWeaponConfig {
    get_value: Box<dyn Fn(&WeaponConfig) -> f32 + Send + Sync>,
    set_value: Box<dyn Fn(&mut WeaponConfig, f32) + Send + Sync>,
}

/// Decribes a request to update all the UI elements of the inspector mode overlay.
#[derive(Event)]
struct UpdateInspectorOverlay {
    weapon_config: WeaponConfig,
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
                    width: px(300.0),
                    margin: UiRect::all(px(20.0)),
                    padding: UiRect::all(px(10.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6.0),
                    ..default()
                },
                BackgroundColor(Color::from(NEUTRAL_700)),
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

            overlay.spawn((
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    left: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::from(SKY_600)),
                observe(on_save_button_click),
                children![Text::new("Save configuration")],
            ));
        });
}

fn on_toggle_inspector_mode(
    event: On<ToggleInspectorMode>,
    mut inspector_overlay_visibility: Single<&mut Visibility, With<InspectorOverlay>>,
    grabbed_object: Single<&GrabbedObject>,
    weapons: Query<&Weapon>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    mut commands: Commands,
) {
    **inspector_overlay_visibility = match event.set_inspecting {
        true => Visibility::Visible,
        false => Visibility::Hidden,
    };

    if event.set_inspecting
        && let Some(grabbed_entity) = grabbed_object.entity
        && let Ok(weapon) = weapons.get(grabbed_entity)
    {
        let weapon_config = weapon_configs.get(weapon.config()).unwrap().clone();

        commands.trigger(UpdateInspectorOverlay { weapon_config });
    };
}

fn on_weapon_config_modified(
    weapon_config_modified: On<WeaponConfigModified>,
    grabbed_object: Single<&GrabbedObject>,
    mut commands: Commands,
) {
    if grabbed_object.entity.is_some_and(|grabbed_entity| {
        weapon_config_modified
            .weapon_entities
            .contains(&grabbed_entity)
    }) {
        commands.trigger(UpdateInspectorOverlay {
            weapon_config: weapon_config_modified.new_data.clone(),
        });
    }
}

fn on_update_inspector_overlay(
    update_inspector_overlay: On<UpdateInspectorOverlay>,
    sliders: Query<(Entity, &SliderForWeaponConfig)>,
    mut commands: Commands,
) {
    for (slider_entity, slider_for_weapon_config) in sliders.iter() {
        let new_value =
            (slider_for_weapon_config.get_value)(&update_inspector_overlay.weapon_config);

        commands
            .entity(slider_entity)
            .insert(SliderValue(new_value));
    }
}

fn on_slider_value_changed(
    value_change: On<ValueChange<f32>>,
    grabbed_object: Single<&GrabbedObject>,
    weapons: Query<&Weapon>,
    sliders: Query<&SliderForWeaponConfig>,
    mut weapon_configs: ResMut<Assets<WeaponConfig>>,
) {
    if let Some(grabbed_entity) = grabbed_object.entity
        && let Ok(weapon) = weapons.get(grabbed_entity)
    {
        let weapon_config = weapon_configs.get_mut(weapon.config()).unwrap();

        let slider_for_weapon_config = sliders.get(value_change.source).unwrap();
        (slider_for_weapon_config.set_value)(weapon_config, value_change.value);
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

fn on_save_button_click(
    _: On<Pointer<Click>>,
    grabbed_object: Single<&GrabbedObject>,
    weapons: Query<&Weapon>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if let Some(grabbed_entity) = grabbed_object.entity
        && let Ok(weapon) = weapons.get(grabbed_entity)
    {
        let weapon_config = weapon_configs.get(weapon.config()).unwrap().clone();
        let path = asset_server.get_path(weapon.config()).unwrap().to_string();

        commands.trigger(SaveWeaponConfig {
            path,
            weapon_config,
        });
    };
}

// Gizmos

fn draw_test_gizmo(
    mut gizmos: Gizmos,
    grabbed_object: Single<&GrabbedObject>,
    weapons: Query<(&Weapon, &GlobalTransform)>,
    weapon_configs: Res<Assets<WeaponConfig>>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let Ok((weapon, global_transform)) = weapons.get(grabbed_entity) else {
        return;
    };

    let weapon_config = weapon_configs.get(weapon.config()).unwrap();

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
                SliderPrecision(3),
                SliderStep(0.001),
                slider_for_weapon_config,
            ),
        ),
        observe(on_slider_value_changed),
    )
}
