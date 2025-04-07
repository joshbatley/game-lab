use std::time::Duration;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Asset, AssetServer, Assets, Handle};
use bevy::image::Image;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Commands, Component, Query, Res, ResMut, Resource, Single, TextureAtlas, TextureAtlasLayout, Time, Timer, TimerMode, Transform, TypePath, With};
use bevy::sprite::Sprite;
use bevy_common_assets::json::JsonAssetPlugin;
use ::serde::Deserialize;
use crate::{asset_folder_sprout};

pub struct MapPlugin { }

#[derive(Asset, TypePath, Deserialize)]
struct MapData {
    map: Layer,
}

#[derive(Deserialize)]
struct Layer {
    layer: Vec<Data>
}

#[derive(Deserialize)]
struct Data {
    data: Content
}
#[derive(Deserialize)]
struct Content {
    content: Vec<isize>
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<MapData>::new(&[".json"]))
            .add_systems(Startup, setup)
            .add_systems(Update, (load_level, water_tile));

            // .add_systems(Update, );//.add_systems(Update, update);
    }
}

#[derive(Resource)]
struct MapState {
    level: Handle<MapData>,
    // layer: u32,
    image: Handle<Image>,
    water: Handle<Image>,
    atlas: Handle<TextureAtlasLayout>,
    water_atlas: Handle<TextureAtlasLayout>,
    completed: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) {
    let handle = asset_server.load("maps/game2/tiled/level_0.json");
    let image = asset_server.load(asset_folder_sprout("tilesets/grass/grass.png"));
    let water = asset_server.load(asset_folder_sprout("tilesets/water.png"));
    let atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16),39,7,None, None));
    let water_atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(16),4,1,None, None));
    commands.insert_resource(MapState {
        level: handle,
        image,
        atlas,
        water_atlas,
        // layer: 1,
        water,
        completed: false,
    });

    commands.spawn(WaterTimer{ timer: Timer::new(Duration::from_millis(300), TimerMode::Repeating) });
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct WaterTile;

#[derive(Component)]
struct WaterTimer {
   timer: Timer,
}

fn load_level(mut commands: Commands, datas: Res<Assets<MapData>>, mut map: ResMut<MapState>) {
    if map.completed {
        return;
    }

    if let  Some(t) = datas.get(map.level.id()) {
        for layer_index in 0..t.map.layer.len() {
           let layer = t.map.layer.get(layer_index).unwrap();
            if layer_index == 0 {
                for x in 0..layer.data.content.len() {
                    // let tile = layer.data.content[x];
                    let xpos = (((x % 100) * 32) as f32) - 16.0;
                    let ypos = -(((x / 100) * 32) as f32) - 16.0;
                    commands.spawn((
                        WaterTile,
                        Sprite {
                            image: map.water.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: map.water_atlas.clone(),
                                index: x % 4,
                            }),
                            custom_size: Some(Vec2::splat(32.0)),
                            ..Default::default()
                        },
                        // Transform::from_xyz(0.0,0.0,1.0)
                        Transform::from_xyz(xpos as f32, ypos as f32,0.0)
                    ));
                }
            } else {
                for x in 0..layer.data.content.len() {
                    let tile = layer.data.content[x];
                    let xpos = (((x % 100) * 32) as f32) - 16.0;
                    let ypos = -(((x / 100) * 32) as f32) - 16.0;

                    if tile == -1 {
                        continue;
                    }
                    commands.spawn((
                        Tile,
                        Sprite {
                            image: map.image.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: map.atlas.clone(),
                                index: tile as usize,
                            }),
                            custom_size: Some(Vec2::splat(32.0)),
                            ..Default::default()
                        },
                        // Transform::from_xyz(0.0,0.0,1.0)
                        Transform::from_xyz(xpos as f32, ypos as f32, 1.0)
                    ));
                }
            }
        }
        map.completed = true;
    }
}

fn water_tile(mut query: Query<&mut Sprite, With<WaterTile>>, time: Res<Time>, mut timer: Single<&mut WaterTimer>) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    for mut sprite in query.iter_mut() {

        if let Some(atlas) = &mut sprite.texture_atlas {
            // info!("water_tile: {:?}", atlas.index);
            atlas.index = if atlas.index == 3 {
                0
            } else {
                atlas.index + 1
            };
            // info!("water_tile: {:?}", atlas.index);
        }
        // info!("water_tile: {:?}", &sprite.texture_atlas.clone().unwrap().index.clone());
        // info!("-----");

    }
}