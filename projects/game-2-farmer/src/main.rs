mod camera;
mod player;
mod controller;

use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{ImagePlugin, PluginGroup};
use game_lab_utils::internal_asset_plugin::InternalAssetPlugin;
use game_lab_utils::debug_plugin::{DebugPlugin};
use crate::camera::initialize_camera;
use crate::controller::ControllerPlugin;
use crate::player::plugin::PlayerPlugin;

const ASSET_ROOT_FOLDER: &str = "hana-caraka";
fn asset_folder(folder: &str) -> String {
    format!("{}/{}", ASSET_ROOT_FOLDER, folder)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(InternalAssetPlugin::new())
            .set(ImagePlugin::default_nearest()))
        .add_plugins(DebugPlugin::new(true))
        .add_plugins(ControllerPlugin::new())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, initialize_camera)
        .run();
}