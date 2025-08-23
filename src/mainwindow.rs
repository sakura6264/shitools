use crate::{dir_remember, tools};
use eframe::egui;

const MARGIN: f32 = 40f32;

pub struct MainWindow {
    tool: Box<dyn tools::ToolComponent>,
    toasts: egui_toast::Toasts,
    file_dialog: Option<egui_file::FileDialog>,
    current_dialog_id: u8,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            tool: Box::new(tools::Blank),
            toasts: egui_toast::Toasts::new()
                .anchor(egui::Align2::LEFT_BOTTOM, (MARGIN, -MARGIN))
                .direction(egui::Direction::BottomUp),
            file_dialog: None,
            current_dialog_id: 0,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let height = ctx.available_rect().height();
        let width = ctx.available_rect().width();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                for (name, components) in tools::static_namelist().iter() {
                    egui::menu::menu_button(ui, name, |ui| {
                        for (name, component) in components.iter() {
                            if ui.button(name).clicked() {
                                self.tool.on_close();
                                self.tool = tools::get_component(component.clone());
                                self.file_dialog = None;
                                self.current_dialog_id = 0;
                            }
                        }
                    });
                }
            });
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                self.tool.paint_ui(ui, ctx);
            });
        });
        if let Some((file_op, id)) = self.tool.get_file_op() {
            // build title use title and filter
            let mut title = file_op.title.clone();
            if !file_op.filter.is_empty() {
                title.push_str(" (");
                title.push_str(file_op.filter.join(", ").as_str());
                title.push(')');
            } else {
                title.push_str(" (*)");
            }
            let mut dlg = match file_op.mode {
                tools::FileOpMode::Open => {
                    egui_file::FileDialog::open_file(dir_remember::get_dir())
                        .title(title.as_str())
                        .default_size(egui::vec2(width / 2f32, height - 2f32 * MARGIN))
                        .current_pos(egui::pos2(width / 4f32, MARGIN))
                        .filename_filter(filter_build(file_op.filter))
                }
                tools::FileOpMode::Save => {
                    egui_file::FileDialog::save_file(dir_remember::get_dir())
                        .title(title.as_str())
                        .default_size(egui::vec2(width / 2f32, height - 2f32 * MARGIN))
                        .current_pos(egui::pos2(width / 4f32, MARGIN))
                        .filename_filter(filter_build(file_op.filter))
                }
                tools::FileOpMode::Dir => {
                    egui_file::FileDialog::select_folder(dir_remember::get_dir())
                        .title(title.as_str())
                        .default_size(egui::vec2(width / 2f32, height - 2f32 * MARGIN))
                        .current_pos(egui::pos2(width / 4f32, MARGIN))
                }
            };
            dlg.open();
            self.file_dialog = Some(dlg);
            self.current_dialog_id = id;
        }
        self.toasts.show(ctx);
        if let Some(ref mut dlg) = self.file_dialog {
            if dlg.show(ctx).selected() {
                if let Some(path) = dlg.path() {
                    self.tool
                        .set_file_op(Some((path.into(), self.current_dialog_id)));

                    // Save the directory path and handle any errors
                    if let Err(err) = dir_remember::set_dir(dlg.directory()) {
                        self.toasts.add(egui_toast::Toast {
                            kind: egui_toast::ToastKind::Warning,
                            text: format!("Failed to save directory: {}", err).into(),
                            options: egui_toast::ToastOptions::default()
                                .duration_in_seconds(2f64)
                                .show_progress(true),
                            style: egui_toast::ToastStyle::default(),
                        });
                    }

                    self.file_dialog = None;
                    self.current_dialog_id = 0;
                }
            }
        }
        if let Some(msg) = self.tool.get_msg() {
            match msg.msg_type {
                tools::MsgType::Error => {
                    self.toasts.add(egui_toast::Toast {
                        kind: egui_toast::ToastKind::Error,
                        text: msg.text.into(),
                        options: egui_toast::ToastOptions::default()
                            .duration_in_seconds(3f64)
                            .show_progress(true),
                        style: egui_toast::ToastStyle::default(),
                    });
                }
                tools::MsgType::Warning => {
                    self.toasts.add(egui_toast::Toast {
                        kind: egui_toast::ToastKind::Warning,
                        text: msg.text.into(),
                        options: egui_toast::ToastOptions::default()
                            .duration_in_seconds(2f64)
                            .show_progress(true),
                        style: egui_toast::ToastStyle::default(),
                    });
                }
                tools::MsgType::Info => {
                    self.toasts.add(egui_toast::Toast {
                        kind: egui_toast::ToastKind::Info,
                        text: msg.text.into(),
                        options: egui_toast::ToastOptions::default()
                            .duration_in_seconds(1f64)
                            .show_progress(true),
                        style: egui_toast::ToastStyle::default(),
                    });
                }
            }
        }
    }
}

fn filter_build(filter: Vec<String>) -> Box<dyn Fn(&str) -> bool + Send + Sync> {
    // Pre-normalize filters to lowercase to avoid per-check allocations
    let lowered: Vec<String> = filter.into_iter().map(|s| s.to_ascii_lowercase()).collect();
    Box::new(move |name| {
        if lowered.is_empty() {
            return true;
        }
        let name_lower = name.to_ascii_lowercase();
        lowered.iter().any(|f| name_lower.ends_with(f))
    })
}
