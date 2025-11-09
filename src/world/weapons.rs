use bevy::{color::palettes::tailwind::RED_500, prelude::*};

use crate::utilities::system_sets::{DisplaySystems, InputSystems};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_weapon)
            .add_systems(Update, (shoot_weapon_on_click.in_set(InputSystems), draw_weapon_fire_direction.in_set(DisplaySystems)));
    }
}

/// Base component for weapons.
#[derive(Component, Clone, Copy)]
struct Weapon;

fn spawn_test_weapon(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapon_model = asset_server.load("models/Blocky assault rifle.glb#Scene0");

    commands.spawn((
        Weapon,
        Transform::from_xyz(0.0, 1.0, 0.0),
        SceneRoot(weapon_model),
    ));
}

fn shoot_weapon_on_click(mouse_input: Res<ButtonInput<MouseButton>>, _weapon: Single<&Weapon>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("Shoot!");
    }
}

fn draw_weapon_fire_direction(weapon: Single<&GlobalTransform, With<Weapon>>, mut gizmos: Gizmos) {
    gizmos.ray(weapon.translation(), weapon.forward() * 10.0, RED_500);
}