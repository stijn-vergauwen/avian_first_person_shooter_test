use bevy::{
    color::palettes::{
        css::{BLUE, GREEN, LIME, ORANGE, RED},
        tailwind::{NEUTRAL_500, NEUTRAL_700, SKY_600},
    },
    feathers::controls::{SliderProps, slider},
    prelude::*,
    ui_widgets::{SliderPrecision, SliderStep, SliderValue, ValueChange, observe},
};

use crate::{
    player::grabbed_object::GrabbedObject,
    utilities::{angle::Angle, system_sets::DisplaySystems},
    world::{
        grabbable_object::{DefaultGrabOrientation, GrabOrientation},
        weapons::{
            Weapon, weapon_config::WeaponConfig, weapon_config_modified::WeaponConfigModified,
            weapon_config_save::SaveWeaponConfig,
        },
    },
};

use super::InspectorModeState;

pub struct InspectorModeUiPlugin;

impl Plugin for InspectorModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(InspectorModeState::Enabled),
            on_inspector_mode_enabled,
        )
        .add_systems(
            OnEnter(InspectorModeState::Disabled),
            on_inspector_mode_disabled,
        )
        .add_systems(Startup, spawn_inspector_overlay)
        .add_systems(
            Update,
            draw_test_gizmo
                .in_set(DisplaySystems)
                .run_if(in_state(InspectorModeState::Enabled)),
        )
        .add_observer(on_weapon_config_modified)
        .add_observer(on_update_inspector_overlay);
    }
}

#[derive(Component)]
struct InspectorOverlay;

#[derive(Component)]
struct WeaponConfigMenu;

