use super::*;
use eframe::egui;
use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Hash, Clone)]
enum FileState {
    None,
    LoadText,
    LoadAgainst,
    SaveText,
    SaveSplit,
}

const LOAD_TEXT: u8 = 0;
const LOAD_AGAINST: u8 = 1;
const SAVE_TEXT: u8 = 2;
const SAVE_SPLIT: u8 = 3;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct TextUtil {
    text_add: String,
    texts: BTreeSet<String>,
    against_add: String,
    againsts: BTreeSet<String>,
    split_size: usize,
    msg: Option<Msg>,
    state: FileState,
    current_page: usize,
    page_size: usize,
    against_page: usize,
    against_page_size: usize,
}

impl TextUtil {
    pub fn new() -> Self {
        Self {
            text_add: String::new(),
            texts: BTreeSet::new(),
            against_add: String::new(),
            againsts: BTreeSet::new(),
            split_size: 500,
            msg: None,
            state: FileState::None,
            current_page: 0,
            page_size: 50,
            against_page: 0,
            against_page_size: 50,
        }
    }
    fn texts_from_file(&mut self, file: &std::path::PathBuf) -> Result<(), String> {
        let file = crate::read_file(file)?;
        file.lines().for_each(|line| {
            self.texts.insert(line.to_string());
        });
        Ok(())
    }
    fn againsts_from_file(&mut self, file: &std::path::PathBuf) -> Result<(), String> {
        let file = crate::read_file(file)?;
        file.lines().for_each(|line| {
            self.againsts.insert(line.to_string());
        });
        Ok(())
    }
    fn diff(&mut self) {
        let mut diffstr = Vec::new();
        for text in &self.texts {
            if self.againsts.contains(text) {
                diffstr.push(text.clone());
            }
        }
        for remove_str in diffstr {
            self.texts.remove(&remove_str);
        }
    }
    fn save_to_file(&self, file: &std::path::PathBuf) -> Result<(), String> {
        let to_write = self
            .texts
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join("\n");
        crate::write_file(file, &to_write)?;
        Ok(())
    }
    fn save_splited(&self, dir: &std::path::PathBuf) -> Result<(), String> {
        let mut splited = Vec::new();
        let mut temp = Vec::new();
        for text in self.texts.iter() {
            temp.push(text);
            if temp.len() >= self.split_size {
                splited.push(temp);
                temp = Vec::new();
            }
        }
        if !temp.is_empty() {
            splited.push(temp);
        }
        for (i, texts) in splited.iter().enumerate() {
            let file = dir.join(format!("{}.txt", i));
            let to_write = texts
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join("\n");
            crate::write_file(&file, &to_write)?;
        }
        Ok(())
    }
}

