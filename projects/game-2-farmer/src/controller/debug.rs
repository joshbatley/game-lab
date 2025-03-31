use std::fmt::Display;
use bevy::prelude::{Res, Single};
use bevy_egui::{egui, EguiContexts};
use crate::controller::{Action, Controller, ControllerSettings};

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move(direction) => write!(f, "Move({:?})", direction),
            Action::Look(direction) => write!(f, "Look({:?})", direction),
            Action::Interact => write!(f, "Interact"),
            Action::Jump => write!(f, "Jump"),
            Action::Modifier => write!(f, "Modifier"),
            Action::Sneak => write!(f, "Sneak"),
            Action::Pause => write!(f, "Pause"),
        }
    }
}

pub fn debug_controller(mut ctx: EguiContexts, controller: Single<&Controller>, settings: Res<ControllerSettings>) {
    egui::Window::new("Controller").max_width(300.0).resizable([false,false]).movable(false).show(ctx.ctx_mut(), |ui| {
        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .max_col_width(150.0)
            .show(ui, |ui| {
                ui.label("Last Move Action :");
                ui.label(format!("{:?}", controller.last_move_action));
                ui.end_row();

                ui.label("Last Look Action :");
                ui.label(if controller.last_look_action.is_some() { format!("{}", controller.last_look_action.unwrap())} else { "None".to_string()} );
                ui.end_row();

                ui.separator();
                ui.separator();
            });
        egui::CollapsingHeader::new("Controls Settings").show(ui, |ui| {
            egui::Grid::new("controls_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .max_col_width(150.0)
            .show(ui, |ui| {
                for (action, key) in &settings.controls {
                    ui.label(format!("{:?}", action));
                    ui.label(format!("{:?}", key));
                    ui.end_row();
                }
            })
        })
    });
}