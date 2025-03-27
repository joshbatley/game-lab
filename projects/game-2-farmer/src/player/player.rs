use std::collections::HashMap;
use crate::asset_folder;
use crate::player::animation::{PlayerAnimation, PlayerAnimationsIndices};
use crate::player::{Directions, PlayerDirection, PlayerDirectionChange, PlayerResource, PlayerSpriteSheet, PlayerStateChange, PLAYER_HEIGHT, PLAYER_PADDING, PLAYER_SPRITE_SIZE, PLAYER_WIDTH, SPRITE_SHEET_CONFIG};
use bevy::asset::{AssetServer, Assets};
use bevy::log::info;
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy::sprite::Sprite;
use game_lab_utils::texture_atlas_layout::texture_atlas_layout_with_padding;
use std::time::Duration;
use crate::player::AnimationStates;

#[derive(Component)]
pub struct Player {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub state: AnimationStates,
}

pub fn initialize_player_resources(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
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

    commands.insert_resource(PlayerResource {
        sprite_sheet_config
    });
}

pub fn initialize_player(mut commands: Commands, player_resources: Res<PlayerResource>) {
    let default_state = player_resources.sprite_sheet_config.get(&AnimationStates::default()).unwrap();
    let animation_indices = PlayerAnimationsIndices::from_dir(Directions::default(), default_state.column_size);

    commands.spawn((
        Player {
            walk_speed: 1.3,
            run_speed: 2.0,
            state: AnimationStates::default(),
        },
        PlayerDirection(Directions::default()),
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
    player_ent: Single<(Entity, &Player)>,
    player_resource: Res<PlayerResource>,
) {
    let (entity, player) = player_ent.into_inner();
    for event in reader.read() {
        let direction = event.0;

        info!("Updating player direction, {:?}", direction);
        let columns = player_resource.sprite_sheet_config.get(&player.state).unwrap().column_size;
        commands
            .entity(entity)
            .insert(PlayerAnimationsIndices::from_dir(direction, columns))
            .insert(PlayerDirection(direction));
    }
}


pub fn update_player_state(mut reader: EventReader<PlayerStateChange>, mut player: Single<&mut Player>) {
    for event in reader.read() {
        if event.new_state == player.state {
            continue;
        }

        info!("Updating player State, {:?}", player.state);
        player.state = event.new_state;
    }
}

pub fn update_player_sprite_sheet(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &Player, &PlayerDirection), Changed<Player>>,
    player_resource: Res<PlayerResource>,
) {
    for (entity, mut sprite, player, direction) in query.iter_mut() {
        let config = player_resource.sprite_sheet_config.get(&player.state).unwrap();
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


// TODO:
// 3. Fix character controls - maybe start building the firs controller for it?