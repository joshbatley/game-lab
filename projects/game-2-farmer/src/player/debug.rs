use std::fmt::{Display};
use bevy::color::palettes::css::GREY;
use bevy::math::Isometry2d;
use bevy::prelude::{GizmoPrimitive2d, Gizmos, Rectangle, Res, ResMut, Single, Transform, With};
use bevy::sprite::Sprite;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Color32;
use game_lab_utils::debug_plugin::DebugState;
use crate::player::animation::{PlayerAnimation, PlayerAnimationsIndices};
use crate::player::player::{Player};
use crate::player::{AnimationStates, Directions, PlayerDirection, PLAYER_SPRITE_SIZE};

impl Display for AnimationStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimationStates::Idle => write!(f, "Idle"),
            AnimationStates::Walking => write!(f, "Walking"),
            AnimationStates::Running => write!(f, "Running"),
        }
    }
}

impl Display for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Directions::Left => write!(f, "Left"),
            Directions::Right => write!(f, "Right"),
            Directions::Up => write!(f, "Up"),
            Directions::Down => write!(f, "Down"),
        }
    }
}

pub fn draw_bounding_box(
    mut gizmos: Gizmos,
    engine_state: Res<DebugState>,
    player: Single<&Transform, With<Player>>,
) {
    if !engine_state.enabled {
        return;
    }

    gizmos.primitive_2d(
        &Rectangle::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE),
        Isometry2d::from_translation(player.translation.truncate()),
        GREY,
    );
}

pub fn debug_player_state(mut ctx: EguiContexts, debug_state: ResMut<DebugState>, query: Single<(&Player, &Sprite, &PlayerAnimationsIndices, &PlayerDirection, &PlayerAnimation)>) {
    if !debug_state.enabled {
        return;
    }
    let (player, _, animation_indices, direction, animation) = query.into_inner();
    egui::Window::new("PlayerState").max_width(300.0).resizable([false,false]).movable(false).show(ctx.ctx_mut(), |ui| {
        ui.scope(|ui| {

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .max_col_width(150.0)
                .show(ui, |ui| {
                    ui.colored_label(Color32::WHITE, "State:");
                    ui.label(format!("{}", player.state));
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

