use bevy::prelude::{Event, EventReader, EventWriter, Single, Trigger, With};
use crate::controller::{Action, ActionEvent, Direction};
use crate::player::player::{Player, PlayerDirection};

#[derive(Event, Debug)]
pub struct PlayerDirectionChange(pub Direction);

#[derive(Event)]
pub struct PlayerMovementEvent(pub Direction);

pub fn modify_player_direction(
    trigger: Trigger<ActionEvent>,
    mut direction_writer: EventWriter<PlayerDirectionChange>,
    player: Single<&PlayerDirection, With<Player>>
) {
    let event = trigger.event();
    if let Action::Look(direction) = event.0 {
        if direction != player.0 {
            direction_writer.send(PlayerDirectionChange(direction));
        }
    }
}

pub fn modify_player_position(trigger: Trigger<ActionEvent>, mut position_writer: EventWriter<PlayerMovementEvent>) {
    let event = trigger.event();
    if let Action::Move(direction) = event.0 {
        position_writer.send(PlayerMovementEvent(direction));
    }
}

pub fn apply_actions(mut action_reader: EventReader<ActionEvent>, mut player: Single<&mut Player>) {
    for event in action_reader.read() {
        match event {
            ActionEvent(Action::Modifier, 1) => if !player.is_running { player.is_running = true },
            ActionEvent(Action::Modifier, 0) => player.is_running = false,
            _ => {}
        }
    }
}
