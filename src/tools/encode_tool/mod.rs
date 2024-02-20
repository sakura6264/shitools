use super::*;
use eframe::egui;

mod basic;
mod encoding;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Encoder {
    input: String,
    output: String,
    selected_catagory: Catagory,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            selected_catagory: Catagory::Op,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Catagory {
    Op,
    Basic,
    Encoding,
}

impl ToolComponent for Encoder {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        let width = ui.available_width();
        ui.horizontal(|ui| {
            let mut cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_ui_at_rect(cursor, |ui| {
                ui.vertical(|ui| {
                    ui.label("Input");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.input)
                            .desired_width(f32::INFINITY)
                            .desired_rows(10),
                    );
                });
            });
            cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_ui_at_rect(cursor, |ui| {
                ui.vertical(|ui| {
                    ui.label("Output");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.output)
                            .desired_width(f32::INFINITY)
                            .desired_rows(10),
                    );
                });
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.selected_catagory, Catagory::Op, "Operation");
            ui.radio_value(&mut self.selected_catagory, Catagory::Basic, "Basic");
            ui.radio_value(&mut self.selected_catagory, Catagory::Encoding, "Encoding");
        });
        match self.selected_catagory {
            Catagory::Op => {
                ui.horizontal(|ui| {
                    if ui.button("Clear Input").clicked() {
                        self.input.clear();
                    }
                    if ui.button("Clear Output").clicked() {
                        self.output.clear();
                    }
                    if ui.button("Swap").clicked() {
                        std::mem::swap(&mut self.input, &mut self.output);
                    }
                    if ui.button("Copy Input").clicked() {
                        ui.output_mut(|o| {
                            o.copied_text = self.input.clone();
                        });
                    }
                    if ui.button("Copy Output").clicked() {
                        ui.output_mut(|o| {
                            o.copied_text = self.output.clone();
                        });
                    }
                });
            }
            Catagory::Basic => {
                ui.label("Base64");
                ui.horizontal(|ui| {
                    if ui.button("Encode").clicked() {
                        self.output = basic::base64_encode_std(&self.input);
                    }
                    if ui.button("Decode").clicked() {
                        self.output = basic::base64_decode_std(&self.input);
                    }
                    if ui.button("Encode URL").clicked() {
                        self.output = basic::base64_encode_url(&self.input);
                    }
                    if ui.button("Decode URL").clicked() {
                        self.output = basic::base64_decode_url(&self.input);
                    }
                });
                ui.label("Base64 No Padding");
                ui.horizontal(|ui| {
                    if ui.button("Encode").clicked() {
                        self.output = basic::base64_encode_std_no_pad(&self.input);
                    }
                    if ui.button("Decode").clicked() {
                        self.output = basic::base64_decode_std_no_pad(&self.input);
                    }
                    if ui.button("Encode URL").clicked() {
                        self.output = basic::base64_encode_url_no_pad(&self.input);
                    }
                    if ui.button("Decode URL").clicked() {
                        self.output = basic::base64_decode_url_no_pad(&self.input);
                    }
                });
                ui.label("URL");
                ui.horizontal(|ui| {
                    if ui.button("Encode").clicked() {
                        self.output = basic::url_encode(&self.input);
                    }
                    if ui.button("Decode").clicked() {
                        self.output = basic::url_decode(&self.input);
                    }
                });
                ui.label("Hex");
                ui.horizontal(|ui| {
                    if ui.button("Encode").clicked() {
                        self.output = basic::hex_encode(&self.input);
                    }
                    if ui.button("Decode").clicked() {
                        self.output = basic::hex_decode(&self.input);
                    }
                });
            }
            Catagory::Encoding => {
                ui.horizontal(|ui| {
                    ui.label("UTF-8");
                    ui.horizontal(|ui| {
                        if ui.button("To Hex").clicked() {
                            self.output = encoding::utf8_to_hex(&self.input);
                        }
                        if ui.button("From Hex").clicked() {
                            self.output = encoding::hex_to_utf8(&self.input);
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("UTF-16LE");
                    ui.horizontal(|ui| {
                        if ui.button("To Hex").clicked() {
                            self.output = encoding::utf16le_to_hex(&self.input);
                        }
                        if ui.button("From Hex").clicked() {
                            self.output = encoding::hex_to_utf16le(&self.input);
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("UTF-16BE");
                    ui.horizontal(|ui| {
                        if ui.button("To Hex").clicked() {
                            self.output = encoding::utf16be_to_hex(&self.input);
                        }
                        if ui.button("From Hex").clicked() {
                            self.output = encoding::hex_to_utf16be(&self.input);
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("GBK");
                    ui.horizontal(|ui| {
                        if ui.button("To Hex").clicked() {
                            self.output = encoding::gbk_to_hex(&self.input);
                        }
                        if ui.button("From Hex").clicked() {
                            self.output = encoding::hex_to_gbk(&self.input);
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Shift-JIS");
                    ui.horizontal(|ui| {
                        if ui.button("To Hex").clicked() {
                            self.output = encoding::shiftjis_to_hex(&self.input);
                        }
                        if ui.button("From Hex").clicked() {
                            self.output = encoding::hex_to_shiftjis(&self.input);
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Big5");
                    ui.horizontal(|ui| {
                        if ui.button("To Hex").clicked() {
                            self.output = encoding::big5_to_hex(&self.input);
                        }
                        if ui.button("From Hex").clicked() {
                            self.output = encoding::hex_to_big5(&self.input);
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("ascii");
                    ui.horizontal(|ui| {
                        if ui.button("To Hex").clicked() {
                            self.output = encoding::ascii_to_hex(&self.input);
                        }
                        if ui.button("From Hex").clicked() {
                            self.output = encoding::hex_to_ascii(&self.input);
                        }
                    });
                });
            }
        }
    }
}
