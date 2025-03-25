use crate::levels::level_to_map;
use crate::map_plugin::{LevelChangeEvent, MapMeta, TileCreationEvent};
use crate::player_plugin::PlayerPositionUpdated;
use bevy::app::{App, Startup, Update};
use bevy::asset::AssetServer;
use bevy::math::{Rect, Vec2, vec3};
use bevy::prelude::{
    Camera2d, Commands, Component, Entity, EventReader, EventWriter, Plugin, Query, Res, ResMut,
    Resource, Sprite, Text, TextFont, Transform, With, default,
};

#[derive(Resource, Default)]
pub struct Game {
    coins: i32,
    level: i32,
}

#[derive(Component)]
struct Coin {
    index: i32,
}

#[derive(Component)]
struct CoinsText;

pub struct GamePlugin {}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world)
            .add_systems(Update, (clear_coins, create_coins))
            .add_systems(Update, (update_level, text_update_system, coin_collected));
    }
}

fn setup_world(mut commands: Commands, map_meta: Res<MapMeta>) {
    commands.insert_resource(Game { level: 1, coins: 0 });

    let pos = map_meta.get_center_point();
    commands.spawn((Camera2d, Transform::from_xyz(pos.x, pos.y, 0.0)));
    commands.spawn((
        Text::new("Coins: "),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        CoinsText,
    ));
}

fn clear_coins(
    mut commands: Commands,
    mut game: ResMut<Game>,
    reader: EventReader<LevelChangeEvent>,
    query: Query<Entity, With<Coin>>,
) {
    if reader.is_empty() {
        return;
    }
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    game.coins = 0;
}

fn create_coins(
    mut reader: EventReader<TileCreationEvent>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    let coin_handle = asset_server.load("dungeon-stuff/objects/coins.png");
    for event in reader.read() {
        if event.0 != 47 {
            continue;
        }
        let pos = event.1;
        commands.spawn((
            Coin { index: event.2 },
            Sprite {
                image: coin_handle.clone(),
                custom_size: Some(Vec2::splat(32.0)),
                rect: Some(Rect::new(0.0, 0.0, 32.0, 32.0)),
                ..Default::default()
            },
            Transform::from_translation(vec3(pos.x, pos.y, 1.0)),
        ));
        game.coins += 1;
    }
}

fn update_level(
    mut game: ResMut<Game>,
    mut map_meta: ResMut<MapMeta>,
    mut writer: EventWriter<LevelChangeEvent>,
) {
    if game.coins == 0 {
        game.level += 1;
        map_meta.level_data = level_to_map(game.level);
        writer.send(LevelChangeEvent);
    }
    if game.level == 4 {
        game.level = 1;
        map_meta.level_data = level_to_map(game.level);
        writer.send(LevelChangeEvent);
    }
}

fn text_update_system(mut query: Query<&mut Text, With<CoinsText>>, game: Res<Game>) {
    for mut span in &mut query {
        **span = format!("Coins: {:?}", game.coins);
    }
}

fn coin_collected(
    mut reader: EventReader<PlayerPositionUpdated>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    query: Query<(Entity, &Coin)>,
) {
    for event in reader.read() {
        for (entity, coin) in query.iter() {
            if coin.index == event.0 {
                commands.entity(entity).despawn();
                game.coins -= 1;
            }
        }
    }
}
