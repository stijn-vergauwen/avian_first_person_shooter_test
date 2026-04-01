use bevy::{
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_skybox_assets)
            .add_systems(Update, reinterpret_skybox_image_on_asset_loaded);
    }
}

#[derive(Resource)]
pub struct SkyboxAssets {
    pub skybox_image: Handle<Image>,
}

fn setup_skybox_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SkyboxAssets {
        skybox_image: asset_server.load("textures/skybox_test.png"),
    });
}

fn reinterpret_skybox_image_on_asset_loaded(
    mut reader: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    skybox_assets: Res<SkyboxAssets>,
) {
    for message in reader.read() {
        let AssetEvent::LoadedWithDependencies { id } = message else {
            continue;
        };

        if skybox_assets.skybox_image.id() != *id {
            continue;
        }

        let skybox_image = images.get_mut(*id).unwrap();

        skybox_image
                .reinterpret_stacked_2d_as_array(skybox_image.height() / skybox_image.width())
                .expect("asset should be 2d texture and height will always be evenly divisible with the given layers");

        skybox_image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }
}
