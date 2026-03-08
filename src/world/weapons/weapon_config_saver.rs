use bevy::{
    asset::{AssetLoader, AsyncWriteExt, saver::AssetSaver},
};
use thiserror::Error;

use super::{weapon_config::WeaponConfig, weapon_config_loader::WeaponConfigLoader};

#[derive(Default)]
pub struct WeaponConfigSaver;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum WeaponConfigSaverError {
    /// A [RON](ron) serialization error
    #[error("Could not serialize to RON: {0}")]
    Ron(#[from] ron::error::Error),
    /// An [IO](std::io) error during writing
    #[error("Could not write asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetSaver for WeaponConfigSaver {
    type Asset = WeaponConfig;
    type Settings = ();
    type OutputLoader = WeaponConfigLoader;
    type Error = WeaponConfigSaverError;

    async fn save(
        &self,
        writer: &mut bevy::asset::io::Writer,
        asset: bevy::asset::saver::SavedAsset<'_, Self::Asset>,
        _settings: &Self::Settings,
    ) -> Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error> {
        let serialized = ron::ser::to_string(asset.get())?;
        writer.write_all(serialized.as_bytes()).await?;
        Ok(())
    }
}
