use bevy::app::{App, Plugin, Startup, Update};
use game_lab_utils::debug_plugin::Debugger;
use crate::controller::ActionEvent;
use crate::controller::basic_controller::{initialize_basic_controller, look_controller, modifier_controller, movement_controller};
use crate::controller::debug::debug_controller;

// I need to extend this, so I work out how to do controller, mouse etc
pub struct ControllerPlugin;
impl ControllerPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_debug_system(debug_controller, "Controller".to_string())
            .add_systems(Startup, initialize_basic_controller)
            .add_systems(Update, (look_controller, movement_controller, modifier_controller));
    }
}

