use crate::player::player::*;
use crate::player::animation::{animated_player_sprite, update_player_animation_indices, update_player_animation_state, update_sprite_texture_atlas};
use crate::player::debug::{debug_player_state, draw_sprite_bounding_box, draw_target_block};
use crate::player::controller::{apply_actions, modify_player_direction, modify_player_position, PlayerDirectionChange, PlayerMovementEvent};
use bevy::app::{App, Startup};
use bevy::prelude::{IntoSystemConfigs, Plugin, Update};
use bevy::sprite::Material2dPlugin;
use game_lab_utils::debug_plugin::{debug_enable, Debugger};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDirectionChange>()
            .add_event::<PlayerMovementEvent>()
            .add_plugins(Material2dPlugin::<CustomMaterial>::default(),)
            .add_systems(Startup, (initialize_player_resources, initialize_player).chain())
            .add_systems(Update, (apply_actions, update_player_transform))
            .add_systems(Update, (update_player_direction, move_shadow, update_player_animation_state, update_sprite_texture_atlas, animated_player_sprite, update_player_animation_indices, update_player_target))
            .add_systems(Update, (draw_sprite_bounding_box.run_if(debug_enable), draw_target_block.run_if(debug_enable)))
            .add_debug_system(debug_player_state, "Player".to_string())
            .add_observer(modify_player_direction)
            .add_observer(modify_player_position);
    }
}
