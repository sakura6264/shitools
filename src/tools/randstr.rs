use super::*;
use eframe::egui;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RandStr {
    charset: String,
    output: String,
    len: usize,
}

impl RandStr {
    pub fn new() -> Self {
        Self {
            charset: "abcdefghijklmnopqrstuvwxyz".to_string(),
            output: String::new(),
            len: 16,
        }
    }
}

impl ToolComponent for RandStr {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    self.output.clear();
                }
                if ui.button("Copy").clicked() {
                    ui.output_mut(|reader| {
                        reader.copied_text = self.output.clone();
                    });
                }
                if ui.button("Generate").clicked() && self.len > 0 && !self.charset.is_empty() {
                    let mut rng = ChaChaRng::from_seed(crate::get_seed());
                    let charset: Vec<char> = self.charset.chars().collect();
                    let mut output = String::new();
                    for _ in 0..self.len {
                        let index: usize = rng.gen_range(0..charset.len());
                        output.push(charset[index]);
                    }
                    self.output = output;
                }
            });
            ui.add(egui::TextEdit::multiline(&mut self.output).desired_width(f32::INFINITY));
            ui.horizontal(|ui| {
                ui.label("Length:");
                ui.add(
                    egui::DragValue::new(&mut self.len)
                        .speed(1.0)
                        .range(0..=512),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Charset:");
                if ui.button("Clear").clicked() {
                    self.charset.clear();
                }
                if ui.button("+Num").clicked() {
                    self.charset.push_str("0123456789");
                }
                if ui.button("+a-z").clicked() {
                    self.charset.push_str("abcdefghijklmnopqrstuvwxyz");
                }
                if ui.button("+A-Z").clicked() {
                    self.charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                }
                if ui.button("0-Z_").clicked() {
                    self.charset =
                        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"
                            .to_string();
                }
                if ui.button("ASCII").clicked() {
                    self.charset = (0x20..=0x7E).map(|x| x as u8 as char).collect::<String>();
                }
            });
            ui.add(egui::TextEdit::multiline(&mut self.charset).desired_width(f32::INFINITY));
        });
    }
}
