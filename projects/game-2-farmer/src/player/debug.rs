use std::fmt::{Display};
use bevy::color::palettes::css::GREY;
use bevy::math::Isometry2d;
use bevy::prelude::{GizmoPrimitive2d, Gizmos, Rectangle, Single, Transform, With};
use bevy::sprite::Sprite;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Color32;
use crate::player::animation::{PlayerTimers, PlayerAnimationsIndices, AnimationState, PlayerAnimationState};
use crate::player::player::{Player, PlayerDirection};

impl Display for AnimationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimationState::Idle => write!(f, "Idle"),
            AnimationState::Walking => write!(f, "Walking"),
            AnimationState::Running => write!(f, "Running"),
        }
    }
}

pub fn draw_bounding_box(
    mut gizmos: Gizmos,
    player: Single<(&Transform, &Sprite), With<Player>>,
) {
    let (transform, sprite) = player.into_inner();
    let mut translation = transform.translation.truncate();
    let size = sprite.custom_size.unwrap_or_default();
    translation.y = translation.y + (size.y / 2.0);
    gizmos.primitive_2d(
        &Rectangle::new(size.x, size.y),
        Isometry2d::from_translation(translation),
        GREY,
    );
}

pub fn debug_player_state(mut ctx: EguiContexts, query: Single<(&Player, &Sprite, &PlayerAnimationsIndices, &PlayerDirection, &PlayerTimers, &PlayerAnimationState)>) {
    let (player, sprite, animation_indices, direction, timers, state) = query.into_inner();
    egui::Window::new("PlayerState").max_width(300.0).resizable([false,false]).movable(false).show(ctx.ctx_mut(), |ui| {
        ui.scope(|ui| {

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .max_col_width(150.0)
                .show(ui, |ui| {
                    ui.colored_label(Color32::WHITE, "State:");
                    ui.label(format!("{}", state.0));
                    ui.end_row();

                    ui.label("Direction:");
                    ui.label(format!("{}", direction.0));
                    ui.end_row();

                    ui.label("Animation_indices:");
                    ui.label(format!("{:?}", animation_indices));
                    ui.end_row();

                    ui.label("Animations index:");
                    ui.label(format!("{:?}", sprite.texture_atlas.clone().unwrap().index.clone() % animation_indices.column_size));
                    ui.end_row();

                    ui.label("Animations Time:");
                    ui.label(format!("{:?}", timers.animations.duration()));
                    ui.end_row();

                    ui.label("is_running:");
                    ui.label(format!("{:?}", player.is_running));
                    ui.end_row();
                });
        });
    });
}
