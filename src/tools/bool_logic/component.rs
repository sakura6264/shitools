use super::super::*;
use super::calc;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::collections::BTreeMap;

const SAVE_JSON: u8 = 0;
const LOAD_JSON: u8 = 1;

#[derive(PartialEq, Eq, Hash, Clone)]
enum FileOpType {
    Save,
    Load,
    None,
}

pub struct BoolLogic {
    inputs: BTreeMap<char, String>,
    consts: Vec<String>,
    truthtable: calc::TruthTable,
    gen_input_num: usize,
    gen_output_num: usize,
    msg: Option<Msg>,
    op_type: FileOpType,
}
impl BoolLogic {
    pub fn new() -> Self {
        Self {
            inputs: BTreeMap::new(),
            consts: Vec::new(),
            truthtable: calc::TruthTable::new(0, 0).unwrap(),
            gen_input_num: 2,
            gen_output_num: 1,
            msg: None,
            op_type: FileOpType::None,
        }
    }
}

impl ToolComponent for BoolLogic {
    fn get_file_op(&mut self) -> Option<(FileOp, u8)> {
        let ret = match self.op_type {
            FileOpType::Save => Some((
                FileOp {
                    title: "Save".to_string(),
                    filter: vec!["json".to_string()],
                    mode: FileOpMode::Save,
                },
                SAVE_JSON,
            )),
            FileOpType::Load => Some((
                FileOp {
                    title: "Load".to_string(),
                    filter: vec!["json".to_string()],
                    mode: FileOpMode::Open,
                },
                LOAD_JSON,
            )),
            FileOpType::None => None,
        };
        self.op_type = FileOpType::None;
        ret
    }
    fn set_file_op(&mut self, file_open: Option<(path::PathBuf, u8)>) {
        if let Some((path, id)) = file_open {
            match id {
                SAVE_JSON => {
                    if let Ok(s) = serde_json::to_string(&self.truthtable) {
                        if let Err(err) = crate::write_file(&path, &s) {
                            self.msg = Some(Msg::new(err.to_string(), MsgType::Error));
                        } else {
                            self.msg = Some(Msg::new("Save Success".to_string(), MsgType::Info));
                        }
                    } else {
                        self.msg =
                            Some(Msg::new("Serialization Error".to_string(), MsgType::Error));
                    };
                }
                LOAD_JSON => {
                    if let Ok(s) = crate::read_file(&path) {
                        match serde_json::from_str::<calc::TruthTable>(&s) {
                            Ok(table) => {
                                self.truthtable = table;
                                self.msg =
                                    Some(Msg::new("Load Success".to_string(), MsgType::Info));
                            }
                            Err(err) => {
                                self.msg = Some(Msg::new(err.to_string(), MsgType::Error));
                            }
                        }
                    } else {
                        self.msg = Some(Msg::new(
                            "Deserialization Error".to_string(),
                            MsgType::Error,
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    fn get_msg(&mut self) -> Option<Msg> {
        let ret_msg = self.msg.clone();
        self.msg = None;
        ret_msg
    }
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.vertical(|ui| {
            ui.label("Expression");
            let mut remove = Vec::new();
            for (key, value) in self.inputs.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("{} = ", key));
                    ui.text_edit_singleline(value);
                    if ui.button(" - ").clicked() {
                        remove.push(*key);
                    }
                });
            }
            for key in remove {
                self.inputs.remove(&key);
            }

            ui.separator();
            ui.horizontal(|u| {
                if u.button(" + ").clicked() {
                    for i in calc::CHARLIST.iter() {
                        if !self.inputs.contains_key(i) {
                            self.inputs.insert(*i, String::new());
                            break;
                        }
                    }
                }
                if u.button("Clear").clicked() {
                    self.inputs.clear();
                }
            });
        });
        ui.separator();
        ui.vertical(|ui| {
            ui.label("Constrain");
            let mut remove = Vec::new();
            let mut id = 0;
            while id < self.consts.len() {
                ui.horizontal(|ui| {
                    ui.label("0 =");
                    ui.text_edit_singleline(&mut self.consts[id]);
                    if ui.button(" - ").clicked() {
                        remove.push(id);
                    }
                });
                id += 1;
            }
            for key in remove {
                self.consts.remove(key);
            }
            ui.separator();
            ui.horizontal(|u| {
                if u.button(" + ").clicked() {
                    self.consts.push(String::new());
                }
                if u.button("Clear").clicked() {
                    self.consts.clear();
                }
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Calculate").clicked() {
                match calc::TruthTable::calc(&self.inputs, &self.consts) {
                    Ok(table) => {
                        self.truthtable = table;
                    }
                    Err(err) => {
                        self.msg = Some(Msg::new(err.to_string(), MsgType::Error));
                    }
                }
            }
            if ui.button("Export").clicked() {
                self.op_type = FileOpType::Save;
            }
            if ui.button("Import").clicked() {
                self.op_type = FileOpType::Load;
            }
            if ui.button("Simplify").clicked() {
                self.inputs = calc::qmc_simplify(&self.truthtable);
                self.consts.clear();
            }
            if ui.button("Clear").clicked() {
                self.truthtable = calc::TruthTable::new(0, 0).unwrap();
            }
            if ui.button("New").clicked() {
                match calc::TruthTable::new(self.gen_input_num, self.gen_output_num) {
                    Some(table) => {
                        self.truthtable = table;
                    }
                    None => {
                        self.msg = Some(Msg::new("Too Many Variables".to_string(), MsgType::Error));
                    }
                }
            }
            ui.label("Input:");
            ui.add(
                egui::DragValue::new(&mut self.gen_input_num)
                    .speed(1.0)
                    .range(0..=calc::CHARLIST.len()),
            );
            ui.label("Output:");
            ui.add(
                egui::DragValue::new(&mut self.gen_output_num)
                    .speed(1.0)
                    .range(0..=calc::CHARLIST.len()),
            );
        });
        ui.vertical(|ui| {
            let mut builder = TableBuilder::new(ui);
            builder = builder.column(Column::auto().resizable(true).at_least(60.0));
            for _ in 0..self.truthtable.vars.len() {
                builder = builder.column(Column::auto().resizable(true).at_least(40.0));
            }
            builder = builder.column(Column::auto().resizable(true).at_least(60.0));
            for _ in 0..self.truthtable.outputs.len() {
                builder = builder.column(Column::auto().resizable(true).at_least(40.0));
            }
            builder = builder.column(Column::auto().resizable(false).at_least(20.0));
            let table = builder.header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label("Input");
                });
                for i in 0..self.truthtable.vars.len() {
                    header.col(|ui| {
                        ui.label(format!(" {} ", &self.truthtable.vars[i]));
                    });
                }
                header.col(|ui| {
                    ui.label("Output");
                });
                for i in 0..self.truthtable.outputs.len() {
                    header.col(|ui| {
                        ui.label(format!(" {} ", &self.truthtable.outputs[i]));
                    });
                }
                header.col(|_| {});
            });
            table.body(|mut body| {
                let mut input = BTreeMap::new();
                let mut index = 0;
                for i in self.truthtable.vars.iter() {
                    input.insert(i.clone(), false);
                }
                for _ in 0..2usize.pow(input.len() as u32) {
                    body.row(30.0, |mut row| {
                        row.col(|_| {});
                        for i in 0..self.truthtable.vars.len() {
                            row.col(|ui| {
                                ui.label(if *input.get(&self.truthtable.vars[i]).unwrap() {
                                    "1"
                                } else {
                                    "0"
                                });
                            });
                        }
                        row.col(|_| {});
                        for i in 0..self.truthtable.outputs.len() {
                            row.col(|ui| {
                                if ui
                                    .button(match self.truthtable.table[index][i] {
                                        calc::TruthTableResult::Val(true) => " 1 ",
                                        calc::TruthTableResult::Val(false) => " 0 ",
                                        calc::TruthTableResult::NotCare => " X ",
                                    })
                                    .clicked()
                                {
                                    match self.truthtable.table[index][i] {
                                        calc::TruthTableResult::Val(true) => {
                                            self.truthtable.table[index][i] =
                                                calc::TruthTableResult::Val(false)
                                        }
                                        calc::TruthTableResult::Val(false) => {
                                            self.truthtable.table[index][i] =
                                                calc::TruthTableResult::NotCare
                                        }
                                        calc::TruthTableResult::NotCare => {
                                            self.truthtable.table[index][i] =
                                                calc::TruthTableResult::Val(true)
                                        }
                                    }
                                }
                            });
                        }
                        row.col(|_| {});
                    });
                    calc::TruthTable::next(&mut input);
                    index += 1;
                }
            });
        });
    }
    fn on_close(&mut self) {}
}
