use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                setup_muzzle_flash_images,
                setup_world,
                spawn_muzzle_flash_image,
            ).chain(),
        )
        .run();
}

#[derive(Resource)]
struct MuzzleFlashImages {
    image_handles: [Handle<Image>; 3],
}

fn setup_muzzle_flash_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_paths = [
        "textures/Muzzle flash sprites test/Backside frame 1.png",
        "textures/Muzzle flash sprites test/Backside frame 2.png",
        "textures/Muzzle flash sprites test/Backside frame 3.png",
    ];
    let image_handles = sprite_paths.map(|path| asset_server.load(path));
    commands.insert_resource(MuzzleFlashImages { image_handles });
}

fn setup_world(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 1.0)));
}

fn spawn_muzzle_flash_image(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    muzzle_flash_images: Res<MuzzleFlashImages>,
) {
    let mesh_handle = meshes.add(Rectangle::from_length(1.0));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(muzzle_flash_images.image_handles[0].clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle),
        Transform::default(),
    ));
}
