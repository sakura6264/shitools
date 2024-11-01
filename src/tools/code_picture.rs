use super::*;
use eframe::egui;
use silicon::assets::HighlightingAssets;
use silicon::formatter::ImageFormatterBuilder;
use silicon::utils::ShadowAdder;
use std::sync::mpsc;
use std::thread;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

const DEFAULT_SHADOW_COLOR: &[u8; 4] = b"\x70\x70\x70\xff"; // #707070
const DEFAULT_SHADOW_BACKGROUND_SOLID: &[u8; 4] = b"\xab\xb8\xc3\xff"; // #abb8c3

#[derive(Clone, PartialEq, Hash, Eq)]
enum ShadowBackground {
    Solid,
    Image,
}

#[derive(Clone, PartialEq, Hash, Eq)]
enum FileState {
    None,
    OpenBackgroud,
    SaveImage,
}

const OPEN_BACKGROUND: u8 = 0;
const SAVE_IMAGE: u8 = 1;

#[derive(Clone, PartialEq)]
struct GenArgs {
    pub code: String,
    pub font_size: f32,
    pub theme: String,
    pub theme_search: String,
    pub syntax: String,
    pub syntax_search: String,
    pub shadow_blur_radius: f32,
    pub shadow_pad_horizontal: u32,
    pub shadow_pad_vertical: u32,
    pub shadow_offset_x: i32,
    pub shadow_offset_y: i32,
    pub shadow_color: [u8; 4],
    pub shadow_background: ShadowBackground,
    pub shadow_background_solid: [u8; 4],
    pub shadow_background_image: image::RgbaImage,
    pub shadow_background_image_info: String,
}

pub struct CodePicture {
    code: String,
    font_size: f32,
    highlightassets: HighlightingAssets,
    theme: String,
    theme_search: String,

    syntax: String,
    syntax_search: String,
    shadow_blur_radius: f32,
    shadow_pad_horizontal: u32,
    shadow_pad_vertical: u32,
    shadow_offset_x: i32,
    shadow_offset_y: i32,
    shadow_color: [u8; 4],
    shadow_background: ShadowBackground,
    shadow_background_solid: [u8; 4],
    shadow_background_image: image::RgbaImage,
    shadow_background_image_info: String,
    msg: Option<Msg>,
    file_state: FileState,
    hthread: Option<thread::JoinHandle<()>>,
    recv: Option<mpsc::Receiver<Msg>>,
}

