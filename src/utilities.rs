mod angle;
mod app_state;
pub mod euler_angle;
pub mod fraction;
mod system_sets;

use bevy::prelude::*;

use crate::utilities::{app_state::AppStatePlugin, system_sets::SystemSetPlugin};

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppStatePlugin, SystemSetPlugin));
    }
}
