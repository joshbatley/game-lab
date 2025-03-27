use bevy::prelude::{Camera2d, Commands, Transform};

pub fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}