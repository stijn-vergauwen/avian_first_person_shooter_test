mod world;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), WorldPlugin))
        .run();
}
