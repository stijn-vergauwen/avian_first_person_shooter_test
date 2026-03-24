use std::time::Duration;

use avian3d::prelude::{Sleeping, WakeBody};
use bevy::prelude::*;

use super::system_sets::DataSystems;

pub struct DespawnAfterSleepPlugin;

impl Plugin for DespawnAfterSleepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_started_sleeping_at.in_set(DataSystems::UpdateEntities),
                delete_sleeping_entities_after_duration.in_set(DataSystems::DespawnEntities),
            ),
        );
    }
}

/// Automatically despawns this entity after it has been sleeping for the given duration.
#[derive(Component, Clone, Copy)]
#[require(StartedSleepingAt)]
pub struct DespawnAfterSleepingForDuration(pub Duration);

/// Tracks the time since this entity started sleeping.
///
/// This entity starts sleeping whenever Avian physics adds the [Sleeping] component. The reason I'm not just using Avian's [SleepTimer] is because that stops counting once the entity is sleeping.
#[derive(Component, Clone, Copy, Default)]
pub struct StartedSleepingAt {
    duration: Option<Duration>,
}

fn update_started_sleeping_at(
    mut entities: Query<(&mut StartedSleepingAt, Has<Sleeping>)>,
    time: Res<Time>,
) {
    for (mut started_sleeping_at, is_sleeping) in entities.iter_mut() {
        if is_sleeping {
            if started_sleeping_at.duration.is_none() {
                started_sleeping_at.duration = Some(time.elapsed());
            }
        } else if started_sleeping_at.duration.is_some() {
            started_sleeping_at.duration = None;
        }
    }
}

fn delete_sleeping_entities_after_duration(
    sleeping_particles: Query<(Entity, &DespawnAfterSleepingForDuration, &StartedSleepingAt)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (particle_entity, despawn_after, started_sleeping) in sleeping_particles.iter() {
        let Some(started_sleeping) = started_sleeping.duration else {
            continue;
        };

        if time.elapsed() > started_sleeping + despawn_after.0 {
            commands.queue(WakeBody(particle_entity));
            commands.entity(particle_entity).despawn();
        }
    }
}
