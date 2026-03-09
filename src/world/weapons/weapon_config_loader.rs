use bevy::{asset::AssetLoader, reflect::TypePath};
use thiserror::Error;

use super::weapon_config::WeaponConfig;

#[derive(Default, TypePath)]
pub struct WeaponConfigLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum WeaponConfigLoaderError {
    /// An [IO](std::io) error during reading
    #[error("Could not read asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) deserialization error
    #[error("Could not parse RON: {0}")]
    Ron(#[from] ron::de::SpannedError),
}

impl AssetLoader for WeaponConfigLoader {
    type Asset = WeaponConfig;
    type Settings = ();
    type Error = WeaponConfigLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let weapon_config = ron::de::from_bytes(&bytes)?;
        Ok(weapon_config)
    }
}
