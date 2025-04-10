use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::Image;
use bevy::math::{IVec2, Vec2, vec3};
use bevy::prelude::{
    Commands, Component, Entity, Event, EventReader, EventWriter, Query, Res, ResMut, Resource,
    Sprite, Startup, TextureAtlas, TextureAtlasLayout, Transform, UVec2, Update,
};

#[derive(Resource)]
pub struct MapResources {
    atlas_handle: Handle<TextureAtlasLayout>,
    tile_map_handle: Handle<Image>,
}
#[derive(Component)]
pub struct Tile {
    pub sprite_index: IVec2,
    pub position: Vec2,
    pub index: i32,
}
#[derive(Event)]
pub struct LevelChangeEvent;

#[derive(Event)]
pub struct TileCreationEvent(pub usize, pub Vec2, pub i32);

#[derive(Resource)]
pub struct MapMeta {
    size: (i32, i32),
    total_count: i32,
    sprite_size: i32,
    atlas_path: String,
    columns: u32,
    rows: u32,
    pub level_data: Vec<Vec<usize>>,
    pub level_mask: Vec<Vec<usize>>,
}
impl MapMeta {
    pub fn translate_index_to_coords(&self, i: i32) -> (i32, i32) {
        let x: i32 = i % self.size.0;
        let y: i32 = i / self.size.1;
        (x, y)
    }
    pub fn translate_coords_to_transform(&self, coords: (i32, i32)) -> Vec2 {
        let x_pos = coords.0 * self.sprite_size;
        let y_pos = coords.1 * self.sprite_size;
        Vec2::new(x_pos as f32, -y_pos as f32)
    }
    pub fn translate_index_to_transform(&self, i: i32) -> Vec2 {
        self.translate_coords_to_transform(self.translate_index_to_coords(i))
    }
    pub fn translate_transform_to_index(&self, i: Vec2) -> i32 {
        if !self.within_grid(i) {
            return -1;
        }

        let x = (i.x as i32 / self.sprite_size).abs();
        let y = ((i.y as i32 / self.sprite_size) * self.size.0).abs();

        y + x
    }
    pub fn translate_coords_to_index(&self, pos: (i32, i32)) -> i32 {
        (pos.1 * self.size.0) + pos.0
    }
    pub fn get_center_point(&self) -> Vec2 {
        let c = (self.sprite_size * self.size.0) / 2;
        Vec2::new(c as f32, -c as f32)
    }
    pub fn within_grid(&self, i: Vec2) -> bool {
        if i.x < 0.0 || i.y > 0.0 {
            return false;
        }
        let max_size = (self.size.0 * self.sprite_size) as f32;
        if i.x >= max_size || i.y <= -max_size {
            return false;
        }
        true
    }
}

pub struct MapGenerator {
    size: (i32, i32),
    atlas_path: String,
    columns: u32,
    rows: u32,
    level_data: Vec<Vec<usize>>,
}
impl Plugin for MapGenerator {
    fn build(&self, app: &mut App) {
        let map_meta = MapMeta {
            size: self.size,
            sprite_size: 32,
            atlas_path: self.atlas_path.clone(),
            total_count: self.size.0 * self.size.1,
            columns: self.columns,
            rows: self.rows,
            level_data: self.level_data.clone(),
            level_mask: vec![vec![]],
        };

        app.add_event::<LevelChangeEvent>()
            .add_event::<TileCreationEvent>()
            .insert_resource(map_meta)
            .add_systems(Startup, (load_assets, generate_tiles))
            .add_systems(Update, (generate_sprites, generate_layer_mask));
    }
}

impl MapGenerator {
    pub fn new(level_data: Vec<Vec<usize>>) -> Self {
        MapGenerator {
            size: (15, 15),
            atlas_path: String::from("internal/dungeon-stuff/tiles/dungeon-tiles.png"),
            columns: 12,
            rows: 10,
            level_data,
        }
    }
}

fn load_assets(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    map_meta: Res<MapMeta>,
) {
    let atlas = TextureAtlasLayout::from_grid(
        UVec2::splat(map_meta.sprite_size as u32),
        map_meta.columns,
        map_meta.rows,
        None,
        None,
    );
    commands.insert_resource(MapResources {
        atlas_handle: texture_atlas_layouts.add(atlas),
        tile_map_handle: asset_server.load(map_meta.atlas_path.clone()),
    })
}

fn generate_tiles(
    mut commands: Commands,
    mut writer: EventWriter<LevelChangeEvent>,
    map_meta: Res<MapMeta>,
) {
    for i in 0..map_meta.total_count {
        let (x, y) = map_meta.translate_index_to_coords(i);
        let transform = map_meta.translate_coords_to_transform((x, y));
        commands.spawn(Tile {
            sprite_index: IVec2::new(x, y),
            position: transform,
            index: i,
        });
    }

    writer.send(LevelChangeEvent);
}

fn generate_layer_mask(mut map_meta: ResMut<MapMeta>, reader: EventReader<LevelChangeEvent>) {
    if reader.is_empty() {
        return;
    }
    map_meta.level_mask = map_meta.level_data.iter()
        .map(|i| i.iter().map(|i| if *i == 47 { 0 } else { 1 }).collect())
        .collect();
}

fn generate_sprites(
    mut commands: Commands,
    mut ev_create_coin: EventWriter<TileCreationEvent>,
    query: Query<(Entity, &Tile)>,
    map_meta: Res<MapMeta>,
    map_resources: Res<MapResources>,
    reader: EventReader<LevelChangeEvent>,
) {
    if reader.is_empty() {
        return;
    }

    for (ent, tile) in query.iter() {
        let tile_index =
            map_meta.level_data[tile.sprite_index.y as usize][tile.sprite_index.x as usize];

        commands.entity(ent).insert((
            Sprite::from_atlas_image(
                map_resources.tile_map_handle.clone(),
                TextureAtlas {
                    layout: map_resources.atlas_handle.clone(),
                    index: tile_index,
                },
            ),
            Transform::from_translation(vec3(tile.position.x, tile.position.y, 0.0)),
        ));
        ev_create_coin.send(TileCreationEvent(tile_index, tile.position, tile.index));
    }
}
