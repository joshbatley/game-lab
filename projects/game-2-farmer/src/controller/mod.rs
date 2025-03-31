mod debug;
pub mod plugin;
mod basic_controller;

use std::collections::HashMap;
use bevy::prelude::{Component, Event, KeyCode, Resource};

// TODO: Add loads of config
// Loads needs to be updated here, but for now it all works as I need for this game.
// Future goals:
// - Sort out actions so there are more generic
// - add config for settings
// - and systems/events to update controls
// - Support controller types, mouse, gamepad etc, point and click and so on
// - Support for directional changes and modifiers send a vec 2
// - Support for dead zones and smoothing etc
// - Better Debugging tools
// - Error handling if a controller is left unset it will blow up but for now its fine, one to think of when setting custom keys
// - Support for multiple keys to one actions

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self { Direction::South }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Action {
    // Movement Directions
    Move(Direction),

    // Look Directions
    Look(Direction),

    // Actions - Events
    Interact,   // e?
    Jump,       // Space
    Modifier,   // E.g. Shift
    Sneak,      // Control

    // Game State
    Pause,    // ESC
}

#[derive(Event)]
pub struct ActionEvent(pub Action, pub i32);

#[derive(Component)]
struct Controller {
    last_move_action: Vec<Action>,
    last_look_action: Option<Action>,
}

#[derive(Resource)]
struct ControllerSettings {
    pub controls: HashMap<Action, KeyCode>,
}


// TODO: Add methods
// - Get movement keys
// - Allow updating of settings
impl ControllerSettings {
    pub fn actions_to_keys(&self, actions: [Action; 4]) -> Vec<KeyCode> {
        actions.iter()
            .map(|action| self.controls[action].clone())
            .collect()
    }
}

