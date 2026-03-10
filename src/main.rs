use avian_first_person_shooter_test::{
    player::{
        PlayerPlugin, player_weapon_collision_hook::PlayerWeaponCollisionHooks,
    },
    utilities::UtilitiesPlugin,
    world::WorldPlugin,
};
use avian3d::prelude::*;
#[allow(unused_imports)]
use bevy::{prelude::*, window::WindowMode};

fn main() {
    App::new()
        .add_plugins((
            // DefaultPlugins,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default().with_collision_hooks::<PlayerWeaponCollisionHooks>(),
            PhysicsDebugPlugin,
            PhysicsPickingPlugin,
            WorldPlugin,
            UtilitiesPlugin,
            PlayerPlugin,
        ))
        .insert_resource(Time::from_hz(128_f64))
        .run();
}
