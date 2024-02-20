#![windows_subsystem = "windows"]
mod mainwindow;
mod tools;
mod utils;
use eframe::egui;
use mimalloc::MiMalloc;
use std::sync::Arc;
pub use utils::*;

include_flate::flate!(static FONTS: [u8] from "assets/JetBrainsMono-Regular.ttf");
include_flate::flate!(static FALLBACK: [u8] from "assets/DroidSansFallback.ttf");
include_flate::flate!(static ICON: [u8] from "assets/shitools.png");

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let icon_img = image::load_from_memory(&ICON).unwrap();
    let icon_buffer = icon_img.to_rgba8();
    let icon_pixels = icon_buffer.as_flat_samples();
    let icon_data = egui::IconData {
        rgba: icon_pixels.to_vec().samples,
        width: icon_img.width(),
        height: icon_img.height(),
    };
    let option = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            title: Some(format!(
                "ShiTools [{}]",
                static_dir_path().to_string_lossy()
            )),
            inner_size: Some(egui::vec2(900.0, 600.0)),
            icon: Some(Arc::new(icon_data)),
            ..Default::default()
        },
        default_theme: eframe::Theme::Dark,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native(
        "ShiTools",
        option,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts
                .font_data
                .insert("fonts".to_string(), egui::FontData::from_static(&FONTS));
            fonts.font_data.insert(
                "fallback".to_string(),
                egui::FontData::from_static(&FALLBACK),
            );
            let proportional = fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default();
            proportional.insert(0, "fonts".to_string());
            proportional.insert(1, "fallback".to_string());

            let monospace = fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default();
            monospace.insert(0, "fallback".to_string());
            monospace.push("fonts".to_string());
            cc.egui_ctx.set_fonts(fonts);
            Box::new(mainwindow::MainWindow::new())
        }),
    )
    .unwrap();
}
