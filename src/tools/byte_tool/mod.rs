use super::*;
use eframe::egui;

mod basic;
mod hashes;
mod summarize;

const LOAD_FILE: u8 = 0;
const SAVE_FILE: u8 = 1;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Catagory {
    Basic,
    Hash(HashType),
    Summarize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum HashType {
    MDSome,
    SHA12,
    SHA3,
    SHABAL,
    FSB,
    GOST94,
    GROESTL,
    JH,
    OTHER,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ByteTool {
    input: Vec<u8>,
    display_cache: String,
    output: String,
    selected_catagory: Catagory,
    msg: Option<Msg>,
    file_op: SLState,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum SLState {
    Load,
    Save,
    None,
}

impl ByteTool {
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            display_cache: String::new(),
            output: String::new(),
            selected_catagory: Catagory::Basic,
            msg: None,
            file_op: SLState::None,
        }
    }
    fn set_input(&mut self, input: Vec<u8>) {
        self.input = input;
        const MAX_DISPLAY: usize = 64;
        if self.input.len() > MAX_DISPLAY {
            self.display_cache = Self::byte2display(&self.input[0..MAX_DISPLAY]) + "...";
        } else {
            self.display_cache = Self::byte2display(&self.input);
        }
    }
    fn delete_input(&mut self) {
        self.input.clear();
        self.display_cache.clear();
    }
    fn byte2display(input: &[u8]) -> String {
        let output_str = hex::encode(input);
        let output_bytes = output_str.as_bytes();
        let mut buffer_out = Vec::new();
        for i in 0..output_bytes.len() {
            if i % 2 == 0 && i != 0 {
                buffer_out.push(b' ');
            }
            buffer_out.push(output_bytes[i]);
        }
        String::from_utf8(buffer_out).unwrap()
    }
}

impl ToolComponent for ByteTool {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.selected_catagory, Catagory::Basic, "Basic");
            ui.radio_value(
                &mut self.selected_catagory,
                Catagory::Hash(HashType::SHA12),
                "Hash",
            );
            ui.radio_value(
                &mut self.selected_catagory,
                Catagory::Summarize,
                "Summarize",
            );
        });
        match self.selected_catagory {
            Catagory::Basic => {
                ui.horizontal(|ui| {
                    if ui.button("Base64 Encode").clicked() {
                        self.output = basic::base64_encode(&self.input);
                    }
                    if ui.button("Base64 Decode").clicked() {
                        match basic::base64_decode(&self.output) {
                            Ok(v) => self.set_input(v),
                            Err(e) => {
                                self.msg = Some(Msg::new(format!("Error: {}", e), MsgType::Error))
                            }
                        }
                    }
                    if ui.button("HEX Encode").clicked() {
                        self.output = basic::hex_encode(&self.input);
                    }
                    if ui.button("HEX Decode").clicked() {
                        match basic::hex_decode(&self.output) {
                            Ok(v) => self.set_input(v),
                            Err(e) => {
                                self.msg = Some(Msg::new(format!("Error: {}", e), MsgType::Error))
                            }
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Try ASCII").clicked() {
                        self.output = basic::try_ascii(&self.input);
                    }
                });
            }
            Catagory::Hash(ref mut ht) => {
                ui.horizontal(|ui| {
                    ui.radio_value(ht, HashType::MDSome, "MD?");
                    ui.radio_value(ht, HashType::SHA12, "SHA1/2");
                    ui.radio_value(ht, HashType::SHA3, "SHA3");
                    ui.radio_value(ht, HashType::SHABAL, "SHABAL");
                    ui.radio_value(ht, HashType::FSB, "FSB");
                    ui.radio_value(ht, HashType::GOST94, "GOST94");
                    ui.radio_value(ht, HashType::GROESTL, "GROESTL");
                    ui.radio_value(ht, HashType::JH, "JH");
                    ui.radio_value(ht, HashType::OTHER, "Other");
                });
                ui.horizontal(|ui| match ht {
                    HashType::MDSome => {
                        if ui.button("MD2").clicked() {
                            self.output = hashes::md2(&self.input);
                        }
                        if ui.button("MD4").clicked() {
                            self.output = hashes::md4(&self.input);
                        }
                        if ui.button("MD5").clicked() {
                            self.output = hashes::md5(&self.input);
                        }
                        if ui.button("RIPEMD128").clicked() {
                            self.output = hashes::ripemd128(&self.input);
                        }
                        if ui.button("RIPEMD160").clicked() {
                            self.output = hashes::ripemd160(&self.input);
                        }
                        if ui.button("RIPEMD256").clicked() {
                            self.output = hashes::ripemd256(&self.input);
                        }
                        if ui.button("RIPEMD320").clicked() {
                            self.output = hashes::ripemd320(&self.input);
                        }
                    }
                    HashType::SHA12 => {
                        if ui.button("SHA1").clicked() {
                            self.output = hashes::sha1(&self.input);
                        }
                        if ui.button("SHA224").clicked() {
                            self.output = hashes::sha224(&self.input);
                        }
                        if ui.button("SHA256").clicked() {
                            self.output = hashes::sha256(&self.input);
                        }
                        if ui.button("SHA384").clicked() {
                            self.output = hashes::sha384(&self.input);
                        }
                        if ui.button("SHA512").clicked() {
                            self.output = hashes::sha512(&self.input);
                        }
                        if ui.button("SHA512-224").clicked() {
                            self.output = hashes::sha512_224(&self.input);
                        }
                        if ui.button("SHA512-256").clicked() {
                            self.output = hashes::sha512_256(&self.input);
                        }
                    }
                    HashType::SHA3 => {
                        if ui.button("SHA3-224").clicked() {
                            self.output = hashes::sha3_224(&self.input);
                        }
                        if ui.button("SHA3-256").clicked() {
                            self.output = hashes::sha3_256(&self.input);
                        }
                        if ui.button("SHA3-384").clicked() {
                            self.output = hashes::sha3_384(&self.input);
                        }
                        if ui.button("SHA3-512").clicked() {
                            self.output = hashes::sha3_512(&self.input);
                        }
                        if ui.button("KECCAK224").clicked() {
                            self.output = hashes::keccak224(&self.input);
                        }
                        if ui.button("KECCAK256").clicked() {
                            self.output = hashes::keccak256(&self.input);
                        }
                        if ui.button("KECCAK384").clicked() {
                            self.output = hashes::keccak384(&self.input);
                        }
                        if ui.button("KECCAK512").clicked() {
                            self.output = hashes::keccak512(&self.input);
                        }
                        if ui.button("KECCAK256FULL").clicked() {
                            self.output = hashes::keccak256full(&self.input);
                        }
                    }
                    HashType::SHABAL => {
                        if ui.button("SHABAL192").clicked() {
                            self.output = hashes::shabal192(&self.input);
                        }
                        if ui.button("SHABAL224").clicked() {
                            self.output = hashes::shabal224(&self.input);
                        }
                        if ui.button("SHABAL256").clicked() {
                            self.output = hashes::shabal256(&self.input);
                        }
                        if ui.button("SHABAL384").clicked() {
                            self.output = hashes::shabal384(&self.input);
                        }
                        if ui.button("SHABAL512").clicked() {
                            self.output = hashes::shabal512(&self.input);
                        }
                    }
                    HashType::FSB => {
                        if ui.button("FSB160").clicked() {
                            self.output = hashes::fsb160(&self.input);
                        }
                        if ui.button("FSB224").clicked() {
                            self.output = hashes::fsb224(&self.input);
                        }
                        if ui.button("FSB256").clicked() {
                            self.output = hashes::fsb256(&self.input);
                        }
                        if ui.button("FSB384").clicked() {
                            self.output = hashes::fsb384(&self.input);
                        }
                        if ui.button("FSB512").clicked() {
                            self.output = hashes::fsb512(&self.input);
                        }
                    }
                    HashType::GOST94 => {
                        if ui.button("GOST94-UA").clicked() {
                            self.output = hashes::gost94ua(&self.input);
                        }
                        if ui.button("GOST94-CryptoPro").clicked() {
                            self.output = hashes::gost94cryptopro(&self.input);
                        }
                        if ui.button("GOST94-Test").clicked() {
                            self.output = hashes::gost94test(&self.input);
                        }
                        if ui.button("GOST94-S2015").clicked() {
                            self.output = hashes::gost94s2015(&self.input);
                        }
                    }
                    HashType::GROESTL => {
                        if ui.button("GROESTL224").clicked() {
                            self.output = hashes::groestl224(&self.input);
                        }
                        if ui.button("GROESTL256").clicked() {
                            self.output = hashes::groestl256(&self.input);
                        }
                        if ui.button("GROESTL384").clicked() {
                            self.output = hashes::groestl384(&self.input);
                        }
                        if ui.button("GROESTL512").clicked() {
                            self.output = hashes::groestl512(&self.input);
                        }
                    }
                    HashType::JH => {
                        if ui.button("JH224").clicked() {
                            self.output = hashes::jh224(&self.input);
                        }
                        if ui.button("JH256").clicked() {
                            self.output = hashes::jh256(&self.input);
                        }
                        if ui.button("JH384").clicked() {
                            self.output = hashes::jh384(&self.input);
                        }
                        if ui.button("JH512").clicked() {
                            self.output = hashes::jh512(&self.input);
                        }
                    }
                    HashType::OTHER => {
                        if ui.button("Ascon").clicked() {
                            self.output = hashes::asconhash(&self.input);
                        }
                        if ui.button("AsconA").clicked() {
                            self.output = hashes::asconahash(&self.input);
                        }
                        if ui.button("Blake3").clicked() {
                            self.output = hashes::blake3(&self.input);
                        }
                        if ui.button("Belt").clicked() {
                            self.output = hashes::belt(&self.input);
                        }
                        if ui.button("SM3").clicked() {
                            self.output = hashes::sm3(&self.input);
                        }
                        if ui.button("Tiger").clicked() {
                            self.output = hashes::tiger(&self.input);
                        }
                        if ui.button("Tiger2").clicked() {
                            self.output = hashes::tiger2(&self.input);
                        }
                        if ui.button("Streebog-256").clicked() {
                            self.output = hashes::streebog256(&self.input);
                        }
                        if ui.button("Streebog-512").clicked() {
                            self.output = hashes::streebog512(&self.input);
                        }
                        if ui.button("Whirlpool").clicked() {
                            self.output = hashes::whirlpool(&self.input);
                        }
                    }
                });
            }
            Catagory::Summarize => {
                ui.horizontal(|ui| {
                    if ui
                        .button("Find Max")
                        .on_hover_text("Find which byte used the max times.")
                        .clicked()
                    {
                        self.output = hex::encode([summarize::find_max_used(&self.input)]);
                    }
                    if ui.button("Size").clicked() {
                        self.output = self.input.len().to_string();
                    }
                    if ui.button("Count 00").clicked() {
                        self.output = summarize::calc_00(&self.input).to_string();
                    }
                    if ui.button("Count FF").clicked() {
                        self.output = summarize::calc_ff(&self.input).to_string();
                    }
                    if ui
                        .button("Count Byte")
                        .on_hover_text("Count the byte in the \"Output\".")
                        .clicked()
                    {
                        match hex::decode(&self.output) {
                            Ok(v) => {
                                if v.len() == 1 {
                                    self.output =
                                        summarize::find_num(&self.input, v[0]).to_string();
                                } else {
                                    self.msg = Some(Msg::new(
                                        "Error: Please input only one byte.".to_string(),
                                        MsgType::Error,
                                    ));
                                }
                            }
                            Err(e) => {
                                self.msg = Some(Msg::new(format!("Error: {}", e), MsgType::Error));
                            }
                        }
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("00 Percent").clicked() {
                        self.output = format!(
                            "{:.2}%",
                            summarize::calc_00(&self.input) as f64 / self.input.len() as f64
                                * 100.0
                        );
                    }
                    if ui.button("FF Percent").clicked() {
                        self.output = format!(
                            "{:.2}%",
                            summarize::calc_ff(&self.input) as f64 / self.input.len() as f64
                                * 100.0
                        );
                    }
                    if ui
                        .button("Byte Percent")
                        .on_hover_text("Count the byte percentage in the \"Output\".")
                        .clicked()
                    {
                        match hex::decode(&self.output) {
                            Ok(v) => {
                                if v.len() == 1 {
                                    self.output = format!(
                                        "{:.2}%",
                                        summarize::find_num(&self.input, v[0]) as f64
                                            / self.input.len() as f64
                                            * 100.0
                                    );
                                } else {
                                    self.msg = Some(Msg::new(
                                        "Error: Please input only one byte.".to_string(),
                                        MsgType::Error,
                                    ));
                                }
                            }
                            Err(e) => {
                                self.msg = Some(Msg::new(format!("Error: {}", e), MsgType::Error));
                            }
                        }
                    }
                    if ui
                        .button("Xor")
                        .on_hover_text("Apply Output XOR to Input")
                        .clicked()
                    {
                        match hex::decode(&self.output) {
                            Ok(v) => {
                                self.output = summarize::xor(&self.input, &v);
                            }
                            Err(e) => {
                                self.msg = Some(Msg::new(format!("Error: {}", e), MsgType::Error));
                            }
                        }
                    }
                    if ui
                        .button("Guess")
                        .on_hover_text("Guess the input file format")
                        .clicked()
                    {
                        self.output = summarize::format_guess(&self.input);
                    }
                });
            }
        }
        ui.separator();
        let width = ui.available_width();
        ui.horizontal(|ui| {
            let mut cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Clear").clicked() {
                            self.delete_input();
                        }
                        if ui.button("Load").clicked() {
                            self.file_op = SLState::Load;
                        }
                        if ui.button("Save").clicked() {
                            self.file_op = SLState::Save;
                        }
                    });
                    ui.label("Preview HEX");
                    ui.add_enabled(
                        false,
                        egui::TextEdit::multiline(&mut self.display_cache)
                            .desired_width(f32::INFINITY)
                            .desired_rows(9),
                    );
                });
            });
            cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Output");
                        if ui.button("Clear").clicked() {
                            self.output.clear();
                        }
                        if ui.button("Copy").clicked() {
                            ui.output_mut(|w| {
                                w.copied_text = self.output.clone();
                            });
                        }
                    });
                    ui.add(
                        egui::TextEdit::multiline(&mut self.output)
                            .desired_width(f32::INFINITY)
                            .desired_rows(10),
                    );
                });
            });
        });
    }
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        match self.file_op {
            SLState::Load => {
                self.file_op = SLState::None;
                Some((
                    FileOp {
                        title: "Load File".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Open,
                    },
                    LOAD_FILE,
                ))
            }
            SLState::Save => {
                self.file_op = SLState::None;
                Some((
                    FileOp {
                        title: "Save File".to_string(),
                        filter: Vec::new(),
                        mode: FileOpMode::Save,
                    },
                    SAVE_FILE,
                ))
            }
            SLState::None => None,
        }
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
    fn set_file_op(&mut self, file_path: Option<(path::PathBuf, u8)>) {
        if let Some((fp, id)) = file_path {
            match id {
                LOAD_FILE => match std::fs::read(fp) {
                    Ok(v) => {
                        self.set_input(v);
                        self.msg = Some(Msg::new("Load Success".to_string(), MsgType::Info));
                    }
                    Err(e) => {
                        self.msg = Some(Msg::new(format!("Error: {}", e), MsgType::Error));
                    }
                },
                SAVE_FILE => match std::fs::write(fp, &self.input) {
                    Ok(_) => {
                        self.msg = Some(Msg::new("Save Success".to_string(), MsgType::Info));
                    }
                    Err(e) => {
                        self.msg = Some(Msg::new(format!("Error: {}", e), MsgType::Error));
                    }
                },
                _ => {}
            }
        }
    }
}
