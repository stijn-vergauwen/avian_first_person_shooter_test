#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian_first_person_shooter_test::{
    player::{PlayerPlugin, player_weapon_collision_hook::PlayerWeaponCollisionHooks},
    utilities::UtilitiesPlugin,
    world::WorldPlugin,
};
use avian3d::prelude::*;
use bevy::{
    asset::AssetMetaCheck,
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
};
use bevy::{prelude::*, window::WindowMode};

const FULLSCREEN: bool = true;

fn main() {
    let mut default_plugins = DefaultPlugins.build().set(AssetPlugin {
        // Checking for meta files that don't exist can cause errors on Wasm builds.
        meta_check: AssetMetaCheck::Never,
        ..default()
    });

    #[cfg(target_family = "wasm")]
    {
        default_plugins = default_plugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "First-person shooter prototype".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        });
    }

    #[cfg(not(target_family = "wasm"))]
    if FULLSCREEN {
        default_plugins = default_plugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        });
    };

    App::new()
        .add_plugins((
            default_plugins,
            PhysicsPlugins::default().with_collision_hooks::<PlayerWeaponCollisionHooks>(),
            // #[cfg(feature = "dev")]
            // PhysicsDebugPlugin,
            PhysicsPickingPlugin,
            FeathersPlugins,
            WorldPlugin,
            UtilitiesPlugin,
            PlayerPlugin,
        ))
        .insert_resource(Time::from_hz(128_f64))
        .insert_resource(UiTheme(create_dark_theme()))
        .run();
}
