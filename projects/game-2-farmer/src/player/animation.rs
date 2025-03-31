use bevy::prelude::{Changed, Commands, Component, Entity, EventReader, Query, Res, Single, Sprite, Time, Timer, TimerMode};
use crate::controller::Direction;
use crate::player::controller::{PlayerMovementEvent};
use crate::player::player::{Player, PlayerDirection, PlayerResource};

#[derive(Component, Debug)]
pub struct PlayerAnimationsIndices {
    pub first: usize,
    pub last: usize,
    pub column_size: usize,
}

#[derive(Component)]
pub struct PlayerTimers {
    pub animations: Timer,
}

#[derive(Component)]
pub struct PlayerAnimationState(pub AnimationState);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
}
impl Default for AnimationState {
    fn default() -> Self { AnimationState::Idle }
}

impl PlayerAnimationsIndices {
    pub fn from_dir(dir: Direction, cols: u32) -> Self {
        let first = Self::get_first_index(dir, cols);

        Self {
            first,
            last: first + (cols as usize) - 1,
            column_size: cols as usize,
        }
    }
    fn get_first_index(dir: Direction, cols: u32) -> usize {
        match dir {
            Direction::East => 0 * cols as usize,
            Direction::West => 1 * cols as usize,
            Direction::South => 2 * cols as usize,
            Direction::North => 3 * cols as usize,
        }
    }
}

pub fn update_player_animation_state(
    reader: EventReader<PlayerMovementEvent>,
    player: Single<(&Player, &mut PlayerAnimationState)>,
) {
    let (player, mut current_state) = player.into_inner();
    let is_moving = !reader.is_empty();

    let new_state = match (player.is_running, is_moving) {
        (true, true) => AnimationState::Running ,
        (false, true) => AnimationState::Walking,
        _ => AnimationState::Idle,
    };
    if new_state != current_state.0 {
        current_state.0 = new_state;
    }
}

pub fn update_player_animation_indices(
    mut commands: Commands,
    player_resource: Res<PlayerResource>,
    query: Query<(Entity, &PlayerAnimationState, &PlayerDirection), Changed<PlayerDirection>>
) {
    for (entity, animation_state, player_direction) in query.iter() {
        let columns = player_resource.sprite_sheet_config.get(&animation_state.0).unwrap().columns;

        commands
            .entity(entity)
            .insert(PlayerAnimationsIndices::from_dir(player_direction.0, columns));
    }
}

pub fn update_sprite_texture_atlas(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &PlayerAnimationState, &PlayerDirection), Changed<PlayerAnimationState>>,
    player_resource: Res<PlayerResource>,
) {
    for (entity, mut sprite, state, direction) in query.iter_mut() {
        let config = player_resource.sprite_sheet_config.get(&state.0).unwrap();
        let n_animation_indices = PlayerAnimationsIndices::from_dir(direction.0, config.columns);

        sprite.image = config.image_handle.clone();
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = n_animation_indices.first;
            atlas.layout = config.atlas_layout_handle.clone();
        }

        commands.entity(entity)
            .insert(n_animation_indices)
            .insert(PlayerTimers { animations: Timer::new(config.duration, TimerMode::Repeating) });
    }
}

pub fn animated_player_sprite(time: Res<Time>, player: Single<(&mut Sprite, &mut PlayerTimers, &PlayerAnimationsIndices)>, ) {
    let (mut sprite, mut player_animation, animation_indices) = player.into_inner();

    player_animation.animations.tick(time.delta());
    if !player_animation.animations.just_finished() {
        return;
    }

    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = if atlas.index == animation_indices.last {
            animation_indices.first
        } else {
            animation_indices.first + ((atlas.index + 1) % animation_indices.column_size)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_dir() {
        struct TestCase {
            dir: Direction,
            expected_first: usize,
            expected_last: usize,
        }

        let cases = vec!(
            TestCase {
                dir: Direction::East,
                expected_first: 0,
                expected_last: 7,
            },
            TestCase {
                dir: Direction::North,
                expected_first: 8,
                expected_last: 15,
            },
            TestCase {
                dir: Direction::South,
                expected_first: 16,
                expected_last: 23,
            },
            TestCase {
                dir: Direction::West,
                expected_first: 24,
                expected_last: 31,
            },
        );

        for c in cases {
            let a = PlayerAnimationsIndices::from_dir(c.dir, 8);
            assert_eq!(a.first, c.expected_first);
            assert_eq!(a.last, c.expected_last);
        }
    }

    #[test]
    fn from_dir_4() {
        struct TestCase {
            dir: Direction,
            expected_first: usize,
            expected_last: usize,
        }

        let cases = vec!(
            TestCase {
                dir: Direction::East,
                expected_first: 0,
                expected_last: 3,
            },
            TestCase {
                dir: Direction::West,
                expected_first: 4,
                expected_last: 7,
            },
            TestCase {
                dir: Direction::South,
                expected_first: 8,
                expected_last: 11,
            },
            TestCase {
                dir: Direction::North,
                expected_first: 12,
                expected_last: 15,
            },
        );

        for c in cases {
            let a = PlayerAnimationsIndices::from_dir(c.dir, 4);
            assert_eq!(a.first, c.expected_first);
            assert_eq!(a.last, c.expected_last);
        }
    }
}