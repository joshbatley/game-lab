use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::basic::PURPLE;
use bevy::math::Isometry2d;
use bevy::prelude::{Camera2d, Commands, GizmoPrimitive2d, Gizmos, IntoSystemConfigs, Rectangle, Single, Transform, With, Without};
use game_lab_utils::debug_plugin::debug_enable;
use crate::player::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera)
            .add_systems(Update, move_camera)
            .add_systems(Update, debug_camera.run_if(debug_enable));
    }
}

const CAMERA_ZONE: f32 = 200.0;

pub fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(1920.0, -1920.0, 0.0) // play pos
    ));
}

pub fn move_camera(camera: Single<(&Camera2d, &mut Transform), Without<Player>>, player: Single<(&Player, &Transform), With<Player>>) {
    let (_, mut cam_transform) = camera.into_inner();
    let (player, player_transform) = player.into_inner();

    let borders: [f32; 4] = [
        cam_transform.translation.x + CAMERA_ZONE / 2.0, // right
        cam_transform.translation.x - CAMERA_ZONE / 2.0, // left
        cam_transform.translation.y + CAMERA_ZONE / 2.0, // up
        cam_transform.translation.y - CAMERA_ZONE / 2.0, // down
    ];

    let speed = if player.is_running { player.run_speed } else { player.walk_speed };

    if player_transform.translation.x < borders[0] {
        cam_transform.translation.x -= speed;
    }

    if player_transform.translation.x > borders[1] {
        cam_transform.translation.x += speed;
    }

    if player_transform.translation.y < borders[2] {
        cam_transform.translation.y -= speed;
    }

    if player_transform.translation.y > borders[3] {
        cam_transform.translation.y += speed;
    }
}

pub fn debug_camera(mut gizmos: Gizmos, camera: Single<&mut Transform, With<Camera2d>>) {
    gizmos.primitive_2d(
        &Rectangle::new(CAMERA_ZONE, CAMERA_ZONE),
        Isometry2d::from_translation(camera.translation.truncate()),
        PURPLE,
    );
}