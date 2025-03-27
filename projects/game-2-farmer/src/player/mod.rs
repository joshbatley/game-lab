use std::collections::HashMap;
use std::time::Duration;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{Component, Event, Resource, TextureAtlasLayout};

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

const SPRITE_SHEET_CONFIG: [(AnimationStates, &str, u32, u32, u64); 3] = [
    (AnimationStates::Idle, "character/basic/idle.png", PLAYER_SMALL_COLS, PLAYER_ROWS, 200),
    (AnimationStates::Walking, "character/basic/walk.png", PLAYER_LARGE_COLS, PLAYER_ROWS, 100),
    (AnimationStates::Running, "character/basic/run.png", PLAYER_LARGE_COLS, PLAYER_ROWS, 100),
];


#[derive(Clone, Copy, Debug)]
pub enum Directions {
    Left,
    Right,
    Down,
    Up,
}

impl Default for Directions {
    fn default() -> Self { Directions::Down }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AnimationStates {
    Idle,
    Walking,
    Running,
}

impl Default for AnimationStates {
    fn default() -> Self { AnimationStates::Idle }
}

#[derive(Event, Debug)]
pub struct PlayerDirectionChange(pub Directions);

#[derive(Event)]
pub struct PlayerStateChange {
    pub new_state: AnimationStates
}


#[derive(Component)]
pub struct PlayerDirection(pub Directions);


#[derive(Resource)]
pub struct PlayerResource {
    sprite_sheet_config: HashMap<AnimationStates, PlayerSpriteSheet>,
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