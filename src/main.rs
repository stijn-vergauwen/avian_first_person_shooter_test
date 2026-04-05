#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian_first_person_shooter_test::{
    player::{PlayerPlugin, player_weapon_collision_hook::PlayerWeaponCollisionHooks},
    utilities::UtilitiesPlugin,
    world::WorldPlugin,
};
use avian3d::prelude::*;
use bevy::feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme};
use bevy::{prelude::*, window::WindowMode};

const FULLSCREEN: bool = true;

fn main() {
    App::new()
        .add_plugins((
            if FULLSCREEN {
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
            } else {
                DefaultPlugins.build()
            },
            PhysicsPlugins::default().with_collision_hooks::<PlayerWeaponCollisionHooks>(),
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
