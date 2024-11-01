use super::*;
use eframe::egui;
use mlua::prelude::*;

static LUA_STDOUT: std::sync::RwLock<Vec<String>> = std::sync::RwLock::new(Vec::new());
fn lua_print(vals: &LuaMultiValue) -> i32 {
    let mut stdout = match LUA_STDOUT.write() {
        Ok(w) => w,
        Err(_) => return -1,
    };
    let mut buffer = Vec::new();
    for v in vals {
        match v.to_string() {
            Ok(s) => buffer.push(s),
            Err(_) => return i32::MAX,
        }
    }
    stdout.push(buffer.join(" "));
    0
}

pub struct LuaTool {
    lua_stack: Lua,
    input: String,
    return_output: String,
    std_output: String,
    msg: Option<Msg>,
    info: String,
    clear_after_exec: bool,
}

impl LuaTool {
    pub fn new() -> Result<Self, String> {
        let mut ret = Self {
            lua_stack: Self::lua_build()?,
            input: String::new(),
            return_output: String::new(),
            std_output: String::new(),
            msg: None,
            info: String::new(),
            clear_after_exec: false,
        };
        ret.init()?;
        ret.build_info();
        Ok(ret)
    }
    fn lua_build() -> Result<Lua, String> {
        // only use table, string, jit and bit lib
        let stdlib = LuaStdLib::TABLE
            | LuaStdLib::STRING
            | LuaStdLib::JIT
            | LuaStdLib::BIT
            | LuaStdLib::MATH;
        let lua = Lua::new_with(stdlib, LuaOptions::default());
        lua.map_err(|e| e.to_string())
    }
    fn fetch_stdout(&mut self) {
        let stdout = match LUA_STDOUT.write() {
            Ok(mut w) => {
                let mut ret = String::new();
                for s in w.iter() {
                    ret.push_str(s);
                    ret.push('\n');
                }
                w.clear();
                ret
            }
            Err(e) => {
                self.msg = Some(Msg::new(e.to_string(), MsgType::Error));
                return;
            }
        };
        self.std_output.push_str(&stdout);
    }
    fn init(&mut self) -> Result<(), String> {
        // set a custom print function
        let lua_print = self
            .lua_stack
            .create_function(|_, vals: LuaMultiValue| Ok(lua_print(&vals)))
            .map_err(|e| e.to_string())?;
        self.lua_stack
            .globals()
            .set("print", lua_print)
            .map_err(|e| e.to_string())?;
        // preload lua script from lua/preload.lua
        let preload_path = crate::sub_path("lua/preload.lua")?;
        let preload_dir = preload_path
            .parent()
            .ok_or("preload.lua has no parent".to_string())?
            .to_path_buf();
        crate::ensure_dir(&preload_dir)?;
        crate::ensure_file(&preload_path)?;
        let preload_text = crate::read_file(&preload_path)?;
        self.lua_stack
            .load(&preload_text)
            .exec()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    fn build_info(&mut self) {
        let vm_mem = crate::format_mem(self.lua_stack.used_memory());
        self.info = format!("Lua VM Memory: {}", vm_mem);
    }
    fn exec_code(&mut self) {
        match self.lua_stack.load(&self.input).eval::<mlua::MultiValue>() {
            Ok(r) => {
                let mut buildstr = if r.is_empty() {
                    "=> [No Return Value]\n".to_string()
                } else {
                    let mut strcollect = "=> ".to_string();
                    strcollect.push_str(
                        &r.iter()
                            .map(|v| format!("{:#?} [type:{}]", v, v.type_name()))
                            .collect::<Vec<String>>()
                            .join("\n.. "),
                    );
                    strcollect.push('\n');
                    strcollect
                };
                buildstr.push_str(&self.return_output);
                self.return_output = buildstr;
            }
            Err(e) => self.msg = Some(Msg::new(e.to_string(), MsgType::Error)),
        }
        self.fetch_stdout();
        if self.clear_after_exec {
            self.input.clear();
        }
    }
}

impl ToolComponent for LuaTool {
    fn paint_ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Enter)) {
            self.exec_code();
        }
        ui.horizontal(|ui| {
            if ui.button("Info").on_hover_text("Refresh Info").clicked() {
                self.build_info();
            }
            if ui.button("GC").on_hover_text("Run Lua GC").clicked() {
                match self.lua_stack.gc_collect() {
                    Ok(_) => self.build_info(),
                    Err(e) => self.msg = Some(Msg::new(e.to_string(), MsgType::Error)),
                }
                self.build_info();
            }
            if ui.button("Reset").on_hover_text("Reset Lua VM").clicked() {
                self.lua_stack = Lua::new();
                match self.init() {
                    Ok(_) => self.build_info(),
                    Err(e) => self.msg = Some(Msg::new(e, MsgType::Error)),
                }
                self.build_info();
            }
            if ui.button("Run").on_hover_text("Run Lua Code").clicked() {
                self.exec_code();
            }
            ui.checkbox(&mut self.clear_after_exec, "Clear")
                .on_hover_text("Clear input after execution");
            ui.label(self.info.clone());
        });
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            use egui_extras::syntax_highlighting::*;
            let mut layout_job = egui_extras::syntax_highlighting::highlight(
                ctx,
                ui.style(),
                &CodeTheme::from_style(&ctx.style()),
                string,
                "lua",
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };
        ui.separator();
        ui.add(
            egui::TextEdit::multiline(&mut self.input)
                .desired_rows(4)
                .desired_width(f32::INFINITY)
                .code_editor()
                .layouter(&mut layouter),
        );
        ui.separator();
        let width = ui.available_width();
        ui.horizontal(|ui| {
            let mut cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Returns");
                        if ui.button("Copy").clicked() {
                            ui.output_mut(|out| {
                                out.copied_text = self.return_output.clone();
                            });
                        }
                        if ui.button("Clear").clicked() {
                            self.return_output.clear();
                        }
                    });
                    ui.add(
                        egui::TextEdit::multiline(&mut self.return_output.clone())
                            .desired_rows(8)
                            .desired_width(f32::INFINITY),
                    );
                });
            });
            cursor = ui.cursor();
            cursor.set_width(width / 2.0 - 5.0);
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Stdout");
                        if ui.button("Copy").clicked() {
                            ui.output_mut(|out| {
                                out.copied_text = self.std_output.clone();
                            });
                        }
                        if ui.button("Clear").clicked() {
                            self.std_output.clear();
                        }
                    });
                    ui.add(
                        egui::TextEdit::multiline(&mut self.std_output.clone())
                            .desired_rows(8)
                            .desired_width(f32::INFINITY),
                    );
                });
            });
        });
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
}
