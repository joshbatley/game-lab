mod camera;
mod player;
mod controller;

use bevy::app::{App, Startup};
use bevy::color::palettes::css::GREY;
use bevy::DefaultPlugins;
use bevy::math::{Isometry2d, UVec2, Vec2};
use bevy::prelude::{Gizmos, ImagePlugin, IntoSystemConfigs, PluginGroup, Update};
use game_lab_utils::internal_asset_plugin::InternalAssetPlugin;
use game_lab_utils::debug_plugin::{debug_enable, DebugPlugin};
use crate::camera::initialize_camera;
use crate::controller::plugin::ControllerPlugin;
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
        .add_systems(Update, cheap_map.run_if(debug_enable))
        .run();
}


fn cheap_map(mut gizmos: Gizmos) {
    gizmos.grid_2d(Isometry2d::IDENTITY, UVec2::splat(32), Vec2::splat(48.0), GREY);
}
