pub mod angle;
mod app_state;
pub mod euler_angle;
pub mod fraction;
pub mod system_sets;
pub mod pd_controller;

use bevy::prelude::*;

use crate::utilities::{app_state::AppStatePlugin, system_sets::SystemSetPlugin};

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppStatePlugin, SystemSetPlugin));
    }
}

/// Marker component for entities whos gizmos should be drawn.
#[derive(Component, Clone, Copy)]
pub struct DrawGizmos;