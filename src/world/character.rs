use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems();
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Character {
    /// If this Character is currently active / controllable.
    pub is_active: bool,
}
