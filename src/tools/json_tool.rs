use super::*;
use eframe::egui;

const IMPORT_TEXT: u8 = 0;
const EXPORT_TEXT: u8 = 1;
const EXPORT_FORMATTED: u8 = 2;
const EXPORT_PLAIN: u8 = 3;
const TO_YAML: u8 = 4;

#[derive(PartialEq, Eq, Hash, Clone)]
enum FileOpState {
    None,
    ImportText,
    ExportText,
    ExportFormatted,
    ExportPlain,
    ToYaml,
}

#[derive(PartialEq, Clone)]
pub struct JsonTool {
    json: String,
    display: Result<serde_json::Value, String>,
    op_state: FileOpState,
    msg: Option<Msg>,
}

impl JsonTool {
    pub fn new() -> Self {
        Self {
            json: String::new(),
            display: Err(String::new()),
            op_state: FileOpState::None,
            msg: None,
        }
    }
    fn sync(&mut self) {
        self.display = serde_json::from_str(&self.json).map_err(|e| e.to_string());
    }
}

impl ToolComponent for JsonTool {
    fn paint_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let width = ui.available_width();
        ui.horizontal(|ui| {
            let mut cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Text");
                        if ui.button("Clear").clicked() {
                            self.json.clear();
                            self.display = Err(String::new());
                        }
                        if ui.button("Copy").clicked() {
                            ui.output_mut(|out| {
                                out.copied_text = self.json.clone();
                            });
                        }
                        if ui.button("Import").clicked() {
                            self.op_state = FileOpState::ImportText;
                        }
                        if ui.button("Export").clicked() {
                            self.op_state = FileOpState::ExportText;
                        }
                        if ui.button("Format").clicked() {
                            if let Ok(v) = &self.display {
                                let result = serde_json::to_string_pretty(&v);
                                match result {
                                    Ok(s) => {
                                        self.json = s;
                                    }
                                    Err(e) => {
                                        self.display = Err(e.to_string());
                                    }
                                }
                            }
                        }
                    });
                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        use egui_extras::syntax_highlighting::*;
                        let mut layout_job = egui_extras::syntax_highlighting::highlight(
                            ctx,
                            ui.style(),
                            &CodeTheme::from_style(&ctx.style()),
                            string,
                            "json",
                        );
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };
                    if ui
                        .add(
                            egui::TextEdit::multiline(&mut self.json)
                                .desired_width(f32::INFINITY)
                                .desired_rows(20)
                                .code_editor()
                                .layouter(&mut layouter),
                        )
                        .changed()
                    {
                        self.sync();
                    }
                });
            });
            cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("JSON");
                        if ui.button("Export Formatted").clicked() && self.display.is_ok() {
                            self.op_state = FileOpState::ExportFormatted;
                        }
                        if ui.button("Export Plain").clicked() && self.display.is_ok() {
                            self.op_state = FileOpState::ExportPlain;
                        }
                        if ui.button("To Yaml").clicked() && self.display.is_ok() {
                            self.op_state = FileOpState::ToYaml;
                        }
                    });
                    match &self.display {
                        Ok(value) => {
                            egui_json_tree::JsonTree::new("tool/json_tool/tree", value).show(ui);
                        }
                        Err(err) => {
                            ui.label(err);
                        }
                    }
                });
            });
        });
    }
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        match self.op_state {
            FileOpState::ImportText => {
                self.op_state = FileOpState::None;
                Some((
                    FileOp {
                        mode: FileOpMode::Open,
                        title: "Import JSON".to_string(),
                        filter: Vec::new(),
                    },
                    IMPORT_TEXT,
                ))
            }
            FileOpState::ExportText => {
                self.op_state = FileOpState::None;
                Some((
                    FileOp {
                        mode: FileOpMode::Save,
                        title: "Export JSON".to_string(),
                        filter: Vec::new(),
                    },
                    EXPORT_TEXT,
                ))
            }
            FileOpState::ExportFormatted => {
                self.op_state = FileOpState::None;
                Some((
                    FileOp {
                        mode: FileOpMode::Save,
                        title: "Export Formatted JSON".to_string(),
                        filter: vec!["json".to_string()],
                    },
                    EXPORT_FORMATTED,
                ))
            }
            FileOpState::ExportPlain => {
                self.op_state = FileOpState::None;
                Some((
                    FileOp {
                        mode: FileOpMode::Save,
                        title: "Export Plain JSON".to_string(),
                        filter: vec!["json".to_string()],
                    },
                    EXPORT_PLAIN,
                ))
            }
            FileOpState::ToYaml => {
                self.op_state = FileOpState::None;
                Some((
                    FileOp {
                        mode: FileOpMode::Save,
                        title: "To YAML".to_string(),
                        filter: vec!["yaml".to_string(), "yml".to_string()],
                    },
                    TO_YAML,
                ))
            }
            _ => None,
        }
    }
    fn set_file_op(&mut self, _file_path: Option<(path::PathBuf, u8)>) {
        if let Some((file_path, mode)) = _file_path {
            match mode {
                IMPORT_TEXT => match crate::read_file(&file_path) {
                    Ok(s) => {
                        self.json = s;
                        self.sync();
                    }
                    Err(e) => {
                        self.msg = Some(Msg {
                            text: e,
                            msg_type: MsgType::Error,
                        });
                    }
                },
                EXPORT_TEXT => match crate::write_file(&file_path, &self.json) {
                    Ok(_) => {
                        self.msg = Some(Msg {
                            text: "Exported".to_string(),
                            msg_type: MsgType::Info,
                        });
                    }
                    Err(e) => {
                        self.msg = Some(Msg {
                            text: e,
                            msg_type: MsgType::Error,
                        });
                    }
                },
                EXPORT_FORMATTED => {
                    if let Ok(v) = &self.display {
                        let result = serde_json::to_string_pretty(&v);
                        match result {
                            Ok(s) => match crate::write_file(&file_path, &s) {
                                Ok(_) => {
                                    self.msg = Some(Msg {
                                        text: "Exported".to_string(),
                                        msg_type: MsgType::Info,
                                    });
                                }
                                Err(e) => {
                                    self.msg = Some(Msg {
                                        text: e,
                                        msg_type: MsgType::Error,
                                    });
                                }
                            },
                            Err(e) => {
                                self.msg = Some(Msg {
                                    text: e.to_string(),
                                    msg_type: MsgType::Error,
                                });
                            }
                        }
                    }
                }
                EXPORT_PLAIN => {
                    if let Ok(v) = &self.display {
                        let result = serde_json::to_string(&v);
                        match result {
                            Ok(s) => match crate::write_file(&file_path, &s) {
                                Ok(_) => {
                                    self.msg = Some(Msg {
                                        text: "Exported".to_string(),
                                        msg_type: MsgType::Info,
                                    });
                                }
                                Err(e) => {
                                    self.msg = Some(Msg {
                                        text: e,
                                        msg_type: MsgType::Error,
                                    });
                                }
                            },
                            Err(e) => {
                                self.msg = Some(Msg {
                                    text: e.to_string(),
                                    msg_type: MsgType::Error,
                                });
                            }
                        }
                    }
                }
                TO_YAML => {
                    if let Ok(v) = &self.display {
                        let result = serde_yaml::to_string(&v);
                        match result {
                            Ok(s) => match crate::write_file(&file_path, &s) {
                                Ok(_) => {
                                    self.msg = Some(Msg {
                                        text: "Exported".to_string(),
                                        msg_type: MsgType::Info,
                                    });
                                }
                                Err(e) => {
                                    self.msg = Some(Msg {
                                        text: e,
                                        msg_type: MsgType::Error,
                                    });
                                }
                            },
                            Err(e) => {
                                self.msg = Some(Msg {
                                    text: e.to_string(),
                                    msg_type: MsgType::Error,
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
}
