mod polyglot_png_algo;

use super::*;
use eframe::egui;

#[derive(Clone, Hash, PartialEq, Eq)]
enum FileState {
    None,
    OpenPng,
    OpenContent,
    SaveResult,
}

const OPEN_PNG: u8 = 0;
const OPEN_CONTENT: u8 = 1;
const SAVE_RESULT: u8 = 2;

pub struct PolyglotPng {
    png_path: String,
    content_path: String,
    result_path: String,
    force_fix_zip: Option<bool>,
    state: FileState,
    msg: Option<Msg>,
    hthread: Option<std::thread::JoinHandle<()>>,
    recv: Option<std::sync::mpsc::Receiver<Option<String>>>,
}

impl PolyglotPng {
    pub fn new() -> Self {
        Self {
            png_path: String::new(),
            content_path: String::new(),
            result_path: String::new(),
            force_fix_zip: None,
            state: FileState::None,
            msg: None,
            hthread: None,
            recv: None,
        }
    }
    fn prepare_byte(path: &str) -> Result<Vec<u8>, String> {
        let path = std::path::PathBuf::from(path);
        if !path.exists() {
            return Err(format!("{} does not exist", path.to_string_lossy()));
        }
        if !path.is_file() {
            return Err(format!("{} is not a file", path.to_string_lossy()));
        }
        std::fs::read(path).map_err(|e| e.to_string())
    }
    fn prepare_inputs(&self) -> Result<(Vec<u8>, Vec<u8>), String> {
        let png = Self::prepare_byte(&self.png_path)?;
        let content = Self::prepare_byte(&self.content_path)?;
        Ok((png, content))
    }
    fn prepare_fix_zip(&self) -> bool {
        match self.force_fix_zip {
            Some(true) => true,
            Some(false) => false,
            None => {
                const EXTLIST: [&str; 7] =
                    [".zip", ".jar", ".apk", ".docx", ".xlsx", ".pptx", ".epub"];
                EXTLIST
                    .iter()
                    .any(|ext| self.result_path.to_lowercase().ends_with(ext))
            }
        }
    }
}

impl ToolComponent for PolyglotPng {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if let Some(recv) = &self.recv {
            if let Ok(result) = recv.try_recv() {
                match result {
                    Some(err) => self.msg = Some(Msg::new(err, MsgType::Error)),
                    None => self.msg = Some(Msg::new("Success".to_string(), MsgType::Info)),
                }
                self.hthread = None;
                self.recv = None;
            }
        }
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label("PNG Path:");
                if ui.button("...").clicked() {
                    self.state = FileState::OpenPng;
                }
                ui.text_edit_singleline(&mut self.png_path);
            });
            ui.horizontal(|ui| {
                ui.label("Content Path:");
                if ui.button("...").clicked() {
                    self.state = FileState::OpenContent;
                }
                ui.text_edit_singleline(&mut self.content_path);
            });
            ui.horizontal(|ui| {
                ui.label("Output Path:");
                if ui.button("...").clicked() {
                    self.state = FileState::SaveResult;
                }
                ui.text_edit_singleline(&mut self.result_path);
            });
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.force_fix_zip, None, "Auto Fix Zip");
                ui.radio_value(&mut self.force_fix_zip, Some(true), "Force Fix Zip");
                ui.radio_value(&mut self.force_fix_zip, Some(false), "No Fix Zip");
                if ui.button("Run").clicked() {
                    if self.png_path.is_empty() {
                        self.msg = Some(Msg::new("PNG Path is empty".to_string(), MsgType::Error));
                    } else if self.content_path.is_empty() {
                        self.msg = Some(Msg::new(
                            "Content Path is empty".to_string(),
                            MsgType::Error,
                        ));
                    } else if self.result_path.is_empty() {
                        self.msg =
                            Some(Msg::new("Output Path is empty".to_string(), MsgType::Error));
                    } else {
                        match self.prepare_inputs() {
                            Err(e) => self.msg = Some(Msg::new(e, MsgType::Error)),
                            Ok((png, content)) => {
                                let fix_zip = self.prepare_fix_zip();
                                let output_path = self.result_path.clone();
                                let (send, recv) = std::sync::mpsc::channel();
                                self.recv = Some(recv);
                                self.hthread = Some(std::thread::spawn(move || {
                                    let result =
                                        polyglot_png_algo::polyglot(&png, &content, fix_zip);
                                    let error = match result {
                                        Ok(data) => {
                                            if let Err(e) = std::fs::write(output_path, data) {
                                                Some(e.to_string())
                                            } else {
                                                None
                                            }
                                        }
                                        Err(e) => Some(e.to_string()),
                                    };
                                    send.send(error).unwrap();
                                }));
                            }
                        }
                    }
                }
                if self.hthread.is_some() {
                    ui.spinner();
                }
            });
        });
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        match self.state {
            FileState::OpenPng => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Open PNG".to_string(),
                        filter: vec!["png".to_string()],
                        mode: FileOpMode::Open,
                    },
                    OPEN_PNG,
                ))
            }
            FileState::OpenContent => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Open Content".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Open,
                    },
                    OPEN_CONTENT,
                ))
            }
            FileState::SaveResult => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Save Result".to_string(),
                        filter: vec!["png".to_string()],
                        mode: FileOpMode::Save,
                    },
                    SAVE_RESULT,
                ))
            }
            _ => None,
        }
    }
    fn set_file_op(&mut self, _file_path: Option<(path::PathBuf, u8)>) {
        if let Some((path, mode)) = _file_path {
            match mode {
                OPEN_PNG => self.png_path = path.to_string_lossy().to_string(),
                OPEN_CONTENT => self.content_path = path.to_string_lossy().to_string(),
                SAVE_RESULT => self.result_path = path.to_string_lossy().to_string(),
                _ => {}
            }
        }
    }
}
