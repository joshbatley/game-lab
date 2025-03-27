use crate::map_plugin::{LevelChangeEvent, MapMeta};
use crate::utils::{bfs, get_ray_vec, vec_to_nearest};
use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::math::{Rect, Vec2, vec3};
use bevy::prelude::{
    ButtonInput, Camera, Commands, Component, Event, EventReader, EventWriter, GlobalTransform,
    KeyCode, Query, Res, ResMut, Resource, Single, Sprite, Transform, Update, Window,
};
use bevy::time::{Time, Timer, TimerMode};
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Component)]
pub struct Player {
    pub is_moving: bool,
    pub index: i32,
}

#[derive(Event)]
struct MovePlayer(pub Vec2);

#[derive(Event)]
pub struct PlayerPositionUpdated(pub i32);

#[derive(Resource)]
pub struct PlayerMovement {
    movement: VecDeque<(i32, i32)>,
    timer: Timer,
}

pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovePlayer>()
            .add_event::<PlayerPositionUpdated>()
            .insert_resource(PlayerMovement {
                movement: VecDeque::new(),
                timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            })
            .add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (move_player, create_directions_for_player, transform_player, update_player_position,),
            );
    }
}

impl PlayerPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, map_meta: Res<MapMeta>) {
    let transform = map_meta.translate_index_to_transform(16);
    commands.spawn((
        Player {
            is_moving: false,
            index: 16,
        },
        Sprite {
            image:  asset_server.load("dungeon-stuff/characters/archer/archer-idle-front.png"),
            custom_size: Some(Vec2::splat(32.0)),
            rect: Some(Rect::new(0.0, 0.0, 24.0, 24.0)),
            ..Default::default()
        },
        Transform::from_xyz(transform.x, transform.y, 10.0),
    ));
}

fn move_player(
    mut player: Single<&mut Player>,
    mut writer: EventWriter<MovePlayer>,
    player_movement: Res<PlayerMovement>,
    keys: Res<ButtonInput<KeyCode>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let is_moving = player.is_moving;
    if is_moving {
        if player_movement.movement.is_empty() {
            player.is_moving = false;
        }
        return;
    }
    if keys.just_pressed(KeyCode::Space) && !is_moving {
        player.is_moving = true;
        let vec = vec_to_nearest(get_ray_vec(camera, window), 32.0);
        writer.send(MovePlayer(vec));
    }
}

fn create_directions_for_player(
    mut reader: EventReader<MovePlayer>,
    mut player_movement: ResMut<PlayerMovement>,
    player: Single<&Player>,
    map_meta: Res<MapMeta>,
) {
    for event in reader.read() {
        let cursor_index = map_meta.translate_transform_to_index(event.0);
        let coords = map_meta.translate_index_to_coords(cursor_index);

        if cursor_index == -1 || map_meta.level_mask[coords.1 as usize][coords.0 as usize] != 0 {
            return;
        }

        bfs(&map_meta.level_mask, map_meta.translate_index_to_coords(player.index), coords)
            .iter().for_each(|m| player_movement.movement.push_back(m.clone()));
    }
}

fn transform_player(
    mut player: Query<(&mut Transform, &mut Player)>,
    mut player_movement: ResMut<PlayerMovement>,
    mut writer: EventWriter<PlayerPositionUpdated>,
    map_meta: Res<MapMeta>,
    time: Res<Time>,
) {
    player_movement.timer.tick(time.delta());

    if player_movement.timer.finished() && !player_movement.movement.is_empty() {
        let movement = player_movement.movement.pop_front().unwrap();
        let (mut player_transform, mut player) = player.single_mut();
        let new_transform = map_meta.translate_coords_to_transform(movement);

        player_transform.translation = vec3(new_transform.x, new_transform.y, 11.0);
        player.index = map_meta.translate_coords_to_index(movement);
        writer.send(PlayerPositionUpdated(
            map_meta.translate_coords_to_index(movement),
        ));
    }
}

fn update_player_position(reader: EventReader<LevelChangeEvent>, mut player: Query<(&mut Transform, &mut Player)>, map_meta: Res<MapMeta>) {
    if reader.is_empty() {
        return;
    }
    let (mut player_transform, mut player) = player.single_mut();
    let transform = map_meta.translate_index_to_transform(16);
    player_transform.translation = vec3(transform.x, transform.y, 11.0);
    player.index = 16;
}