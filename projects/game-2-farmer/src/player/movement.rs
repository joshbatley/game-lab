use bevy::input::ButtonInput;
use bevy::prelude::{EventWriter, KeyCode, Res, Single, Transform};
use crate::player::player::{Player};
use crate::player::{AnimationStates, Directions, PlayerDirectionChange, PlayerStateChange};

pub fn move_player(
    mut state_writer: EventWriter<PlayerStateChange>,
    mut direction_writer: EventWriter<PlayerDirectionChange>,
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Single<(&mut Transform, &mut Player)>,
) {
    let (mut transform, player) = player_q.into_inner();
    let is_running = keys.pressed(KeyCode::ShiftLeft);
    let speed = if is_running { player.run_speed } else { player.walk_speed };
    let mut new_state = if is_running { AnimationStates::Running } else { AnimationStates::Walking };

    let preformed = vec![
        key_action(KeyCode::KeyW, Directions::Up, speed, &mut transform, &mut direction_writer, &keys),
        key_action(KeyCode::KeyA, Directions::Left, -speed, &mut transform, &mut direction_writer, &keys),
        key_action(KeyCode::KeyS, Directions::Down, -speed, &mut transform, &mut direction_writer, &keys),
        key_action(KeyCode::KeyD, Directions::Right, speed, &mut transform, &mut direction_writer, &keys),
    ].iter().any(|&x| x);

    if !preformed {
        new_state = AnimationStates::Idle;
    }

    state_writer.send(PlayerStateChange { new_state });
}

fn key_action(
    key: KeyCode,
    dir: Directions,
    speed: f32,
    transform: &mut Transform,
    direction_writer: &mut EventWriter<PlayerDirectionChange>,
    keys: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let mut preformed = false;
    if keys.just_pressed(key) {
        direction_writer.send(PlayerDirectionChange(dir));
        preformed = true;
    }

    if keys.pressed(key) {
        match dir {
            Directions::Left => transform.translation.x += speed,
            Directions::Right => transform.translation.x += speed,
            Directions::Down => transform.translation.y += speed,
            Directions::Up => transform.translation.y += speed,
        }
        preformed = true;
    }
    preformed
}