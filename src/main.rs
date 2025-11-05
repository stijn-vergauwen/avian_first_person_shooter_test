mod utilities;
mod world;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{utilities::UtilitiesPlugin, world::WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            WorldPlugin,
            UtilitiesPlugin,
        ))
        .run();
}
