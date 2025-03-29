use crate::player::player::*;
use bevy::app::{App, Startup};
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};
use crate::player::animation::animated_player_sprite;
use crate::player::debug::{debug_player_state, draw_bounding_box};
use crate::player::movement::{apply_modifiers, modify_player_direction, modify_player_position, PlayerMovementEvent};
use crate::player::{PlayerDirectionChange, PlayerAnimationChange};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDirectionChange>()
            .add_event::<PlayerAnimationChange>()
            .add_event::<PlayerMovementEvent>()
            .add_systems(Startup, (initialize_player_resources, initialize_player).chain())
            .add_systems(Update, (apply_modifiers, move_player))
            .add_systems(Update, (update_animation_state, update_player_direction, update_player_sprite_sheet))
            .add_systems(Update, animated_player_sprite)
            .add_systems(Update, (draw_bounding_box, debug_player_state))
            .add_observer(modify_player_direction)
            .add_observer(modify_player_position);
    }
}
