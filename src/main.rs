mod player;
mod utilities;
mod world;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{player::PlayerPlugin, utilities::UtilitiesPlugin, world::WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // DefaultPlugins.set(WindowPlugin {
            //     primary_window: Some(Window {
            //         mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            //         ..default()
            //     }),
            //     ..default()
            // }),
            PhysicsPlugins::default(),
            PhysicsPickingPlugin,
            WorldPlugin,
            UtilitiesPlugin,
            PlayerPlugin,
        ))
        .run();
}
