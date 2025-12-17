mod player;
mod utilities;
mod world;

use avian3d::prelude::*;
use bevy::{prelude::*, window::WindowMode};

use crate::{player::PlayerPlugin, utilities::UtilitiesPlugin, world::WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            WorldPlugin,
            UtilitiesPlugin,
            PlayerPlugin,
        ))
        .run();
}
