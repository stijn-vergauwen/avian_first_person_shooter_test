use std::time::Duration;

use bevy::{color::palettes::css::GREY, prelude::*};

const ANIMATION_DURATION: Duration = Duration::from_millis(40);
const IMAGE_SIZE: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_muzzle_flash_images,
                setup_world,
                spawn_muzzle_flash_image,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                rotate_muzzle_flash_animation_on_arrow_keys,
                start_animation_on_click,
                update_muzzle_flash_animation,
                draw_gizmos,
            )
                .chain(),
        )
        .run();
}

#[derive(Resource)]
struct MuzzleFlashImages {
    image_handles: Vec<Handle<Image>>,
}

impl MuzzleFlashImages {
    fn image_count(&self) -> usize {
        self.image_handles.len()
    }

    fn get_image_at_index(&self, index: usize) -> Handle<Image> {
        self.image_handles[index].clone()
    }
}

#[derive(Component)]
struct MuzzleFlashAnimation {
    started_at: Option<Duration>,
    animation_duration: Duration,
}

impl MuzzleFlashAnimation {
    fn new(animation_duration: Duration) -> Self {
        Self {
            started_at: None,
            animation_duration,
        }
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

fn setup_world(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 1.5)));
}

fn spawn_muzzle_flash_image(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    muzzle_flash_images: Res<MuzzleFlashImages>,
) {
    let mesh_handle = meshes.add(Rectangle::from_length(IMAGE_SIZE));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(muzzle_flash_images.image_handles[0].clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        MuzzleFlashAnimation::new(ANIMATION_DURATION),
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle),
        Transform::default(),
        Visibility::Hidden,
    ));
}

fn start_animation_on_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut muzzle_flash_animations: Query<&mut MuzzleFlashAnimation>,
    time: Res<Time>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for mut muzzle_flash_animation in muzzle_flash_animations.iter_mut() {
            muzzle_flash_animation.started_at = Some(time.elapsed());
        }
    }
}

fn rotate_muzzle_flash_animation_on_arrow_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut transforms: Query<&mut Transform, With<MuzzleFlashAnimation>>,
) {
    for pressed in input.get_pressed() {
        let y_rotation_degrees: f32 = match *pressed {
            KeyCode::ArrowLeft => -1.0,
            KeyCode::ArrowRight => 1.0,
            _ => continue,
        };

        for mut transform in transforms.iter_mut() {
            transform.rotate_axis(Dir3::Y, y_rotation_degrees.to_radians());
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

fn draw_gizmos(mut gizmos: Gizmos, transforms: Query<&Transform, With<MuzzleFlashAnimation>>) {
    for transform in transforms.iter() {
        gizmos.rect(transform.to_isometry(), Vec2::splat(IMAGE_SIZE), GREY.with_alpha(0.05));
    }
}
