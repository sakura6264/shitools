use eframe::egui;
use std::sync::mpsc;
use std::thread;
use vtracer;

use super::*;

const LOAD_IMG: u8 = 0;
const SAVE_IMG: u8 = 1;

#[derive(PartialEq, Eq, Hash, Clone)]
enum FileOpType {
    Open,
    Save,
    None,
}

pub struct Vtracer {
    config: vtracer::Config,
    hthread: Option<thread::JoinHandle<()>>,
    msg: Option<Msg>,
    op_type: FileOpType,
    pathbuffer: Option<path::PathBuf>,
    recv: Option<mpsc::Receiver<Msg>>,
}

impl Vtracer {
    pub fn new() -> Self {
        Self {
            config: vtracer::Config::default(),
            hthread: None,
            msg: None,
            op_type: FileOpType::None,
            pathbuffer: None,
            recv: None,
        }
    }
}

impl ToolComponent for Vtracer {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.config.length_threshold < 3.0 {
            self.config.length_threshold = 3.0;
        }
        if self.config.length_threshold > 12.0 {
            self.config.length_threshold = 12.0;
        }
        if let Some(r) = &self.recv {
            if let Ok(msg) = r.try_recv() {
                self.msg = Some(msg);
                self.recv = None;
                self.hthread = None;
                self.pathbuffer = None;
            }
        }
        ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Convert").clicked() {
                        if self.hthread.is_none() {
                            self.op_type = FileOpType::Open;
                        }
                        else {
                            self.msg = Some(Msg::new("Running. Please Wait.".into(), MsgType::Warning));
                        }
                    }
                    if let Some(ht) = &self.hthread {
                        if ht.is_finished() {
                            self.hthread = None;
                        }
                        else {
                            ui.spinner();
                        }
                    }
                });
                ui.add_space(10.0);
                ui.horizontal(|ui|{
                    ui.label("ColorMode:");
                    if ui.button(match self.config.color_mode{
                        vtracer::ColorMode::Binary => "Binary",
                        vtracer::ColorMode::Color => "Color"
                    }).clicked() {
                        self.config.color_mode = match self.config.color_mode{
                            vtracer::ColorMode::Binary => vtracer::ColorMode::Color,
                            vtracer::ColorMode::Color => vtracer::ColorMode::Binary
                        }
                    }
                    ui.label("Hierarchical:");
                    if ui.button(match self.config.hierarchical {
                        vtracer::Hierarchical::Cutout => "Cutout",
                        vtracer::Hierarchical::Stacked => "Stacked"
                    }).clicked() {
                        self.config.hierarchical = match self.config.hierarchical{
                            vtracer::Hierarchical::Cutout => vtracer::Hierarchical::Stacked,
                            vtracer::Hierarchical::Stacked => vtracer::Hierarchical::Cutout
                        }
                    }
                    ui.label("Curve Fitting:");
                    if ui.button(match self.config.mode{
                        visioncortex::PathSimplifyMode::None => "None",
                        visioncortex::PathSimplifyMode::Polygon => "Polygon",
                        visioncortex::PathSimplifyMode::Spline => "Spline"
                    }).clicked() {
                        self.config.mode = match self.config.mode {
                            visioncortex::PathSimplifyMode::None => visioncortex::PathSimplifyMode::Polygon,
                        visioncortex::PathSimplifyMode::Polygon => visioncortex::PathSimplifyMode::Spline,
                        visioncortex::PathSimplifyMode::Spline => visioncortex::PathSimplifyMode::None
                        }
                    }
                });
                ui.add_space(10.0);
                ui.horizontal(|ui|{
                    ui.label("Filter Speckle:").on_hover_text("Discard patches small than X px in size");
                    ui.add(egui::widgets::DragValue::new(&mut self.config.filter_speckle).speed(1.0).range(0..=128));
                    ui.separator();
                    if ui.button(
                        if self.config.path_precision.is_none() {
                            "Do not Use Path Precision Parameter"
                        }
                        else{
                            "Path Precision:"
                        }
                    ).on_hover_text("Number of decimal places to use in path string").clicked() {
                        self.config.path_precision = match self.config.path_precision {
                            Some(_) => None,
                            None => Some(8)
                        }
                    }
                    if let Some(s) = &mut self.config.path_precision {
                        ui.add(egui::widgets::DragValue::new(s).speed(1.0).range(0..=32));
                    }
                });
                if let vtracer::ColorMode::Color = self.config.color_mode {
                    ui.add_space(10.0);
                    ui.horizontal(|ui|{
                        ui.label("Color Precision:").on_hover_text("Number of significant bits to use in a RGB channel");
                        ui.add(egui::widgets::DragValue::new(&mut self.config.color_precision).speed(1.0).range(1..=6));
                        ui.label("Gradient Step:").on_hover_text("Color difference between gradient layers");
                        ui.add(egui::widgets::DragValue::new(&mut self.config.max_iterations).speed(1.0).range(0..=16));
                    });
                }

                if let visioncortex::PathSimplifyMode::Spline = self.config.mode {
                    ui.add_space(10.0);
                    ui.horizontal(|ui|{
                        ui.label("Corner Threshold").on_hover_text("Minimum Momentary Angle (in degrees) to be considered a corner (to be kept after smoothing)");
                        ui.add(egui::widgets::DragValue::new(&mut self.config.corner_threshold).speed(1.0).range(0..=180));
                        ui.label("Segment Length").on_hover_text("Perform Iterative Subdivide Smooth until all segments are shorter than this length");
                        ui.add(egui::widgets::DragValue::new(&mut self.config.length_threshold).speed(1.0));
                        ui.label("Splice Threshold").on_hover_text("Minimum Angle Displacement (in degrees) to be considered a cutting point between curves");
                        ui.add(egui::widgets::DragValue::new(&mut self.config.splice_threshold).speed(1.0).range(0..=180));
                    });
                }
            });
    }
    fn get_msg(&mut self) -> Option<Msg> {
        let msgtmp = self.msg.clone();
        self.msg = None;
        msgtmp
    }
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        match self.op_type {
            FileOpType::None => None,
            FileOpType::Open => {
                self.op_type = FileOpType::None;
                Some((
                    FileOp {
                        title: "Open Image".into(),
                        mode: FileOpMode::Open,
                        filter: Vec::new(),
                    },
                    LOAD_IMG,
                ))
            }
            FileOpType::Save => {
                self.op_type = FileOpType::None;
                Some((
                    FileOp {
                        title: "Save SVG".into(),
                        filter: vec!["svg".into()],
                        mode: FileOpMode::Save,
                    },
                    SAVE_IMG,
                ))
            }
        }
    }
    fn set_file_op(&mut self, file_path: Option<(path::PathBuf, u8)>) {
        if let Some((path, op)) = file_path {
            match op {
                LOAD_IMG => {
                    self.pathbuffer = Some(path);
                    self.op_type = FileOpType::Save;
                }
                SAVE_IMG => {
                    if let Some(pb) = &self.pathbuffer {
                        let input = pb.clone();
                        let output = path.clone();
                        let cfg = vtracer::Config {
                            color_mode: match self.config.color_mode {
                                vtracer::ColorMode::Binary => vtracer::ColorMode::Binary,
                                vtracer::ColorMode::Color => vtracer::ColorMode::Color,
                            },
                            hierarchical: match self.config.hierarchical {
                                vtracer::Hierarchical::Cutout => vtracer::Hierarchical::Cutout,
                                vtracer::Hierarchical::Stacked => vtracer::Hierarchical::Stacked,
                            },
                            ..self.config
                        };
                        let (tx, rx) = mpsc::channel();
                        self.recv = Some(rx);
                        self.hthread = Some(thread::spawn(move || {
                            if let Err(e) = vtracer::convert_image_to_svg(&input, &output, cfg) {
                                tx.send(Msg::new(e.into(), MsgType::Error)).unwrap();
                            } else {
                                tx.send(Msg::new("Done".into(), MsgType::Info)).unwrap();
                            }
                        }));
                    }
                    self.pathbuffer = None;
                    self.op_type = FileOpType::None;
                }
                _ => {}
            }
        }
    }
}
