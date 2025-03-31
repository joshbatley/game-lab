use crate::asset_folder;
use crate::player::{AnimationState, PlayerAnimationState};
use crate::player::animation::{PlayerAnimation, PlayerAnimationsIndices};
use crate::player::{
    Direction, PLAYER_HEIGHT, PLAYER_PADDING, PLAYER_SPRITE_SIZE, PLAYER_WIDTH, PlayerDirection,
    PlayerDirectionChange, PlayerResource, PlayerSpriteSheet, PlayerAnimationChange,
    SPRITE_SHEET_CONFIG,
};
use bevy::asset::{AssetServer, Assets};
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy::sprite::Sprite;
use game_lab_utils::texture_atlas_layout::texture_atlas_layout_with_padding;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use crate::player::movement::PlayerMovementEvent;

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub is_running: bool,
}

pub fn initialize_player_resources(mut commands: Commands, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, asset_server: Res<AssetServer>) {
    let mut sprite_sheet_config = HashMap::new();
    for (state, image_url, cols, rows, animation_time) in SPRITE_SHEET_CONFIG {
        let atlas = texture_atlas_layout_with_padding(uvec2(PLAYER_HEIGHT, PLAYER_WIDTH), cols, rows, PLAYER_PADDING);

        sprite_sheet_config.insert(state, PlayerSpriteSheet::new(
            asset_server.load(asset_folder(image_url)),
            texture_atlas_layouts.add(atlas),
            cols,
            Duration::from_millis(animation_time),
        ));
    }

    commands.insert_resource(PlayerResource { sprite_sheet_config });
}

pub fn initialize_player(mut commands: Commands, player_resources: Res<PlayerResource>) {
    let default_state = player_resources.sprite_sheet_config.get(&AnimationState::default()).unwrap();
    let animation_indices = PlayerAnimationsIndices::from_dir(Direction::default(), default_state.column_size);

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
            custom_size: Some(vec2(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE)),
            ..Default::default()
        },
        animation_indices,
        PlayerAnimation {
            timer: Timer::new(default_state.duration, TimerMode::Repeating),
        },
        Transform::default(),
    ));
}

pub fn update_player_direction(
    mut reader: EventReader<PlayerDirectionChange>,
    mut commands: Commands,
    player_ent: Single<(Entity, &PlayerAnimationState, &mut PlayerDirection)>,
    player_resource: Res<PlayerResource>,
) {
    let (entity, animation_state, mut player_direction) = player_ent.into_inner();
    for event in reader.read() {
        let new_direction = event.0;
        if new_direction == player_direction.0 {
            return;
        }

        let columns = player_resource.sprite_sheet_config.get(&animation_state.0).unwrap().column_size;
        player_direction.0 = new_direction;

        commands
            .entity(entity)
            .insert(PlayerAnimationsIndices::from_dir(new_direction, columns));
    }
}


pub fn update_animation_state(mut reader: EventReader<PlayerAnimationChange>, mut state: Single<&mut PlayerAnimationState>) {
    if reader.is_empty() && state.0 != AnimationState::Idle {
        state.0 =  AnimationState::Idle;
    }

    for event in reader.read() {
        if event.new_state == state.0 {
            continue;
        }

        state.0 = event.new_state;
    }
}

pub fn update_player_sprite_sheet(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &PlayerAnimationState, &PlayerDirection), Changed<PlayerAnimationState>>,
    player_resource: Res<PlayerResource>,
) {
    for (entity, mut sprite, state, direction) in query.iter_mut() {
        let config = player_resource.sprite_sheet_config.get(&state.0).unwrap();
        let n_animation_indices = PlayerAnimationsIndices::from_dir(direction.0, config.column_size);

        sprite.image = config.image_handle.clone();
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = n_animation_indices.first;
            atlas.layout = config.atlas_layout_handle.clone();
        }
        commands.entity(entity)
            .insert(n_animation_indices)
            .insert(PlayerAnimation { timer: Timer::new(config.duration, TimerMode::Repeating) });
    }
}

pub fn move_player(
    mut reader: EventReader<PlayerMovementEvent>,
    player_q: Single<(&mut Transform, &Player)>,
) {
    let (mut transform, player) = player_q.into_inner();
    let running =  player.is_running;
    let speed = if running { player.run_speed } else { player.walk_speed };
    for event in reader.read() {
        let direction = event.0;
        match direction {
            Direction::Up => transform.translation.y += speed,
            Direction::Left => transform.translation.x += -speed,
            Direction::Down => transform.translation.y += -speed,
            Direction::Right => transform.translation.x += speed,
        }
    }
}