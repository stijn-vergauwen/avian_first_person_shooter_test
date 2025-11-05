use crate::utilities::app_state::AppState;
use bevy::prelude::*;

pub struct SystemSetPlugin;

impl Plugin for SystemSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InputSet,
                UiSet::HandleRequests,
                UiSet::UpdateUi,
                UiSet::DespawnUi,
                DisplaySet.run_if(in_state(AppState::InGame)),
            )
                .chain(),
        )
        .configure_sets(
            FixedUpdate,
            (
                DataSet::PrepareData,
                DataSet::HandleRequests,
                DataSet::SpawnEntities,
                DataSet::UpdateEntities,
                DataSet::DespawnEntities,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    }
}

/// [SystemSet] for user input.
///
/// This set runs in the [Update] schedule, systems using this set should avoid:
/// - modifying anything visual
/// - modifying components
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct InputSet;

/// [SystemSet] for UI logic.
///
/// This set runs in the [Update] schedule, systems using this set should avoid:
/// - directly modifying game-state or simulation data (write events instead)
///
/// To decide what falls under "UI", I think a good way to think about it is "would this be visible or left out in a cinematic?"
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum UiSet {
    HandleRequests,
    UpdateUi,
    DespawnUi,
}

/// [SystemSet] for in-game logic that reads game-state to update or draw things on the screen.
///
/// This set runs in the [Update] schedule, systems using this set should avoid:
/// - spawning or despawning entities or components
/// - ui
/// - sending events
/// - modifying anything that doesn't directly change how something is displayed on screen
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct DisplaySet;

/// [Systemset] for in-game logic that modifies game-state or simulation data.
///
/// This set runs in the [FixedUpdate] schedule, systems using this set should avoid:
/// - modifying anything visual (should be limited to preparing data for [DisplaySet])
/// - ui
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum DataSet {
    PrepareData,
    HandleRequests,
    SpawnEntities,
    UpdateEntities,
    DespawnEntities,
}
