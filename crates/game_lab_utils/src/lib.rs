pub mod internal_asset_plugin;

use bevy::app::App;
use bevy::prelude::{Plugin};

pub struct DiagnosticPlugin;

impl Plugin for  DiagnosticPlugin {
    fn build(&self, _: &mut App) {
        todo!()
    }
}

