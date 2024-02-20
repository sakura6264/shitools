use super::*;
use eframe::egui;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Blank;

impl ToolComponent for Blank {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.heading("Hello World!");
    }
}
