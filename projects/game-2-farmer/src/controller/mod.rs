use std::collections::HashMap;
use std::fmt::Display;
use bevy::app::App;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Component, Event, EventWriter, KeyCode, Plugin, Res, ResMut, Resource, Single, Startup, Update};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Color32;
use game_lab_utils::debug_plugin::DebugState;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ActionDirections {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Action {
    // Movement Directions
    Move(ActionDirections),

    // Look Directions
    Look(ActionDirections),

    // Actions - Events
    Interact,   // e?
    Jump,       // Space
    Modifier,   // E.g. Shift
    Sneak,      // Control

    // Game State
    Pause,    // ESC
    Idle,     // No event
}

#[derive(Event)]
pub struct ActionEvent(pub Action);

#[derive(Resource)]
pub struct ControllerSettings {
    pub controls: HashMap<Action, KeyCode>,
}

#[derive(Component)]
pub struct Controller {
    last_move_action: Vec<Action>,
    last_look_action: Option<Action>,
}

impl ControllerSettings {
    pub fn actions_to_keys(&self, actions: [Action; 4]) -> Vec<KeyCode> {
        let mut keys = vec!();
        for action in actions {
            keys.push(self.controls[&action].clone());
        }
        keys
    }
}

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
            .add_systems(Startup, initialize_controller)
            .add_systems(Update, (look_controller, movement_controller, modifier_controller))
            .add_systems(Update, debug_player_state);
    }
}

pub fn initialize_controller(mut commands: Commands) {
    let mut controls = HashMap::new();

    controls.insert(Action::Look(ActionDirections::North), KeyCode::KeyW);
    controls.insert(Action::Look(ActionDirections::East), KeyCode::KeyD);
    controls.insert(Action::Look(ActionDirections::South), KeyCode::KeyS);
    controls.insert(Action::Look(ActionDirections::West), KeyCode::KeyA);

    controls.insert(Action::Move(ActionDirections::North), KeyCode::KeyW);
    controls.insert(Action::Move(ActionDirections::East), KeyCode::KeyD);
    controls.insert(Action::Move(ActionDirections::South), KeyCode::KeyS);
    controls.insert(Action::Move(ActionDirections::West), KeyCode::KeyA);

    // controls.insert(Action::Look(ActionDirections::North), KeyCode::ArrowUp);
    // controls.insert(Action::Look(ActionDirections::East), KeyCode::ArrowRight);
    // controls.insert(Action::Look(ActionDirections::South), KeyCode::ArrowDown);
    // controls.insert(Action::Look(ActionDirections::West), KeyCode::ArrowLeft);

    commands.insert_resource(ControllerSettings { controls });
    commands.spawn(Controller{ last_move_action: Vec::new(), last_look_action: None });
}

fn movement_controller(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<ControllerSettings>,
    mut controller: Single<&mut Controller>
) {
    let directions = [
        Action::Move(ActionDirections::North), Action::Move(ActionDirections::West),
        Action::Move(ActionDirections::South), Action::Move(ActionDirections::East)];

    for direction in directions {
        let key = settings.controls[&direction];

        if keys.just_pressed(key) && keys.pressed(key) {
            controller.last_move_action.push(direction.clone());
        }

        if keys.just_released(key)  {
            let x = controller.last_move_action.iter().position(|x| x == &direction).unwrap();
            controller.last_move_action.remove(x);
        }

    }

    if !controller.last_move_action.is_empty() {
        commands.trigger(ActionEvent(controller.last_move_action[controller.last_move_action.len()-1]));
    }
}

// Follow direction look
fn look_controller(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    settings: ResMut<ControllerSettings>,
    mut controller: Single<&mut Controller>
) {
    let directions = [
        Action::Look(ActionDirections::North), Action::Look(ActionDirections::West),
        Action::Look(ActionDirections::South), Action::Look(ActionDirections::East)];

    for direction in directions {
        let key = settings.controls[&direction];

        if keys.pressed(key) {
            controller.last_look_action = Some(direction);
        }

        let new_direction = if keys.just_pressed(key)  {
            Some(direction)
        } else if keys.just_released(key) && keys.any_pressed(settings.actions_to_keys(directions)) {
            controller.last_look_action
        } else {
            None
        };

        if new_direction.is_some() {
            commands.trigger(ActionEvent(new_direction.unwrap()));
        }
    }
}

// so we should need to think analog stick even for arrow keys.
// So we send when there is any event. I guess we should have a threshold where we dont need to send everything
// for just the look should it fire all the time or just if its a change


fn modifier_controller(mut action_writer: EventWriter<ActionEvent>, keys: Res<ButtonInput<KeyCode>>) {
    // // Modifier - triggers and bumpers
    if keys.pressed(KeyCode::ShiftLeft) && keys.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyS]) {
        action_writer.send(ActionEvent(Action::Modifier));
    }

    // // DEFAULT - Idle think no actions
    if !keys.any_pressed([KeyCode::ShiftLeft]) {
        action_writer.send(ActionEvent(Action::Idle));
    }
}


impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move(direction) => write!(f, "Move({:?})", direction),
            Action::Look(direction) => write!(f, "Look({:?})", direction),
            Action::Interact => write!(f, "Interact"),
            Action::Jump => write!(f, "Jump"),
            Action::Modifier => write!(f, "Modifier"),
            Action::Sneak => write!(f, "Sneak"),
            Action::Pause => write!(f, "Pause"),
            Action::Idle => write!(f, "Idle"),

        }
    }
}


pub fn debug_player_state(mut ctx: EguiContexts, debug_state: ResMut<DebugState>, controller: Single<&Controller>) {
    if !debug_state.enabled {
        return;
    }

    egui::Window::new("Controller").max_width(300.0).resizable([false,false]).movable(false).show(ctx.ctx_mut(), |ui| {
        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .max_col_width(150.0)
            .show(ui, |ui| {
                ui.colored_label(Color32::WHITE, "Last Move Action :");
                ui.label(format!("{:?}", controller.last_move_action));
                ui.end_row();

                ui.colored_label(Color32::WHITE, "Last Look Action :");
                ui.label(if controller.last_look_action.is_some() { format!("{}", controller.last_look_action.unwrap())} else { "None".to_string()} );
                ui.end_row();
            });
    });
}

