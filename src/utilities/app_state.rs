use bevy::prelude::*;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
    }
}

#[derive(States, Copy, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
