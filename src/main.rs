mod utilities;
mod world;
mod player;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{player::PlayerPlugin, utilities::UtilitiesPlugin, world::WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            WorldPlugin,
            UtilitiesPlugin,
            PlayerPlugin,
        ))
        .run();
}