#[allow(clippy::type_complexity)]
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
            overlay
                .spawn((
                    WeaponConfigMenu,
                    Visibility::Hidden,
                    Node {
                        margin: UiRect::all(px(20.0)),
                        padding: UiRect::all(px(10.0)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: px(30.0),
                        ..default()
                    },
                    BackgroundColor(Color::from(NEUTRAL_700)),
                ))
                .with_children(|config_menu| {
                    config_menu.spawn((
                        Node {
                            width: px(300.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(6.0),
                            ..default()
                        },
                        children![
                            Text::new("Shot origin"),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config.shot_origin.x),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .shot_origin
                                    .x =
                                    slider_value)
                            }),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config.shot_origin.y),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .shot_origin
                                    .y =
                                    slider_value)
                            }),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config.shot_origin.z),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .shot_origin
                                    .z =
                                    slider_value)
                            }),
                        ],
                    ));

                    config_menu.spawn((
                        Node {
                            width: px(300.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(6.0),
                            ..default()
                        },
                        children![
                            Text::new("Aim down sight position"),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config.ads_position.x),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .ads_position
                                    .x =
                                    slider_value)
                            }),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config.ads_position.y),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .ads_position
                                    .y =
                                    slider_value)
                            }),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config.ads_position.z),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .ads_position
                                    .z =
                                    slider_value)
                            }),
                        ],
                    ));

                    config_menu.spawn((
                        Node {
                            width: px(300.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(6.0),
                            ..default()
                        },
                        children![
                            Text::new("Shell ejection"),
                            (Text::new("Position"), TextFont::from_font_size(16.0),),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config
                                    .shell_ejection_position
                                    .x),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .shell_ejection_position
                                    .x =
                                    slider_value)
                            }),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config
                                    .shell_ejection_position
                                    .y),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .shell_ejection_position
                                    .y =
                                    slider_value)
                            }),
                            build_config_slider(SliderForWeaponConfig {
                                get_value: Box::new(|weapon_config| weapon_config
                                    .shell_ejection_position
                                    .z),
                                set_value: Box::new(|weapon_config, slider_value| weapon_config
                                    .shell_ejection_position
                                    .z =
                                    slider_value)
                            }),
                            (Text::new("Rotation (Y,X)"), TextFont::from_font_size(16.0),),
                            build_config_slider_with_range(
                                160.0,
                                SliderForWeaponConfig {
                                    get_value: Box::new(|weapon_config| weapon_config
                                        .shell_ejection_rotation
                                        .y
                                        .as_degrees()),
                                    set_value: Box::new(|weapon_config, slider_value| {
                                        weapon_config.shell_ejection_rotation.y =
                                            Angle::from_degrees(slider_value)
                                    })
                                }
                            ),
                            build_config_slider_with_range(
                                160.0,
                                SliderForWeaponConfig {
                                    get_value: Box::new(|weapon_config| weapon_config
                                        .shell_ejection_rotation
                                        .x
                                        .as_degrees()),
                                    set_value: Box::new(|weapon_config, slider_value| {
                                        weapon_config.shell_ejection_rotation.x =
                                            Angle::from_degrees(slider_value)
                                    })
                                }
                            ),
                            (Text::new("Ejection force"), TextFont::from_font_size(16.0)),
                            (
                                slider(
                                    SliderProps {
                                        min: 0.0,
                                        max: 8.0,
                                        value: 0.0,
                                    },
                                    (
                                        SliderPrecision(1),
                                        SliderStep(0.1),
                                        SliderForWeaponConfig {
                                            get_value: Box::new(
                                                |weapon_config| weapon_config.shell_ejection_force
                                            ),
                                            set_value: Box::new(|weapon_config, slider_value| {
                                                weapon_config.shell_ejection_force = slider_value
                                            })
                                        },
                                    ),
                                ),
                                observe(on_slider_value_changed),
                            ),
                            (
                                Text::new("Spin (Y axis, turns/s)"),
                                TextFont::from_font_size(16.0),
                            ),
                            build_config_slider_with_range(
                                5.0,
                                SliderForWeaponConfig {
                                    get_value: Box::new(|weapon_config| Angle(
                                        weapon_config.shell_ejection_spin.y
                                    )
                                    .as_turns()),
                                    set_value: Box::new(|weapon_config, slider_value| {
                                        weapon_config.shell_ejection_spin.y =
                                            Angle::from_turns(slider_value).radians()
                                    })
                                }
                            ),
                            (
                                Text::new("Ejection randomness"),
                                TextFont::from_font_size(16.0)
                            ),
                            (
                                slider(
                                    SliderProps {
                                        min: 0.0,
                                        max: 1.0,
                                        value: 0.0,
                                    },
                                    (
                                        SliderPrecision(2),
                                        SliderStep(0.01),
                                        SliderForWeaponConfig {
                                            get_value: Box::new(|weapon_config| weapon_config
                                                .shell_ejection_randomness),
                                            set_value: Box::new(|weapon_config, slider_value| {
                                                weapon_config.shell_ejection_randomness =
                                                    slider_value
                                            })
                                        },
                                    ),
                                ),
                                observe(on_slider_value_changed),
                            ),
                        ],
                    ));

                    config_menu.spawn((
                        Node {
                            width: px(300.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(6.0),
                            ..default()
                        },
                        #[cfg(not(target_family = "wasm"))]
                        children![
                            (
                                Button,
                                Node {
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::from(NEUTRAL_500)),
                                observe(on_reset_config_button_click),
                                children![Text::new("Reset configuration")],
                            ),
                            (
                                Button,
                                Node {
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::from(SKY_600)),
                                observe(on_save_button_click),
                                children![Text::new("Save configuration")],
                            ),
                        ],
                        #[cfg(target_family = "wasm")]
                        children![(
                            Button,
                            Node {
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::from(NEUTRAL_500)),
                            observe(on_reset_config_button_click),
                            children![Text::new("Reset configuration")],
                        )],
                    ));
                });

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

