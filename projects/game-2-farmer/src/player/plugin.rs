use crate::player::player::*;
use bevy::app::{App, Startup};
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};
use crate::player::animation::animated_player_sprite;
use crate::player::debug::{debug_player_state, draw_bounding_box};
use crate::player::movement::move_player;
use crate::player::{PlayerDirectionChange, PlayerStateChange};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDirectionChange>()
            .add_event::<PlayerStateChange>()
            .add_systems(Startup, (initialize_player_resources, initialize_player).chain())
            .add_systems(Update, move_player)
            .add_systems(Update, (update_player_state, update_player_direction, update_player_sprite_sheet))
            .add_systems(Update, animated_player_sprite)
            .add_systems(Update, (draw_bounding_box, debug_player_state));
    }
}
