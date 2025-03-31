use std::collections::HashMap;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, EventWriter, KeyCode, Res, Single};
use crate::controller::{Action, ActionDirections, ActionEvent, Controller, ControllerSettings};

const MOVE_DIRECTIONS: [Action; 4] = [
    Action::Move(ActionDirections::North), Action::Move(ActionDirections::West),
    Action::Move(ActionDirections::South), Action::Move(ActionDirections::East)];

const LOOK_DIRECTIONS: [Action; 4] = [
    Action::Look(ActionDirections::North), Action::Look(ActionDirections::West),
    Action::Look(ActionDirections::South), Action::Look(ActionDirections::East)];

pub fn initialize_basic_controller(mut commands: Commands) {
    let mut controls = HashMap::new();

    controls.insert(Action::Look(ActionDirections::North), KeyCode::KeyW);
    controls.insert(Action::Look(ActionDirections::East), KeyCode::KeyD);
    controls.insert(Action::Look(ActionDirections::South), KeyCode::KeyS);
    controls.insert(Action::Look(ActionDirections::West), KeyCode::KeyA);

    controls.insert(Action::Move(ActionDirections::North), KeyCode::KeyW);
    controls.insert(Action::Move(ActionDirections::East), KeyCode::KeyD);
    controls.insert(Action::Move(ActionDirections::South), KeyCode::KeyS);
    controls.insert(Action::Move(ActionDirections::West), KeyCode::KeyA);

    controls.insert(Action::Interact, KeyCode::KeyE);
    controls.insert(Action::Modifier, KeyCode::ShiftLeft);
    controls.insert(Action::Jump, KeyCode::Space);
    controls.insert(Action::Pause, KeyCode::Escape);
    controls.insert(Action::Sneak, KeyCode::ControlLeft);

    commands.insert_resource(ControllerSettings { controls });
    commands.spawn(Controller{ last_move_action: Vec::new(), last_look_action: None });
}

pub fn movement_controller(
    mut commands: Commands,
    mut controller: Single<&mut Controller>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<ControllerSettings>,
) {
    for direction in MOVE_DIRECTIONS {
        let key = settings.controls[&direction];

        if keys.just_pressed(key) && keys.pressed(key) {
            controller.last_move_action.push(direction.clone());
        }

        if keys.just_released(key)  {
            let action = controller.last_move_action.iter().position(|x| x == &direction).unwrap();
            controller.last_move_action.remove(action);
        }
    }

    if !controller.last_move_action.is_empty() {
        commands.trigger(ActionEvent(controller.last_move_action[controller.last_move_action.len()-1]));
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


pub fn modifier_controller(
    mut action_writer: EventWriter<ActionEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<ControllerSettings>,
) {

    if keys.pressed(settings.controls[&Action::Modifier]) && keys.any_pressed(settings.actions_to_keys(MOVE_DIRECTIONS)) {
        action_writer.send(ActionEvent(Action::Modifier));
    }

    if keys.pressed(settings.controls[&Action::Interact]) {
        action_writer.send(ActionEvent(Action::Interact));
    }

    if keys.pressed(settings.controls[&Action::Jump]) {
        action_writer.send(ActionEvent(Action::Jump));
    }

    if keys.pressed(settings.controls[&Action::Pause]) {
        action_writer.send(ActionEvent(Action::Pause));
    }

    if keys.pressed(settings.controls[&Action::Sneak]) {
        action_writer.send(ActionEvent(Action::Sneak));
    }
}
