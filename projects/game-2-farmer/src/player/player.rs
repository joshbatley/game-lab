use crate::asset_folder;
use crate::player::animation::{PlayerTimers, PlayerAnimationsIndices, PlayerAnimationState, AnimationState};
use crate::player::controller::{PlayerDirectionChange, PlayerMovementEvent};
use crate::player::sprite_sheet::{PlayerSpriteSheet, SPRITE_SHEET_CONFIG};
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite};
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use crate::controller::Direction;

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub is_running: bool,
}

#[derive(Resource)]
pub struct PlayerResource {
    pub sprite_sheet_config: HashMap<AnimationState, PlayerSpriteSheet>,
}

#[derive(Component)]
pub struct PlayerDirection(pub Direction);

pub fn initialize_player_resources(mut commands: Commands, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, asset_server: Res<AssetServer>) {
    let mut sprite_sheet_config = HashMap::new();

    for ss in SPRITE_SHEET_CONFIG {
        let atlas = TextureAtlasLayout::from_grid(ss.tile_size, ss.columns, ss.rows, None, None);
        sprite_sheet_config.insert(ss.state, PlayerSpriteSheet::new(
            asset_server.load(asset_folder(ss.image_url)),
            texture_atlas_layouts.add(atlas),
            ss.columns,
            Duration::from_millis(ss.frame_duration),
            ss.sprite_size,
            ss.rendered_area
        ));
    }

    commands.insert_resource(PlayerResource { sprite_sheet_config });
}

pub fn initialize_player(mut commands: Commands, player_resources: Res<PlayerResource>) {
    let default_state = player_resources.sprite_sheet_config.get(&AnimationState::default()).unwrap();
    let animation_indices = PlayerAnimationsIndices::from_dir(Direction::default(), default_state.columns);

    commands.spawn((
        Player {
            walk_speed: 1.3,
            run_speed: 2.0,
            is_running: false,
        },
        PlayerAnimationState(AnimationState::default()),
        PlayerDirection(Direction::default()),
        Sprite {
            image: default_state.image_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: default_state.atlas_layout_handle.clone(),
                index: animation_indices.first,
            }),
            custom_size: Some(default_state.sprite_size),
            rect: Some(default_state.render_area),
            anchor: Anchor::BottomCenter,
            ..Default::default()
        },
        animation_indices,
        PlayerTimers {
            animations: Timer::new(default_state.duration, TimerMode::Repeating),
        },
        Transform::default(),
    ));
}

pub fn update_player_direction(mut reader: EventReader<PlayerDirectionChange>, mut direction: Single<&mut PlayerDirection>, ) {
    for event in reader.read() {
        let new_direction = event.0;
        if new_direction == direction.0 {
            return;
        }

        direction.0 = new_direction;
    }
}

pub fn update_player_transform(
    mut reader: EventReader<PlayerMovementEvent>,
    player: Single<(&mut Transform, &Player)>,
) {
    let (mut transform, player) = player.into_inner();
    let speed = if player.is_running { player.run_speed } else { player.walk_speed };
    for event in reader.read() {
        let direction = event.0;
        match direction {
            Direction::North => transform.translation.y += speed,
            Direction::West => transform.translation.x += -speed,
            Direction::South => transform.translation.y += -speed,
            Direction::East => transform.translation.x += speed,
        }
    }
}