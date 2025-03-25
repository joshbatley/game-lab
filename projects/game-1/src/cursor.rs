use crate::map_plugin::{MapMeta, Tile};
use crate::player_plugin::Player;
use crate::utils::{bfs, get_ray_vec, vec_to_nearest};
use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::math::{Rect, Vec2, Vec3, vec2};
use bevy::prelude::{
    Camera, Color, Commands, Component, Event, EventReader, EventWriter, GlobalTransform, Plugin,
    Query, Res, Single, Sprite, Startup, Transform, Update, Window, With,
};

#[derive(Component)]
struct Cursor;

#[derive(Event)]
struct HighlightEvent(pub i32);


pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HighlightEvent>()
            .add_systems(Startup, setup_cursor)
            .add_systems(Update, (update_mouse_box, highlight_tiles));
    }
}

impl CursorPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

fn setup_cursor(mut commands: Commands, mut window: Single<&mut Window>, asset_server: Res<AssetServer>) {
    window.cursor_options.visible = false;

    commands.spawn((
        Cursor,
        Sprite {
            image: asset_server.load("sprout-lands/ui/icons/select.png"),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            rect: Some(Rect::new(0.0, 0.0, 32.0, 32.0)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_mouse_box(
    mut writer: EventWriter<HighlightEvent>,
    mut cursor: Single<&mut Transform, With<Cursor>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    map_meta: Res<MapMeta>,
) {
    let ray = vec_to_nearest(get_ray_vec(camera, window), 32.0);
    let pos = Vec3::new(ray.x, ray.y, 11.0);

    if pos != cursor.translation {
        cursor.translation = pos;
        writer.send(HighlightEvent(
            map_meta.translate_transform_to_index(vec2(pos.x, pos.y)),
        ));
    }
}

fn highlight_tiles(
    mut reader: EventReader<HighlightEvent>,
    mut query: Query<(&Tile, &mut Sprite)>,
    map_meta: Res<MapMeta>,
    player: Single<&Player>,
) {
    for event in reader.read() {
        let transform = map_meta.translate_index_to_transform(event.0);
        if !map_meta.within_grid(vec2(transform.x, transform.y)) {
            continue;
        }
        let player_coords = map_meta.translate_index_to_coords(player.index);
        let cursor_coords = map_meta.translate_index_to_coords(event.0);
        let tiles = bfs(&map_meta.level_mask, player_coords, cursor_coords);

        for (tile, mut sprite) in query.iter_mut() {
            if tiles.contains(&map_meta.translate_index_to_coords(tile.index)) {
                sprite.color = Color::srgba(1.3, 1.3, 1.3, 1.0);
            } else {
                sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            }
        }
    }
}
