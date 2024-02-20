use eframe::egui;
use std::path;
mod batch_process;
mod blank;
mod bool_logic;
mod byte_tool;
mod code_picture;
mod encode_tool;
mod fake_data_gen;
mod game2048;
mod json_tool;
mod lua_tool;
mod randstr;
mod text_util;
mod time_tool;
mod uuid_comp;
mod vt_comp;

pub use blank::Blank;
use std::sync::OnceLock;

pub fn static_namelist() -> &'static Vec<(String, Vec<(String, Tools)>)> {
    static ONCE: OnceLock<Vec<(String, Vec<(String, Tools)>)>> = OnceLock::new();
    ONCE.get_or_init(|| get_component_namelist())
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Tools {
    Blank,
    BoolLogic,
    Vtracer,
    UUID,
    RandomString,
    FakeDataGen,
    EncodeTool,
    ByteTool,
    JsonTool,
    TimeTool,
    LuaTool,
    TexctUtil,
    BatchProcess,
    Game2048,
    CodePicture,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FileOp {
    pub title: String,
    pub filter: Vec<String>,
    pub mode: FileOpMode,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum FileOpMode {
    Open,
    Save,
    Dir,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Msg {
    pub text: String,
    pub msg_type: MsgType,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum MsgType {
    Error,
    Warning,
    Info,
}

pub trait ToolComponent {
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        None
    }
    fn set_file_op(&mut self, _file_path: Option<(path::PathBuf, u8)>) {}
    fn get_msg(&mut self) -> Option<Msg> {
        None
    }
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context);
    fn on_close(&mut self) {}
}

impl Msg {
    pub fn new(text: String, msg_type: MsgType) -> Self {
        Self { text, msg_type }
    }
}

pub fn get_component(component: Tools) -> Box<dyn ToolComponent> {
    match component {
        Tools::BoolLogic => Box::new(bool_logic::BoolLogic::new()),
        Tools::Vtracer => Box::new(vt_comp::Vtracer::new()),
        Tools::UUID => Box::new(uuid_comp::UUIDGenerator::new()),
        Tools::RandomString => Box::new(randstr::RandStr::new()),
        Tools::FakeDataGen => Box::new(fake_data_gen::FakeDataGen::new()),
        Tools::EncodeTool => Box::new(encode_tool::Encoder::new()),
        Tools::ByteTool => Box::new(byte_tool::ByteTool::new()),
        Tools::JsonTool => Box::new(json_tool::JsonTool::new()),
        Tools::TimeTool => Box::new(time_tool::TimeTool::new()),
        Tools::LuaTool => match lua_tool::LuaTool::new() {
            Ok(t) => Box::new(t),
            Err(e) => Box::new(InitError::new(e)),
        },
        Tools::TexctUtil => Box::new(text_util::TextUtil::new()),
        Tools::BatchProcess => Box::new(batch_process::BatchProcess::new()),
        Tools::Game2048 => Box::new(game2048::Game2048::new()),
        Tools::CodePicture => match code_picture::CodePicture::new() {
            Ok(t) => Box::new(t),
            Err(e) => Box::new(InitError::new(e)),
        },
        _ => Box::new(blank::Blank),
    }
}

pub fn get_component_namelist() -> Vec<(String, Vec<(String, Tools)>)> {
    let mut list = Vec::new();

    let mut math_list = Vec::new();
    math_list.push(("Bool Logic".to_string(), Tools::BoolLogic));
    math_list.push(("Time Tool".to_string(), Tools::TimeTool));
    math_list.push(("Lua".to_string(), Tools::LuaTool));
    list.push(("Math".to_string(), math_list));

    let mut string_list = Vec::new();
    string_list.push(("UUID".to_string(), Tools::UUID));
    string_list.push(("Random String".to_string(), Tools::RandomString));
    string_list.push(("Fake Data".to_string(), Tools::FakeDataGen));
    string_list.push(("Encode Tool".to_string(), Tools::EncodeTool));
    string_list.push(("JSON Tool".to_string(), Tools::JsonTool));
    string_list.push(("Text Util".to_string(), Tools::TexctUtil));
    list.push(("String".to_string(), string_list));

    let mut file_list = Vec::new();
    file_list.push(("Vtracer".to_string(), Tools::Vtracer));
    file_list.push(("Byte Tool".to_string(), Tools::ByteTool));
    file_list.push(("Batch Process".to_string(), Tools::BatchProcess));
    file_list.push(("Code Picture".to_string(), Tools::CodePicture));
    list.push(("File".to_string(), file_list));

    let mut about_list = Vec::new();
    about_list.push(("Welcome".to_string(), Tools::Blank));
    about_list.push(("2048".to_string(), Tools::Game2048));
    list.push(("About".to_string(), about_list));

    return list;
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct InitError {
    msg: String,
}

impl InitError {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl ToolComponent for InitError {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.heading("Error");
        ui.label(&self.msg);
    }
}