impl ToolComponent for TextUtil {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Export").clicked() {
                self.state = FileState::SaveText;
            }
            if ui.button("Import").clicked() {
                self.state = FileState::LoadText;
            }
            if ui.button("Import Against").clicked() {
                self.state = FileState::LoadAgainst;
            }
            if ui.button("Diff").clicked() {
                self.diff();
            }
            if ui.button("Clear").clicked() {
                self.texts.clear();
            }
            if ui.button("Clear Against").clicked() {
                self.againsts.clear();
            }
            if ui.button("Swap").clicked() {
                std::mem::swap(&mut self.texts, &mut self.againsts);
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Export Split").clicked() {
                self.state = FileState::SaveSplit;
            }
            ui.label("Split Size :");
            ui.add(egui::DragValue::new(&mut self.split_size).speed(1.0));
        });
        let width = ui.available_width();
        ui.horizontal(|ui| {
            let mut cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_ui_at_rect(cursor, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Text");
                        if ui.button("Add").clicked() {
                            self.texts.insert(self.text_add.clone());
                            self.text_add.clear();
                        }
                        ui.add(
                            egui::TextEdit::singleline(&mut self.text_add)
                                .desired_width(f32::INFINITY),
                        );
                    });
                    ui.separator();
                    ui.label(format!("Total: {}", self.texts.len()));
                    ui.horizontal(|ui| {
                        ui.label("Page Size :");
                        ui.add(egui::DragValue::new(&mut self.page_size).speed(1.0));
                        if ui.button("<<").clicked() {
                            self.current_page = 0;
                        }
                        if ui.button("<").clicked() {
                            if self.current_page > 0 {
                                self.current_page -= 1;
                            }
                        }
                        ui.add(
                            egui::DragValue::new(&mut self.current_page)
                                .speed(1.0)
                                .clamp_range(0..=self.texts.len() / self.page_size),
                        );
                        ui.label(format!("/{}", self.texts.len() / self.page_size));
                        if ui.button(">").clicked() {
                            if self.current_page < self.texts.len() / self.page_size {
                                self.current_page += 1;
                            }
                        }
                        if ui.button(">>").clicked() {
                            self.current_page = self.texts.len() / self.page_size;
                        }
                    });
                    let mut to_remove = None;
                    let from_num = self.current_page * self.page_size;
                    let to_num = ((self.current_page + 1) * self.page_size).min(self.texts.len());
                    for text in self.texts.iter().skip(from_num).take(to_num - from_num) {
                        ui.horizontal(|ui| {
                            if ui.button("Remove").clicked() {
                                to_remove = Some(text.clone());
                            }
                            ui.label(text);
                        });
                        ui.separator();
                    }
                    if let Some(text) = to_remove {
                        self.texts.remove(&text);
                    }
                });
            });
            cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_ui_at_rect(cursor, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Against");
                        if ui.button("Add").clicked() {
                            self.againsts.insert(self.against_add.clone());
                            self.against_add.clear();
                        }
                        ui.add(
                            egui::TextEdit::singleline(&mut self.against_add)
                                .desired_width(f32::INFINITY),
                        );
                    });
                    ui.separator();
                    ui.label(format!("Total: {}", self.againsts.len()));
                    ui.horizontal(|ui| {
                        ui.label("Page Size :");
                        ui.add(egui::DragValue::new(&mut self.against_page_size).speed(1.0));
                        if ui.button("<<").clicked() {
                            self.against_page = 0;
                        }
                        if ui.button("<").clicked() {
                            if self.against_page > 0 {
                                self.against_page -= 1;
                            }
                        }
                        ui.add(
                            egui::DragValue::new(&mut self.against_page)
                                .speed(1.0)
                                .clamp_range(0..=self.againsts.len() / self.against_page_size),
                        );
                        ui.label(format!("/{}", self.againsts.len() / self.against_page_size));
                        if ui.button(">").clicked() {
                            if self.against_page < self.againsts.len() / self.against_page_size {
                                self.against_page += 1;
                            }
                        }
                        if ui.button(">>").clicked() {
                            self.against_page = self.againsts.len() / self.against_page_size;
                        }
                    });
                    let mut to_remove = None;
                    let from_num = self.against_page * self.against_page_size;
                    let to_num =
                        ((self.against_page + 1) * self.against_page_size).min(self.againsts.len());
                    for against in self.againsts.iter().skip(from_num).take(to_num - from_num) {
                        ui.horizontal(|ui| {
                            if ui.button("Remove").clicked() {
                                to_remove = Some(against.clone());
                            }
                            ui.label(against);
                        });
                        ui.separator();
                    }
                    if let Some(against) = to_remove {
                        self.againsts.remove(&against);
                    }
                });
            });
        });
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        match self.state {
            FileState::LoadText => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Load Text".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Open,
                    },
                    LOAD_TEXT,
                ))
            }
            FileState::LoadAgainst => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Load Against".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Open,
                    },
                    LOAD_AGAINST,
                ))
            }
            FileState::SaveText => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Save Text".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Save,
                    },
                    SAVE_TEXT,
                ))
            }
            FileState::SaveSplit => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Save Split".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Dir,
                    },
                    SAVE_SPLIT,
                ))
            }
            _ => None,
        }
    }
    fn set_file_op(&mut self, file_path: Option<(path::PathBuf, u8)>) {
        if let Some((ref file, mode)) = file_path {
            match mode {
                LOAD_TEXT => {
                    if let Err(e) = self.texts_from_file(file) {
                        self.msg = Some(Msg::new(e, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new("Load Success".to_string(), MsgType::Info));
                    }
                }
                LOAD_AGAINST => {
                    if let Err(e) = self.againsts_from_file(file) {
                        self.msg = Some(Msg::new(e, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new("Load Success".to_string(), MsgType::Info));
                    }
                }
                SAVE_TEXT => {
                    if let Err(e) = self.save_to_file(file) {
                        self.msg = Some(Msg::new(e, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new("Save Success".to_string(), MsgType::Info));
                    }
                }
                SAVE_SPLIT => {
                    if let Err(e) = self.save_splited(file) {
                        self.msg = Some(Msg::new(e, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new("Save Success".to_string(), MsgType::Info));
                    }
                }
                _ => {}
            }
        }
    }
}
