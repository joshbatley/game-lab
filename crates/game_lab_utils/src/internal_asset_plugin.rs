use bevy::asset::AssetPlugin;

pub const INTERNAL_ASSET_FILE_PATH: &str = "../../assets";
pub struct InternalAssetPlugin;
impl InternalAssetPlugin {
    pub fn new() -> AssetPlugin {
        AssetPlugin{
            file_path: INTERNAL_ASSET_FILE_PATH.to_string(),
            ..Default::default()
        }
    }
}

