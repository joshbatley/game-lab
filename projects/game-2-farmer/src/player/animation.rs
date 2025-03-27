use bevy::prelude::{Component, Res, Single, Sprite, Time, Timer};
use crate::player::{Directions};

#[derive(Component, Debug)]
pub struct PlayerAnimationsIndices {
    pub first: usize,
    pub last: usize,
    pub column_size: usize,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub timer: Timer,
}
impl PlayerAnimationsIndices {
    pub fn from_dir(dir: Directions, cols: u32) -> Self {
        let first = Self::get_first_index(dir, cols);

        Self {
            first,
            last: first + (cols as usize) - 1,
            column_size: cols as usize,
        }
    }

    fn get_first_index(dir: Directions, cols: u32) -> usize {
        match dir {
            Directions::Right => 0 * cols as usize,
            Directions::Left => 1 * cols as usize,
            Directions::Down => 2 * cols as usize,
            Directions::Up => 3 * cols as usize,
        }
    }
}

pub fn animated_player_sprite(
    time: Res<Time>,
    player: Single<(&mut Sprite, &mut PlayerAnimation, &PlayerAnimationsIndices)>,
) {
    let (mut sprite, mut player_animation, animation_indices) = player.into_inner();

    player_animation.timer.tick(time.delta());
    if !player_animation.timer.just_finished() {
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
            dir: Directions,
            expected_first: usize,
            expected_last: usize,
        }

        let cases = vec!(
            TestCase {
                dir: Directions::Right,
                expected_first: 0,
                expected_last: 7,
            },
            TestCase {
                dir: Directions::Left,
                expected_first: 8,
                expected_last: 15,
            },
            TestCase {
                dir: Directions::Down,
                expected_first: 16,
                expected_last: 23,
            },
            TestCase {
                dir: Directions::Up,
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
            dir: Directions,
            expected_first: usize,
            expected_last: usize,
        }

        let cases = vec!(
            TestCase {
                dir: Directions::Right,
                expected_first: 0,
                expected_last: 3,
            },
            TestCase {
                dir: Directions::Left,
                expected_first: 4,
                expected_last: 7,
            },
            TestCase {
                dir: Directions::Down,
                expected_first: 8,
                expected_last: 11,
            },
            TestCase {
                dir: Directions::Up,
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