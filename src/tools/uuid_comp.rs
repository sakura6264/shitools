use super::*;
use eframe::egui;

#[derive(PartialEq, Eq, Hash, Clone)]
enum UUIDNamespace {
    DNS,
    URL,
    OID,
    X500,
}

impl UUIDNamespace {
    fn get_uuid(&self) -> uuid::Uuid {
        match self {
            UUIDNamespace::DNS => uuid::Uuid::NAMESPACE_DNS,
            UUIDNamespace::URL => uuid::Uuid::NAMESPACE_URL,
            UUIDNamespace::OID => uuid::Uuid::NAMESPACE_OID,
            UUIDNamespace::X500 => uuid::Uuid::NAMESPACE_X500,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum UUIDType {
    V1(String),
    V3((String, UUIDNamespace)),
    V4,
    V5((String, UUIDNamespace)),
    V6(String),
    V7,
    V8(String),
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum UUIDOutputType {
    Hyphenated,
    Simple,
    Urn,
    Braced,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct UUIDGenerator {
    uuid: String,
    output_type: UUIDOutputType,
    algo: UUIDType,
}

impl UUIDGenerator {
    pub fn new() -> Self {
        Self {
            uuid: String::new(),
            output_type: UUIDOutputType::Hyphenated,
            algo: UUIDType::V4,
        }
    }
}

impl ToolComponent for UUIDGenerator {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    self.uuid.clear();
                }
                if ui.button("Copy").clicked() {
                    ui.output_mut(|reader| {
                        reader.copied_text = self.uuid.clone();
                    });
                }
            });
            ui.add(egui::TextEdit::singleline(&mut self.uuid).desired_width(f32::INFINITY));
            ui.horizontal(|ui| {
                ui.label("Output Type:");
                ui.radio_value(
                    &mut self.output_type,
                    UUIDOutputType::Hyphenated,
                    "Hyphenated",
                );
                ui.radio_value(&mut self.output_type, UUIDOutputType::Simple, "Simple");
                ui.radio_value(&mut self.output_type, UUIDOutputType::Urn, "Urn");
                ui.radio_value(&mut self.output_type, UUIDOutputType::Braced, "Braced");
            });
            ui.horizontal(|ui| {
                ui.label("Algorithm:");
                ui.selectable_value(&mut self.algo, UUIDType::V1("00".repeat(6)), "V1");
                ui.selectable_value(
                    &mut self.algo,
                    UUIDType::V3((String::new(), UUIDNamespace::DNS)),
                    "V3",
                );
                ui.selectable_value(&mut self.algo, UUIDType::V4, "V4");
                ui.selectable_value(
                    &mut self.algo,
                    UUIDType::V5((String::new(), UUIDNamespace::DNS)),
                    "V5",
                );
                ui.selectable_value(&mut self.algo, UUIDType::V6("00".repeat(6)), "V6");
                ui.selectable_value(&mut self.algo, UUIDType::V7, "V7");
                ui.selectable_value(&mut self.algo, UUIDType::V8("00".repeat(16)), "V8");
            });
            match self.algo {
                UUIDType::V1(ref mut s) => {
                    if let Some(node_id) = hex_edit(ui, "node id:", 6, s) {
                        let node_id = <[u8; 6]>::try_from(node_id).unwrap();
                        if ui.button("Generate").clicked() {
                            self.uuid = get_string_from_uuid(
                                &uuid::Uuid::now_v1(&node_id),
                                &self.output_type,
                            );
                        }
                    }
                }
                UUIDType::V3((ref mut name, ref mut namespace)) => {
                    ui.horizontal(|ui| {
                        ui.label("Namespace:");
                        ui.radio_value(namespace, UUIDNamespace::DNS, "DNS");
                        ui.radio_value(namespace, UUIDNamespace::URL, "URL");
                        ui.radio_value(namespace, UUIDNamespace::OID, "OID");
                        ui.radio_value(namespace, UUIDNamespace::X500, "X500");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(name);
                    });
                    if ui.button("Generate").clicked() {
                        self.uuid = get_string_from_uuid(
                            &uuid::Uuid::new_v3(&namespace.get_uuid(), name.as_bytes()),
                            &self.output_type,
                        );
                    }
                }
                UUIDType::V4 => {
                    if ui.button("Generate").clicked() {
                        self.uuid = get_string_from_uuid(&uuid::Uuid::new_v4(), &self.output_type);
                    }
                }
                UUIDType::V5((ref mut name, ref mut namespace)) => {
                    ui.horizontal(|ui| {
                        ui.label("Namespace:");
                        ui.radio_value(namespace, UUIDNamespace::DNS, "DNS");
                        ui.radio_value(namespace, UUIDNamespace::URL, "URL");
                        ui.radio_value(namespace, UUIDNamespace::OID, "OID");
                        ui.radio_value(namespace, UUIDNamespace::X500, "X500");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(name);
                    });
                    if ui.button("Generate").clicked() {
                        self.uuid = get_string_from_uuid(
                            &uuid::Uuid::new_v5(&namespace.get_uuid(), name.as_bytes()),
                            &self.output_type,
                        );
                    }
                }
                UUIDType::V6(ref mut s) => {
                    if let Some(node_id) = hex_edit(ui, "node id:", 6, s) {
                        let node_id = <[u8; 6]>::try_from(node_id).unwrap();
                        if ui.button("Generate").clicked() {
                            self.uuid = get_string_from_uuid(
                                &uuid::Uuid::now_v6(&node_id),
                                &self.output_type,
                            );
                        }
                    }
                }
                UUIDType::V7 => {
                    if ui.button("Generate").clicked() {
                        self.uuid = get_string_from_uuid(&uuid::Uuid::now_v7(), &self.output_type);
                    }
                }
                UUIDType::V8(ref mut s) => {
                    if let Some(node_id) = hex_edit(ui, "node id:", 16, s) {
                        let node_id = <[u8; 16]>::try_from(node_id).unwrap();
                        if ui.button("Generate").clicked() {
                            self.uuid = get_string_from_uuid(
                                &uuid::Uuid::new_v8(node_id),
                                &self.output_type,
                            );
                        }
                    }
                }
            }
        });
    }
}

fn hex_edit(ui: &mut egui::Ui, label: &str, length: usize, instr: &mut String) -> Option<Vec<u8>> {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(instr);
    });
    match hex::decode(instr) {
        Ok(bytes) => {
            if bytes.len() == length {
                Some(bytes)
            } else {
                ui.label("Invalid length");
                None
            }
        }
        Err(e) => {
            ui.label(format!("Invalid hex: {}", e));
            None
        }
    }
}

fn get_string_from_uuid(id: &uuid::Uuid, output_type: &UUIDOutputType) -> String {
    match output_type {
        UUIDOutputType::Hyphenated => id.as_hyphenated().to_string(),
        UUIDOutputType::Simple => id.as_simple().to_string(),
        UUIDOutputType::Urn => id.as_urn().to_string(),
        UUIDOutputType::Braced => id.as_braced().to_string(),
    }
}
