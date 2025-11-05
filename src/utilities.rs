mod system_sets;
mod app_state;

use bevy::prelude::*;

use crate::utilities::{app_state::AppStatePlugin, system_sets::SystemSetPlugin};

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppStatePlugin, SystemSetPlugin));
    }
}