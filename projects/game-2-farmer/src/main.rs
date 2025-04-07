mod camera;
mod player;
mod controller;
mod map;

use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::{ImagePlugin, PluginGroup};
use game_lab_utils::internal_asset_plugin::InternalAssetPlugin;
use game_lab_utils::debug_plugin::{DebugPlugin};
use crate::camera::CameraPlugin;
use crate::controller::plugin::ControllerPlugin;
use crate::map::MapPlugin;
use crate::player::plugin::PlayerPlugin;

const ASSET_ROOT_FOLDER_HANA: &str = "hana-caraka";
const ASSET_ROOT_FOLDER_SPROUT: &str = "sprout-lands";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(InternalAssetPlugin::new())
            .set(ImagePlugin::default_nearest()))
        .add_plugins(DebugPlugin::new(true))
        .add_plugins(ControllerPlugin::new())
        .add_plugins(MapPlugin{})
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        // .add_systems(Update, gizmo_grid.run_if(debug_enable))
        .run();
}

fn asset_folder_hana(folder: &str) -> String {
    format!("{}/{}", ASSET_ROOT_FOLDER_HANA, folder)
}

fn asset_folder_sprout(folder: &str) -> String {
    format!("{}/{}", ASSET_ROOT_FOLDER_SPROUT, folder)
}

// fn gizmo_grid(mut gizmos: Gizmos, q: Single<(&Camera, &Transform)>) {
//     let (_, transform) = q.into_inner();
//     let mut translation = transform.translation.truncate() + Vec2::new(16.0, 16.0);
//     translation.x = (translation.x / 32.0).round() * 32.0;
//     translation.y = (translation.y / 32.0).round() * 32.0;
//     gizmos.grid_2d(Isometry2d::new(translation, Rot2::IDENTITY), UVec2::splat(100), Vec2::splat(32.0), GREY);
// }
