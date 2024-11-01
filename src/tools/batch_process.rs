use super::*;
use eframe::egui;

#[derive(PartialEq, Eq, Hash, Clone)]
enum FileState {
    None,
    LoadSource,
    LoadDestination,
    ExportSource,
    ExportDestination,
    ScanDir,
}
const LOAD_SOURCE: u8 = 0;
const LOAD_DESTINATION: u8 = 1;
const EXPORT_SOURCE: u8 = 2;
const EXPORT_DESTINATION: u8 = 3;
const SCAN_DIR: u8 = 4;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct BatchProcess {
    from_list: Vec<String>,
    to_list: Vec<String>,
    skip_errors: bool,
    recusive: bool,
    msg: Option<Msg>,
    state: FileState,
    from_page: usize,
    to_page: usize,
    from_page_size: usize,
    to_page_size: usize,
}

impl BatchProcess {
    pub fn new() -> Self {
        Self {
            from_list: Vec::new(),
            to_list: Vec::new(),
            skip_errors: false,
            recusive: false,
            msg: None,
            state: FileState::None,
            from_page: 0,
            to_page: 0,
            from_page_size: 50,
            to_page_size: 50,
        }
    }
    fn load_from(&mut self, file: &std::path::PathBuf) -> Result<(), String> {
        let file = crate::read_file(file)?;
        file.lines().for_each(|line| {
            self.from_list.push(line.to_string());
        });
        Ok(())
    }
    fn load_to(&mut self, file: &std::path::PathBuf) -> Result<(), String> {
        let file = crate::read_file(file)?;
        file.lines().for_each(|line| {
            self.to_list.push(line.to_string());
        });
        Ok(())
    }
    fn export_from(&mut self, file: &std::path::PathBuf) -> Result<(), String> {
        let string_to_write = self.from_list.join("\n");
        crate::write_file(file, &string_to_write)?;
        Ok(())
    }
    fn export_to(&mut self, file: &std::path::PathBuf) -> Result<(), String> {
        let string_to_write = self.to_list.join("\n");
        crate::write_file(file, &string_to_write)?;
        Ok(())
    }
    fn selfcheck(&mut self) {
        match self.from_list.len().cmp(&self.to_list.len()) {
            std::cmp::Ordering::Less => {
                self.from_list.resize(self.to_list.len(), String::new());
            }
            std::cmp::Ordering::Greater => {
                self.to_list.resize(self.from_list.len(), String::new());
            }
            std::cmp::Ordering::Equal => {}
        }
    }
    fn perform_move(&mut self) -> Result<(), String> {
        self.selfcheck();
        let mut errors = Vec::new();
        let mut done_from = std::collections::HashSet::new();
        let mut done_to = std::collections::HashSet::new();
        for (from, to) in self.from_list.iter().zip(self.to_list.iter()) {
            let from_pb = std::path::PathBuf::from(from);
            let to_pb = std::path::PathBuf::from(to);
            if let Err(e) = std::fs::rename(from_pb, to_pb).map_err(|e| e.to_string()) {
                errors.push(e);
                if self.skip_errors {
                    continue;
                } else {
                    break;
                }
            }
            done_from.insert(from.clone());
            done_to.insert(to.clone());
        }
        if done_from.len() == self.from_list.len() && done_to.len() == self.to_list.len() {
            self.from_list.clear();
            self.to_list.clear();
        } else {
            self.from_list.retain(|x| !done_from.contains(x));
            self.to_list.retain(|x| !done_to.contains(x));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }
    fn perform_copy(&mut self) -> Result<(), String> {
        self.selfcheck();
        let mut errors = Vec::new();
        let mut done_from = std::collections::HashSet::new();
        let mut done_to = std::collections::HashSet::new();
        for (from, to) in self.from_list.iter().zip(self.to_list.iter()) {
            let from_pb = std::path::PathBuf::from(from);
            let to_pb = std::path::PathBuf::from(to);
            if let Err(e) = std::fs::copy(from_pb, to_pb).map_err(|e| e.to_string()) {
                errors.push(e);
                if self.skip_errors {
                    continue;
                } else {
                    break;
                }
            }
            done_from.insert(from.clone());
            done_to.insert(to.clone());
        }
        if done_from.len() == self.from_list.len() && done_to.len() == self.to_list.len() {
            self.from_list.clear();
            self.to_list.clear();
        } else {
            self.from_list.retain(|x| !done_from.contains(x));
            self.to_list.retain(|x| !done_to.contains(x));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }
    fn delete_source(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        let mut done_from = std::collections::HashSet::new();
        for from in self.from_list.iter() {
            let from_pb = std::path::PathBuf::from(from);
            if let Err(e) = std::fs::remove_file(from_pb).map_err(|e| e.to_string()) {
                errors.push(e);
                if self.skip_errors {
                    continue;
                } else {
                    break;
                }
            }
            done_from.insert(from.clone());
        }
        if done_from.len() == self.from_list.len() {
            self.from_list.clear();
        } else {
            self.from_list.retain(|x| !done_from.contains(x));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }
    fn delete_destination(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        let mut done_to = std::collections::HashSet::new();
        for to in self.to_list.iter() {
            let to_pb = std::path::PathBuf::from(to);
            if let Err(e) = std::fs::remove_file(to_pb).map_err(|e| e.to_string()) {
                errors.push(e);
                if self.skip_errors {
                    continue;
                } else {
                    break;
                }
            }
            done_to.insert(to.clone());
        }
        if done_to.len() == self.to_list.len() {
            self.to_list.clear();
        } else {
            self.to_list.retain(|x| !done_to.contains(x));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }
    fn scan_dir(&mut self, dir: &std::path::PathBuf) -> Result<(), String> {
        let mut files = Vec::new();
        if self.recusive {
            for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.is_dir() {
                    self.scan_dir(&path)?;
                } else if path.is_file() {
                    files.push(path);
                }
            }
        } else {
            for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.is_file() {
                    files.push(path);
                }
            }
        }
        for file in files {
            self.from_list.push(file.to_string_lossy().to_string());
        }
        Ok(())
    }
}

impl ToolComponent for BatchProcess {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Load Source").clicked() {
                self.state = FileState::LoadSource;
            }
            if ui.button("Load Destination").clicked() {
                self.state = FileState::LoadDestination;
            }
            if ui.button("Clear Source").clicked() {
                self.from_list.clear();
            }
            if ui.button("Clear Destination").clicked() {
                self.to_list.clear();
            }
            if ui.button("Swap").clicked() {
                std::mem::swap(&mut self.from_list, &mut self.to_list);
            }
            if ui.button("Check").clicked() {
                self.selfcheck();
            }
            ui.checkbox(&mut self.skip_errors, "Skip Errors");
        });
        ui.horizontal(|ui| {
            if ui.button("Move").clicked() {
                if let Err(err) = self.perform_move() {
                    self.msg = Some(Msg::new(err, MsgType::Error));
                }
            }
            if ui.button("Copy").clicked() {
                if let Err(err) = self.perform_copy() {
                    self.msg = Some(Msg::new(err, MsgType::Error));
                }
            }
            if ui.button("Delete Source").clicked() {
                if let Err(err) = self.delete_source() {
                    self.msg = Some(Msg::new(err, MsgType::Error));
                }
            }
            if ui.button("Delete Destination").clicked() {
                if let Err(err) = self.delete_destination() {
                    self.msg = Some(Msg::new(err, MsgType::Error));
                }
            }
            if ui.button("Export Source").clicked() {
                self.state = FileState::ExportSource;
            }
            if ui.button("Export Destination").clicked() {
                self.state = FileState::ExportDestination;
            }
            if ui.button("Scan Dir").clicked() {
                self.state = FileState::ScanDir;
            }
            ui.checkbox(&mut self.recusive, "Recusive");
        });
        ui.separator();
        let width = ui.available_width();
        const HEIGHT: f32 = 20.0;
        let max_size = egui::vec2(width / 2.0 - 30.0, HEIGHT);
        ui.horizontal(|ui| {
            let mut cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.add_sized(max_size, egui::Label::new("Source"));
                    ui.horizontal(|ui| {
                        ui.label("Page Size :");
                        ui.add(egui::DragValue::new(&mut self.from_page_size).speed(1.0));
                        if ui.button("<<").clicked() {
                            self.from_page = 0;
                        }
                        if ui.button("<").clicked() {
                            if self.from_page > 0 {
                                self.from_page -= 1;
                            }
                        }
                        ui.add(
                            egui::DragValue::new(&mut self.from_page)
                                .speed(1.0)
                                .range(0..=self.from_list.len() / self.from_page_size),
                        );
                        ui.label(format!("/{}", self.from_list.len() / self.from_page_size));
                        if ui.button(">").clicked() {
                            if self.from_page < self.from_list.len() / self.from_page_size {
                                self.from_page += 1;
                            }
                        }
                        if ui.button(">>").clicked() {
                            self.from_page = self.from_list.len() / self.from_page_size;
                        }
                    });
                    let from_num = self.from_page * self.from_page_size;
                    let to_num =
                        ((self.from_page + 1) * self.from_page_size).min(self.from_list.len());
                    for i in from_num..to_num {
                        ui.horizontal(|ui| {
                            ui.label(i.to_string());
                            ui.separator();
                            ui.label(&self.from_list[i]);
                        });
                    }
                });
            });
            ui.separator();
            cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.add_sized(max_size, egui::Label::new("Destination"));
                    ui.horizontal(|ui| {
                        ui.label("Page Size :");
                        ui.add(egui::DragValue::new(&mut self.to_page_size).speed(1.0));
                        if ui.button("<<").clicked() {
                            self.to_page = 0;
                        }
                        if ui.button("<").clicked() {
                            if self.to_page > 0 {
                                self.to_page -= 1;
                            }
                        }
                        ui.add(
                            egui::DragValue::new(&mut self.to_page)
                                .speed(1.0)
                                .range(0..=self.to_list.len() / self.to_page_size),
                        );
                        ui.label(format!("/{}", self.to_list.len() / self.to_page_size));
                        if ui.button(">").clicked() {
                            if self.to_page < self.to_list.len() / self.to_page_size {
                                self.to_page += 1;
                            }
                        }
                        if ui.button(">>").clicked() {
                            self.to_page = self.to_list.len() / self.to_page_size;
                        }
                    });
                    let from_num = self.to_page * self.to_page_size;
                    let to_num = ((self.to_page + 1) * self.to_page_size).min(self.to_list.len());
                    for i in from_num..to_num {
                        ui.horizontal(|ui| {
                            ui.label(i.to_string());
                            ui.separator();
                            ui.label(&self.to_list[i]);
                        });
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
            FileState::LoadSource => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Load Source".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Open,
                    },
                    LOAD_SOURCE,
                ))
            }
            FileState::LoadDestination => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Load Destination".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Open,
                    },
                    LOAD_DESTINATION,
                ))
            }
            FileState::ExportSource => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Export Source".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Save,
                    },
                    EXPORT_SOURCE,
                ))
            }
            FileState::ExportDestination => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Export Destination".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Save,
                    },
                    EXPORT_DESTINATION,
                ))
            }
            FileState::ScanDir => {
                self.state = FileState::None;
                Some((
                    FileOp {
                        title: "Scan Dir".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Dir,
                    },
                    SCAN_DIR,
                ))
            }
            _ => None,
        }
    }
    fn set_file_op(&mut self, file_path: Option<(path::PathBuf, u8)>) {
        if let Some((path, mode)) = file_path {
            match mode {
                LOAD_SOURCE => {
                    if let Err(err) = self.load_from(&path) {
                        self.msg = Some(Msg::new(err, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new("Load Source Success".to_string(), MsgType::Info));
                    }
                }
                LOAD_DESTINATION => {
                    if let Err(err) = self.load_to(&path) {
                        self.msg = Some(Msg::new(err, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new(
                            "Load Destination Success".to_string(),
                            MsgType::Info,
                        ));
                    }
                }
                EXPORT_SOURCE => {
                    if let Err(err) = self.export_from(&path) {
                        self.msg = Some(Msg::new(err, MsgType::Error));
                    } else {
                        self.msg =
                            Some(Msg::new("Export Source Success".to_string(), MsgType::Info));
                    }
                }
                EXPORT_DESTINATION => {
                    if let Err(err) = self.export_to(&path) {
                        self.msg = Some(Msg::new(err, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new(
                            "Export Destination Success".to_string(),
                            MsgType::Info,
                        ));
                    }
                }
                SCAN_DIR => {
                    if let Err(err) = self.scan_dir(&path) {
                        self.msg = Some(Msg::new(err, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new("Scan Dir Success".to_string(), MsgType::Info));
                    }
                }
                _ => {}
            }
        }
    }
}
