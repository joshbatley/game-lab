use std::time::Duration;
use bevy::app::{App, Plugin, Update};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{IntoSystemConfigs, Res, ResMut, Resource};
use bevy::time::common_conditions::on_timer;
use bevy_egui::{egui, EguiContexts};
use crate::debug_plugin::debug_enable;

#[derive(Resource)]
struct InternalDiagnostics {
    fps: f64,
}

pub struct DiagnosticPlugin;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InternalDiagnostics { fps: 0.0 })
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, (update_fps.run_if(on_timer(Duration::from_secs_f32(0.100))), diagnostics.run_if(debug_enable)));
    }
}

fn update_fps(mut diagnostics: ResMut<InternalDiagnostics>, store: Res<DiagnosticsStore>) {
    if let Some(fps) = store.get(&FrameTimeDiagnosticsPlugin::FPS) {
        diagnostics.fps = fps.value().unwrap_or(0.0);
    } else {
        diagnostics.fps = 0.0;
    }
}

fn diagnostics(mut ctx: EguiContexts, diagnostics: Res<InternalDiagnostics>) {
    egui::TopBottomPanel::top("FPS Display").show(ctx.ctx_mut(), |ui| {
        ui.label(format!("FPS: {:.2}", diagnostics.fps));
    });
}