fn on_inspector_mode_enabled(
    mut inspector_overlay_visibility: Single<&mut Visibility, With<InspectorOverlay>>,
    mut config_menu_visibility: Single<
        &mut Visibility,
        (With<WeaponConfigMenu>, Without<InspectorOverlay>),
    >,
    weapons: Query<&Weapon>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    grabbed_object: Res<GrabbedObject>,
    mut commands: Commands,
) {
    **inspector_overlay_visibility = Visibility::Inherited;

    if let Some(grabbed_entity) = grabbed_object.entity
        && let Ok(weapon) = weapons.get(grabbed_entity)
    {
        **config_menu_visibility = Visibility::Inherited;

        let weapon_config = weapon_configs.get(weapon.config()).unwrap().clone();

        commands.trigger(UpdateInspectorOverlay { weapon_config });
    } else {
        **config_menu_visibility = Visibility::Hidden;
    };
}

fn on_inspector_mode_disabled(
    mut inspector_overlay_visibility: Single<&mut Visibility, With<InspectorOverlay>>,
) {
    **inspector_overlay_visibility = Visibility::Hidden;
}

fn on_weapon_config_modified(
    weapon_config_modified: On<WeaponConfigModified>,
    grabbed_object: Res<GrabbedObject>,
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
    weapons: Query<&Weapon>,
    sliders: Query<&SliderForWeaponConfig>,
    grabbed_object: Res<GrabbedObject>,
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
    grabbed_object: Res<GrabbedObject>,
) {
    let (mut orientation, default) = grab_orientations
        .get_mut(grabbed_object.entity.unwrap())
        .unwrap();

    orientation.0 = default.map_or(Quat::IDENTITY, |orientation| orientation.value());
}

#[cfg(not(target_family = "wasm"))]
fn on_save_button_click(
    _: On<Pointer<Click>>,
    grabbed_object: Res<GrabbedObject>,
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

fn on_reset_config_button_click(
    _: On<Pointer<Click>>,
    grabbed_object: Res<GrabbedObject>,
    weapons: Query<&Weapon>,
    asset_server: Res<AssetServer>,
) {
    if let Some(grabbed_entity) = grabbed_object.entity
        && let Ok(weapon) = weapons.get(grabbed_entity)
    {
        let path = asset_server.get_path(weapon.config()).unwrap();
        asset_server.reload(path);
    };
}

// Utilities

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

fn build_config_slider_with_range(
    range: f32,
    slider_for_weapon_config: SliderForWeaponConfig,
) -> impl Bundle {
    (
        slider(
            SliderProps {
                min: -range,
                max: range,
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

// Gizmos

fn draw_test_gizmo(
    mut gizmos: Gizmos,
    grabbed_object: Res<GrabbedObject>,
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
    let weapon_transform = global_transform.compute_transform();

    draw_forward_direction_arrow(
        &mut gizmos,
        calculate_config_transform(weapon_transform, weapon_config.shot_origin),
    );

    draw_ads_arrow(
        &mut gizmos,
        calculate_config_transform(weapon_transform, weapon_config.ads_position),
    );

    draw_shell_ejection_arrow(
        &mut gizmos,
        weapon_transform
            * Transform::from_translation(weapon_config.shell_ejection_position)
                .with_rotation(weapon_config.shell_ejection_rotation.to_quat()),
    );
}

fn draw_forward_direction_arrow(gizmos: &mut Gizmos, transform: Transform) {
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

fn draw_ads_arrow(gizmos: &mut Gizmos, transform: Transform) {
    gizmos.arrow(
        transform.translation,
        transform.transform_point(Vec3::NEG_Z * 0.5),
        LIME,
    );

    gizmos.circle(transform.to_isometry(), 0.02, LIME);
    gizmos.circle(
        (transform * Transform::from_translation(Vec3::NEG_Z * 0.3)).to_isometry(),
        0.02,
        LIME,
    );
}

fn draw_shell_ejection_arrow(gizmos: &mut Gizmos, transform: Transform) {
    gizmos.arrow(
        transform.translation,
        transform.transform_point(Vec3::NEG_Z * 0.2),
        ORANGE,
    );

    gizmos.rect(transform.to_isometry(), Vec2::new(0.1, 0.05), ORANGE);
}

fn calculate_config_transform(weapon_transform: Transform, offset_config: Vec3) -> Transform {
    weapon_transform * Transform::from_translation(offset_config)
}
