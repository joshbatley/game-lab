mod cursor;
mod game;
mod levels;
mod map_plugin;
mod player_plugin;
mod utils;

use crate::cursor::CursorPlugin;
use crate::game::GamePlugin;
use crate::levels::level_to_map;
use crate::map_plugin::MapGenerator;
use crate::player_plugin::PlayerPlugin;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use game_lab_utils::internal_asset_plugin::InternalAssetPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(InternalAssetPlugin::new())
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PlayerPlugin::new())
        .add_plugins(MapGenerator::new(level_to_map(1)))
        .add_plugins(CursorPlugin::new())
        .add_plugins(GamePlugin {})
        .run();
}
