use std::any::{TypeId};
use std::collections::{BTreeMap, HashMap};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::ButtonInput;
use bevy::prelude::{IntoSystemConfigs, KeyCode, Res, ResMut, Resource, };
use bevy::utils::default;
use bevy_egui::egui::{FontId, TextStyle};
use bevy_egui::egui::FontFamily::{Monospace, Proportional};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::epaint::text::{FontInsert, InsertFontFamily};
use crate::diagnostic_plugin::DiagnosticPlugin;

#[derive(Resource)]
pub struct DebugState {
    pub enabled: bool,
    pub keys: HashMap<String, bool>,
    pub av: Vec<TypeId>
}

impl Default for DebugState {
    fn default() -> Self {
        Self { enabled: true, keys: HashMap::new(), av: Vec::new() }
    }
}

pub struct DebugPlugin {
    pub enabled: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugState{ enabled:self.enabled,..default()})
            .add_plugins(EguiPlugin)
            .add_plugins(DiagnosticPlugin)
            .add_systems(Startup, load_and_set_egui_fonts)
            .add_systems(Update, toggle_debug);
    }
}

impl DebugPlugin {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

pub fn debug_enable(engine_state: Res<DebugState>) -> bool {
    engine_state.enabled
}
pub fn debug_enable_for_key(key: String) -> impl FnMut(Res<DebugState>) -> bool {
    move |state: Res<DebugState>| {
        if let Some(v) = state.keys.get(&key) {
            v.clone()
        } else {
            false
        }
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
        (TextStyle::Button, FontId::new(15.0, Proportional)),
        (TextStyle::Small, FontId::new(12.0, Proportional)),
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

pub trait Debugger {
    fn add_debug_system<M>(&mut self, systems: impl IntoSystemConfigs<M>, key: String) -> &mut Self;
}

impl Debugger for App {
    fn add_debug_system<M>(&mut self, systems: impl IntoSystemConfigs<M>, key: String) -> &mut Self {
        if let Some(mut res) = self.world_mut().get_resource_mut::<DebugState>() {
            res.keys.insert(key.clone(), false);
        }
        self.add_systems(Update, systems.run_if(debug_enable).run_if(debug_enable_for_key(key)));
        self
    }
}
