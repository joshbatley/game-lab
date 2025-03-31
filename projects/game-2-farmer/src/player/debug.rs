use std::fmt::{Display};
use bevy::color::palettes::css::GREY;
use bevy::math::Isometry2d;
use bevy::prelude::{GizmoPrimitive2d, Gizmos, Rectangle, Single, Transform, With};
use bevy::sprite::Sprite;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Color32;
use crate::player::animation::{PlayerAnimation, PlayerAnimationsIndices};
use crate::player::player::{Player};
use crate::player::{AnimationState, Direction, PlayerAnimationState, PlayerDirection, PLAYER_SPRITE_SIZE};

impl Display for AnimationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimationState::Idle => write!(f, "Idle"),
            AnimationState::Walking => write!(f, "Walking"),
            AnimationState::Running => write!(f, "Running"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

pub fn draw_bounding_box(
    mut gizmos: Gizmos,
    player: Single<&Transform, With<Player>>,
) {
    gizmos.primitive_2d(
        &Rectangle::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE),
        Isometry2d::from_translation(player.translation.truncate()),
        GREY,
    );
}

pub fn debug_player_state(mut ctx: EguiContexts, query: Single<(&Player, &Sprite, &PlayerAnimationsIndices, &PlayerDirection, &PlayerAnimation, &PlayerAnimationState)>) {
    let (_, _, animation_indices, direction, animation, state) = query.into_inner();
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

                    ui.label("Animations Time:");
                    ui.label(format!("{:?}", animation.timer.duration()));
                    ui.end_row();
                });
        });
    });
}