impl CodePicture {
    pub fn new() -> Result<Self, String> {
        let ha = HighlightingAssets::new();
        let syntax = ha
            .syntax_set
            .find_syntax_by_token("rs")
            .ok_or("Syntax not found")?;
        let syn_name = syntax.name.clone();
        let theme_name = "Dracula".to_string();
        let sbimage = image::RgbaImage::from_fn(3, 3, |_, _| {
            image::Rgba(DEFAULT_SHADOW_BACKGROUND_SOLID.clone())
        });
        let sbimage_info = format!("Original:3x3 #abb8c3");
        Ok(Self {
            code: String::new(),
            font_size: 26.0,
            highlightassets: ha,
            theme: theme_name,
            theme_search: String::new(),
            syntax: syn_name,
            syntax_search: String::new(),
            shadow_blur_radius: 50.0,
            shadow_pad_horizontal: 80,
            shadow_pad_vertical: 100,
            shadow_offset_x: 0,
            shadow_offset_y: 0,
            shadow_color: DEFAULT_SHADOW_COLOR.clone(),
            shadow_background: ShadowBackground::Solid,
            shadow_background_solid: DEFAULT_SHADOW_BACKGROUND_SOLID.clone(),
            shadow_background_image: sbimage,
            shadow_background_image_info: sbimage_info,
            msg: None,
            file_state: FileState::None,
            hthread: None,
            recv: None,
        })
    }
    fn image_load(&mut self, path: &std::path::PathBuf) -> Result<(), String> {
        let img = image::open(path).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        self.shadow_background_image_info = format!(
            "{}:{}x{} {}",
            path.file_name().unwrap_or_default().to_string_lossy(),
            rgba.width(),
            rgba.height(),
            crate::format_mem(rgba.len())
        );
        self.shadow_background_image = rgba;
        Ok(())
    }
    fn generate_code_picture(args: &GenArgs, path: &std::path::PathBuf) -> Result<(), String> {
        let highlightassets = HighlightingAssets::new();
        let (ps, ts) = (highlightassets.syntax_set, highlightassets.theme_set);
        let syntax = ps
            .find_syntax_by_name(&args.syntax)
            .ok_or("Syntax not found")?;
        let theme = &ts.themes[&args.theme];
        let mut h = HighlightLines::new(syntax, theme);
        let highlight = LinesWithEndings::from(&args.code)
            .map(|line| h.highlight_line(line, &ps))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        let shadowadder = ShadowAdder::new()
            .background(match args.shadow_background {
                ShadowBackground::Solid => {
                    silicon::utils::Background::Solid(image::Rgba(args.shadow_background_solid))
                }
                ShadowBackground::Image => {
                    silicon::utils::Background::Image(args.shadow_background_image.clone())
                }
            })
            .blur_radius(args.shadow_blur_radius)
            .offset_x(args.shadow_offset_x)
            .offset_y(args.shadow_offset_y)
            .pad_horiz(args.shadow_pad_horizontal)
            .pad_vert(args.shadow_pad_vertical)
            .shadow_color(image::Rgba(args.shadow_color));
        let mut formatter = ImageFormatterBuilder::new()
            .font(vec![("Hack", args.font_size)])
            .shadow_adder(shadowadder)
            .build()
            .map_err(|e| e.to_string())?;
        let image = formatter.format(&highlight, theme);
        image.save(path).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl ToolComponent for CodePicture {
    fn paint_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(rx) = &self.recv {
            if let Ok(msg) = rx.try_recv() {
                self.msg = Some(msg);
                self.hthread = None;
                self.recv = None;
            }
        }
        ui.horizontal(|ui| {
            if ui.button("Generate").clicked() {
                if self.hthread.is_none() {
                    self.file_state = FileState::SaveImage;
                } else {
                    self.msg = Some(Msg::new(
                        "Generation in progress".to_string(),
                        MsgType::Info,
                    ));
                }
            }
            ui.label("Font Size");
            ui.add(egui::DragValue::new(&mut self.font_size).range(0.0..=f32::MAX));
            if let Some(ht) = &self.hthread {
                if ht.is_finished() {
                    self.hthread = None;
                } else {
                    ui.spinner();
                }
            }
        });
        ui.separator();
        ui.label("Shadow Settings");
        ui.horizontal(|ui| {
            ui.label("Color");
            ui.color_edit_button_srgba_unmultiplied(&mut self.shadow_color);
            ui.label("Background");
            match &self.shadow_background {
                ShadowBackground::Solid => {
                    if ui.button("Solid").clicked() {
                        self.shadow_background = ShadowBackground::Image;
                    }
                    ui.color_edit_button_srgba_unmultiplied(&mut self.shadow_background_solid);
                }
                ShadowBackground::Image => {
                    if ui.button("Image").clicked() {
                        self.shadow_background = ShadowBackground::Solid;
                    }
                    if ui.button("Load").clicked() {
                        self.file_state = FileState::OpenBackgroud;
                    }
                    ui.label(&self.shadow_background_image_info);
                }
            }
            ui.label("Blur Radius");
            ui.add(egui::DragValue::new(&mut self.shadow_blur_radius).range(0.0..=f32::MAX));
        });
        ui.horizontal(|ui| {
            ui.label("Horizontal Padding");
            ui.add(egui::DragValue::new(&mut self.shadow_pad_horizontal).range(0..=u32::MAX));
            ui.label("Vertical Padding");
            ui.add(egui::DragValue::new(&mut self.shadow_pad_vertical).range(0..=u32::MAX));
            ui.label("Offset X");
            ui.add(
                egui::DragValue::new(&mut self.shadow_offset_x).range(i32::MIN..=i32::MAX),
            );
            ui.label("Offset Y");
            ui.add(
                egui::DragValue::new(&mut self.shadow_offset_y).range(i32::MIN..=i32::MAX),
            );
        });
        ui.separator();
        let width = ui.available_width();
        ui.horizontal(|ui| {
            let mut cursor = ui.cursor();
            cursor.set_width(width / 4.0 - 5.0);
            // theme selector
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    let size = egui::vec2(width / 4.0 - 10.0, 20.0);
                    ui.add_sized(size, egui::Label::new(format!("Theme: {}", self.theme)));
                    ui.horizontal(|ui| {
                        ui.label("Filter");
                        ui.text_edit_singleline(&mut self.theme_search);
                    });
                    ui.separator();
                    for theme in self.highlightassets.theme_set.themes.keys() {
                        if self.theme_search.is_empty()
                            || theme
                                .to_lowercase()
                                .contains(&self.theme_search.to_lowercase())
                        {
                            ui.radio_value(&mut self.theme, theme.clone(), theme);
                        }
                    }
                });
            });
            cursor = ui.cursor();
            cursor.set_width(width / 4.0 - 5.0);
            // syntax selector
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    let size = egui::vec2(width / 4.0 - 10.0, 20.0);
                    ui.add_sized(size, egui::Label::new(format!("Syntax: {}", self.syntax)));
                    ui.horizontal(|ui| {
                        ui.label("Filter");
                        ui.text_edit_singleline(&mut self.syntax_search);
                    });
                    ui.separator();
                    for syntax in self.highlightassets.syntax_set.syntaxes() {
                        let syntax_name = syntax.name.clone();
                        if self.syntax_search.is_empty()
                            || syntax_name
                                .to_lowercase()
                                .contains(&self.syntax_search.to_lowercase())
                            || syntax.file_extensions.iter().any(|ext| {
                                ext.to_lowercase()
                                    .contains(&self.syntax_search.to_lowercase())
                            })
                        {
                            let display_name = format!(
                                "{} ({})",
                                syntax_name,
                                syntax.file_extensions.first().unwrap_or(&"??".to_string())
                            );
                            ui.radio_value(&mut self.syntax, syntax_name.clone(), display_name);
                        }
                    }
                });
            });
            cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            // code editor
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        use egui_extras::syntax_highlighting::*;
                        let mut layout_job = egui_extras::syntax_highlighting::highlight(
                            ctx,
                            ui.style(),
                            &CodeTheme::from_style(&ctx.style()),
                            string,
                            &self.syntax,

                        );
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };
                    ui.add(
                        egui::TextEdit::multiline(&mut self.code)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .code_editor()
                            .layouter(&mut layouter),
                    );
                });
            });
        });
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        match self.file_state {
            FileState::OpenBackgroud => {
                self.file_state = FileState::None;
                Some((
                    FileOp {
                        title: "Open Background".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Open,
                    },
                    OPEN_BACKGROUND,
                ))
            }
            FileState::SaveImage => {
                self.file_state = FileState::None;
                Some((
                    FileOp {
                        title: "Save Image".to_string(),
                        filter: vec!["png".to_string()],
                        mode: FileOpMode::Save,
                    },
                    SAVE_IMAGE,
                ))
            }
            _ => None,
        }
    }
    fn set_file_op(&mut self, file_path: Option<(path::PathBuf, u8)>) {
        if let Some((path, mode)) = file_path {
            match mode {
                OPEN_BACKGROUND => {
                    if let Err(e) = self.image_load(&path) {
                        self.msg = Some(Msg::new(e, MsgType::Error));
                    } else {
                        self.msg = Some(Msg::new(
                            format!("Image loaded from {}", path.to_string_lossy()),
                            MsgType::Info,
                        ));
                    }
                }
                SAVE_IMAGE => {
                    let local_args = GenArgs {
                        code: self.code.clone(),
                        font_size: self.font_size,
                        theme: self.theme.clone(),
                        theme_search: self.theme_search.clone(),
                        syntax: self.syntax.clone(),
                        syntax_search: self.syntax_search.clone(),
                        shadow_blur_radius: self.shadow_blur_radius,
                        shadow_pad_horizontal: self.shadow_pad_horizontal,
                        shadow_pad_vertical: self.shadow_pad_vertical,
                        shadow_offset_x: self.shadow_offset_x,
                        shadow_offset_y: self.shadow_offset_y,
                        shadow_color: self.shadow_color,
                        shadow_background: self.shadow_background.clone(),
                        shadow_background_solid: self.shadow_background_solid,
                        shadow_background_image: self.shadow_background_image.clone(),
                        shadow_background_image_info: self.shadow_background_image_info.clone(),
                    };
                    let local_path = path.clone();
                    let (tx, rx) = mpsc::channel();
                    self.recv = Some(rx);
                    let hthread = thread::spawn(move || {
                        let result = Self::generate_code_picture(&local_args, &local_path);
                        if let Err(e) = result {
                            tx.send(Msg::new(e, MsgType::Error)).unwrap();
                        } else {
                            tx.send(Msg::new(
                                format!("Image saved to {}", local_path.to_string_lossy()),
                                MsgType::Info,
                            ))
                            .unwrap();
                        }
                    });
                    self.hthread = Some(hthread);
                }
                _ => {}
            }
        }
    }
}
