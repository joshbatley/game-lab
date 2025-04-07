use std::time::Duration;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::math::{Rect, UVec2, Vec2};
use bevy::prelude::{TextureAtlasLayout};
use crate::player::animation::{AnimationState};

const TILE_SIZE: (u32, u32) = (80, 80);

const SPRITE_SHEET_LARGE_COLS: u32 = 8;
// const SPRITE_SHEET_MEDIUM_COLS: u32 = 6;
const SPRITE_SHEET_SMALL_COLS: u32 = 4;
const SPRITE_SHEET_ROWS: u32 = 4;
// const PLAYER_SINGLE_ROWS: u32 = 1;
const SPRITE_SIZE: (f32, f32) = (32.0, 32.0);

pub struct SpriteSheetMeta {
    pub state: AnimationState,
    pub image_url: &'static str,
    pub columns: u32,
    pub rows: u32,
    pub frame_duration: u64,
    pub sprite_size: Vec2,
    pub rendered_area: (f32, f32, f32, f32),
    pub tile_size: UVec2,
}

const fn sprite_sheet_default() -> SpriteSheetMeta {
    SpriteSheetMeta {
        state: AnimationState::Idle,
        image_url: "",
        columns: SPRITE_SHEET_LARGE_COLS,
        rows: SPRITE_SHEET_ROWS,
        frame_duration: 100,
        sprite_size: Vec2::new(SPRITE_SIZE.0, SPRITE_SIZE.1),
        rendered_area: (32.0, 32.0, 48.0, 48.0),
        tile_size: UVec2::new(TILE_SIZE.0, TILE_SIZE.0),
    }
}

pub const SPRITE_SHEET_CONFIG: [SpriteSheetMeta; 3] = [
    SpriteSheetMeta {
        state: AnimationState::Idle,
        image_url: "character/basic/idle.png",
        frame_duration: 200,
        columns: SPRITE_SHEET_SMALL_COLS,
        ..sprite_sheet_default()
    },
    SpriteSheetMeta {
        state: AnimationState::Walking,
        image_url: "character/basic/walk.png",
        ..sprite_sheet_default()
    },
    SpriteSheetMeta {
        state: AnimationState::Running,
        image_url: "character/basic/run.png",
        ..sprite_sheet_default()
    },
];

pub struct PlayerSpriteSheet {
    pub image_handle: Handle<Image>,
    pub atlas_layout_handle: Handle<TextureAtlasLayout>,
    pub columns: u32,
    pub duration: Duration,
    pub sprite_size: Vec2,
    pub render_area: Rect,
}

impl PlayerSpriteSheet {
    pub fn new(image_handle: Handle<Image>, atlas_layout_handle: Handle<TextureAtlasLayout>, columns: u32, duration: Duration, sprite_size: Vec2, render_area: (f32, f32, f32, f32)) -> Self {
        Self {
            image_handle,
            atlas_layout_handle,
            columns,
            duration,
            sprite_size,
            render_area: Rect::new(render_area.0, render_area.1, render_area.2, render_area.3),
        }
    }
}

