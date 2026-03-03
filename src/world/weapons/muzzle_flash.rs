use bevy::prelude::*;
use std::time::Duration;

use crate::{
    utilities::system_sets::DisplaySystems,
    world::weapons::{ShootWeapon, Weapon},
};

pub struct MuzzleFlashPlugin;

impl Plugin for MuzzleFlashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_muzzle_flash_images)
            .add_systems(Update, update_muzzle_flash_animation.in_set(DisplaySystems))
            .add_observer(start_muzzle_flash_animation);
    }
}

#[derive(Resource)]
pub struct MuzzleFlashImages {
    image_handles: Vec<Handle<Image>>,
}

impl MuzzleFlashImages {
    pub fn new(image_handles: Vec<Handle<Image>>) -> Self {
        Self { image_handles }
    }

    pub fn image_count(&self) -> usize {
        self.image_handles.len()
    }

    pub fn get_image_at_index(&self, index: usize) -> Handle<Image> {
        self.image_handles[index].clone()
    }
}

#[derive(Component)]
pub struct MuzzleFlashAnimation {
    started_at: Option<Duration>,
    animation_duration: Duration,
}

impl MuzzleFlashAnimation {
    pub fn new(animation_duration: Duration) -> Self {
        Self {
            started_at: None,
            animation_duration,
        }
    }

    pub fn started_at(&self) -> Option<Duration> {
        self.started_at
    }

    pub fn animation_duration(&self) -> Duration {
        self.animation_duration
    }

    pub fn start_animation(&mut self, time: &Time) {
        self.started_at = Some(time.elapsed());
    }
}

fn setup_muzzle_flash_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_paths = [
        "textures/Muzzle flash sprites test/Backside frame 1.png",
        "textures/Muzzle flash sprites test/Backside frame 2.png",
        "textures/Muzzle flash sprites test/Backside frame 3.png",
    ];
    let image_handles = sprite_paths.map(|path| asset_server.load(path)).to_vec();
    commands.insert_resource(MuzzleFlashImages { image_handles });
}

fn start_muzzle_flash_animation(
    shoot_weapon: On<ShootWeapon>,
    weapons: Query<&Children, With<Weapon>>,
    mut muzzle_flash_animations: Query<&mut MuzzleFlashAnimation>,
    time: Res<Time>,
) {
    let weapon_children = weapons
        .get(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");

    for child in weapon_children.iter() {
        if let Ok(mut animation) = muzzle_flash_animations.get_mut(child) {
            animation.start_animation(&time);
        }
    }
}

fn update_muzzle_flash_animation(
    mut muzzle_flash_animations: Query<(
        &mut MuzzleFlashAnimation,
        &mut Visibility,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    time: Res<Time>,
    muzzle_flash_images: Res<MuzzleFlashImages>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut muzzle_flash_animation, mut visibility, mesh_material) in
        muzzle_flash_animations.iter_mut()
    {
        let Some(started_at) = muzzle_flash_animation.started_at else {
            continue;
        };

        let animation_elapsed = time.elapsed() - started_at;
        if animation_elapsed < muzzle_flash_animation.animation_duration {
            *visibility = Visibility::Visible;

            let image_index = calculate_animation_image_index(
                animation_elapsed,
                muzzle_flash_animation.animation_duration,
                muzzle_flash_images.image_count(),
            );

            let material = materials
                .get_mut(mesh_material)
                .expect("MeshMaterial3D should always point to valid Material asset.");

            material.base_color_texture = Some(muzzle_flash_images.get_image_at_index(image_index));
        } else {
            *visibility = Visibility::Hidden;
            muzzle_flash_animation.started_at = None;
        }
    }
}

fn calculate_animation_image_index(
    animation_elapsed: Duration,
    animation_duration: Duration,
    image_count: usize,
) -> usize {
    (animation_elapsed.as_secs_f64() / animation_duration.as_secs_f64() * image_count as f64)
        .floor() as usize
}
