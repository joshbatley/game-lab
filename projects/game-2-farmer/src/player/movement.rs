use bevy::log::info;
use bevy::prelude::{Event, EventReader, EventWriter, Single, Trigger, With};
use crate::controller::{Action, ActionEvent};
use crate::player::player::{Player};
use crate::player::{AnimationState, Direction, PlayerDirection, PlayerDirectionChange, PlayerAnimationChange};

#[derive(Event)]
pub struct PlayerMovementEvent(pub Direction);

pub fn modify_player_direction(
    trigger: Trigger<ActionEvent>,
    mut direction_writer: EventWriter<PlayerDirectionChange>,
    player: Single<&PlayerDirection, With<Player>>
) {
    let event = trigger.event();
    if let Action::Look(direction) = event.0 {
        let direction =  Direction::from_action(direction);
        if let Some(direction) = direction {
            if direction != player.0 {
                direction_writer.send(PlayerDirectionChange(direction));
            }
        }
    }

}

pub fn modify_player_position(
    trigger: Trigger<ActionEvent>,
    mut state_writer: EventWriter<PlayerAnimationChange>,
    mut position_writer: EventWriter<PlayerMovementEvent>,
    player: Single<&mut Player>,
) {
    let event = trigger.event();
    if let Action::Move(direction) = event.0 {
        info!("Updating player position, {:?}", direction);
        let new_state = if player.is_running { AnimationState::Running  } else { AnimationState::Walking };
        state_writer.send(PlayerAnimationChange { new_state });
        position_writer.send(PlayerMovementEvent(Direction::from_action(direction).unwrap()));
    }
}

pub fn apply_modifiers(
    mut action_reader: EventReader<ActionEvent>,
    mut player: Single<&mut Player>,
) {
    for event in action_reader.read() {
        let action = event.0;
        match action {
            Action::Modifier => player.is_running = true,
            _ => player.is_running = false
        }

    }
}
