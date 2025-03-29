use std::collections::HashMap;
use std::time::Duration;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{Component, Event, Resource, TextureAtlasLayout};
use crate::controller::{ActionDirections};

pub mod plugin;
mod player;
mod animation;
mod debug;
mod movement;

const PLAYER_HEIGHT: u32 = 16;
const PLAYER_WIDTH: u32 = 16;
const PLAYER_PADDING: u32 = 64;
const PLAYER_LARGE_COLS: u32 = 8;
// const PLAYER_MEDIUM_COLS: u32 = 6;
const PLAYER_SMALL_COLS: u32 = 4;
const PLAYER_ROWS: u32 = 4;
// const PLAYER_SINGLE_ROWS: u32 = 1;
const PLAYER_SPRITE_SIZE: f32 = 48.0;

const SPRITE_SHEET_CONFIG: [(AnimationState, &str, u32, u32, u64); 3] = [
    (AnimationState::Idle, "character/basic/idle.png", PLAYER_SMALL_COLS, PLAYER_ROWS, 200),
    (AnimationState::Walking, "character/basic/walk.png", PLAYER_LARGE_COLS, PLAYER_ROWS, 100),
    (AnimationState::Running, "character/basic/run.png", PLAYER_LARGE_COLS, PLAYER_ROWS, 100),
];


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    pub fn from_action(action: ActionDirections) -> Option<Direction> {
        match action {
            ActionDirections::North => Some(Direction::Up),
            ActionDirections::East => Some(Direction::Right),
            ActionDirections::South => Some(Direction::Down),
            ActionDirections::West => Some(Direction::Left),
        }
    }

}

impl Default for Direction {
    fn default() -> Self { Direction::Down }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
}

#[derive(Component)]
pub struct PlayerAnimationState(pub AnimationState);

impl Default for AnimationState {
    fn default() -> Self { AnimationState::Idle }
}

#[derive(Event, Debug)]
pub struct PlayerDirectionChange(pub Direction);

#[derive(Event)]
pub struct PlayerAnimationChange {
    pub new_state: AnimationState
}


#[derive(Component)]
pub struct PlayerDirection(pub Direction);


#[derive(Resource)]
pub struct PlayerResource {
    sprite_sheet_config: HashMap<AnimationState, PlayerSpriteSheet>,
}

struct PlayerSpriteSheet {
    image_handle: Handle<Image>,
    atlas_layout_handle: Handle<TextureAtlasLayout>,
    column_size: u32,
    duration: Duration,
}

impl PlayerSpriteSheet {
    pub fn new(image_handle: Handle<Image>, atlas_layout_handle: Handle<TextureAtlasLayout>, column_size: u32, duration: Duration) -> Self {
        Self{
            image_handle,
            atlas_layout_handle,
            column_size,
            duration
        }
    }
}