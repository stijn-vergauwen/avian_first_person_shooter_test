use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    player::grabbed_object::GrabbedObject,
    utilities::system_sets::UiSystems,
    world::{grabbable_object::GrabbableObject, interaction_target::PlayerInteractionTarget},
};

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crosshair)
            .add_systems(Update, update_crosshair_color.in_set(UiSystems::UpdateUi));
    }
}

/// Marker component grabbable object crosshair.
#[derive(Component)]
struct Crosshair;

fn spawn_crosshair(mut commands: Commands) {
    const RADIUS: f32 = 16.0;
    const THICKNESS: f32 = 3.0;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                Crosshair,
                Node {
                    width: Val::Px(RADIUS),
                    height: Val::Px(RADIUS),
                    border: UiRect::all(Val::Px(THICKNESS)),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderColor::all(NEUTRAL_400),
                Pickable::IGNORE,
            ));
        });
}

fn update_crosshair_color(
    player_interaction_target: Res<PlayerInteractionTarget>,
    grabbable_query: Query<&GrabbableObject>,
    grabbed_object: Res<GrabbedObject>,
    mut crosshair_color: Single<&mut BorderColor, With<Crosshair>>,
) {
    let new_color = match player_interaction_target.current_target() {
        Some(target)
            if grabbable_query.contains(target.entity)
                && grabbed_object.entity != Some(target.entity) =>
        {
            Color::Srgba(TEAL_400)
        }
        _ => Color::Srgba(NEUTRAL_400),
    };

    if crosshair_color.top != new_color {
        crosshair_color.set_all(new_color);
    }
}
