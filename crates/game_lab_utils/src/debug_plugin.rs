use std::collections::BTreeMap;
use std::time::Duration;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::ButtonInput;
use bevy::prelude::{IntoSystemConfigs, KeyCode, Res, ResMut, Resource};
use bevy::time::common_conditions::on_timer;
use bevy_egui::egui::{FontId, TextStyle};
use bevy_egui::egui::FontFamily::{Monospace, Proportional};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::epaint::text::{FontInsert, InsertFontFamily};

#[derive(Default, Resource)]
pub struct DebugState {
    pub enabled: bool,
}

#[derive(Resource)]
struct InternalDiagnostics {
    fps: f64,
}

pub struct DebugPlugin {
    enabled: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugState{enabled: self.enabled})
            .insert_resource(InternalDiagnostics { fps: 0.0 })
            .add_plugins(EguiPlugin)
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, load_and_set_egui_fonts)
            .add_systems(Update, (toggle_debug, diagnostics))
            .add_systems(Update, update_fps.run_if(on_timer(Duration::from_secs_f32(0.100))), );
    }
}

impl DebugPlugin {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

fn toggle_debug(keys: Res<ButtonInput<KeyCode>>, mut engine_state: ResMut<DebugState>) {
    if keys.just_pressed(KeyCode::Backquote) {
        engine_state.enabled = !engine_state.enabled;
    }
}

fn load_and_set_egui_fonts(contexts: EguiContexts) {
    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(18.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ].into();
    
    contexts.ctx().all_styles_mut(move |style| style.text_styles = text_styles.clone());
    contexts.ctx().add_font(FontInsert::new(
        "debugger_font",
        egui::FontData::from_static(include_bytes!("../../../assets/fonts/debug/debug.ttf")),
        vec![
            InsertFontFamily {
                family: Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
        ]));
}

fn update_fps(mut diagnostics: ResMut<InternalDiagnostics>, store: Res<DiagnosticsStore>) {
    diagnostics.fps = store.get(&FrameTimeDiagnosticsPlugin::FPS).unwrap().value().unwrap();
}

fn diagnostics(mut ctx: EguiContexts, diagnostics: Res<InternalDiagnostics>) {
    egui::Window::new("FPS Display").show(ctx.ctx_mut(), |ui| {
        ui.label(format!("FPS: {:.2}", diagnostics.fps));
    });
}