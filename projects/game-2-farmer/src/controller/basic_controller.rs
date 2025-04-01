use std::collections::HashMap;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, EventWriter, KeyCode, Res, Single};
use crate::controller::{Action, Direction, ActionEvent, Controller, ControllerSettings};

const MOVE_DIRECTIONS: [Action; 4] = [
    Action::Move(Direction::North), Action::Move(Direction::West),
    Action::Move(Direction::South), Action::Move(Direction::East)];

const LOOK_DIRECTIONS: [Action; 4] = [
    Action::Look(Direction::North), Action::Look(Direction::West),
    Action::Look(Direction::South), Action::Look(Direction::East)];

const MODIFIER_ACTIONS: [Action; 5] = [
    Action::Modifier, Action::Interact,
    Action::Jump, Action::Pause, Action::Sneak
];

pub fn initialize_basic_controller(mut commands: Commands) {
    let mut controls = HashMap::new();

    controls.insert(Action::Look(Direction::North),vec!(KeyCode::KeyW,KeyCode::ArrowUp));
    controls.insert(Action::Look(Direction::East),vec!(KeyCode::KeyD, KeyCode::ArrowRight));
    controls.insert(Action::Look(Direction::South),vec!(KeyCode::KeyS, KeyCode::ArrowDown));
    controls.insert(Action::Look(Direction::West), vec!(KeyCode::KeyA, KeyCode::ArrowLeft));


    controls.insert(Action::Move(Direction::North), vec!(KeyCode::KeyW));
    controls.insert(Action::Move(Direction::East), vec!(KeyCode::KeyD));
    controls.insert(Action::Move(Direction::South), vec!(KeyCode::KeyS));
    controls.insert(Action::Move(Direction::West),vec!( KeyCode::KeyA));

    controls.insert(Action::Interact, vec!(KeyCode::KeyE));
    controls.insert(Action::Modifier, vec!(KeyCode::ShiftLeft));
    controls.insert(Action::Jump,vec!( KeyCode::Space));
    controls.insert(Action::Pause,vec!( KeyCode::Escape));
    controls.insert(Action::Sneak, vec!(KeyCode::ControlLeft));

    commands.insert_resource(ControllerSettings { controls, has_conflict: false });
    commands.spawn(Controller{ last_move_action: Vec::new(), last_look_action: None });
}

pub fn movement_controller(
    mut commands: Commands,
    mut controller: Single<&mut Controller>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<ControllerSettings>,
) {
    for direction in MOVE_DIRECTIONS {
        let key = &settings.controls[&direction];

        if keys.any_just_pressed(key.clone()) && keys.any_pressed(key.clone()) {
            controller.last_move_action.push(direction.clone());
        }

        if keys.any_just_released(key.clone())  {
            let action = controller.last_move_action.iter().position(|x| x == &direction).unwrap();
            controller.last_move_action.remove(action);
        }
    }

    if !controller.last_move_action.is_empty() {
        commands.trigger(ActionEvent(controller.last_move_action[controller.last_move_action.len()-1], 1));
    }
}

pub fn look_controller(
    mut commands: Commands,
    mut controller: Single<&mut Controller>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<ControllerSettings>,
) {
    let directions = LOOK_DIRECTIONS;

    for direction in directions {
        let key = &settings.controls[&direction];

        if keys.any_pressed(key.clone()) {
            controller.last_look_action = Some(direction);
        }

        let new_direction = if keys.any_just_pressed(key.clone())  {
            Some(direction)
        } else if keys.any_just_released(key.clone()) && keys.any_pressed(settings.actions_to_keys(directions)) {
            controller.last_look_action
        } else {
            None
        };

        if new_direction.is_some() {
            commands.trigger(ActionEvent(new_direction.unwrap(), 1));
        }
    }
}


pub fn modifier_controller(
    mut action_writer: EventWriter<ActionEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<ControllerSettings>,
) {
    for actions in MODIFIER_ACTIONS {
        let key = &settings.controls[&actions].clone();

        if keys.any_pressed(key.clone())  {
            action_writer.send(ActionEvent(actions, 1));
        }
        if keys.any_just_released(key.clone()) {
            action_writer.send(ActionEvent(actions, 0));
        }
    }
}
