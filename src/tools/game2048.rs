use super::*;
use eframe::egui;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

#[derive(Clone)]
pub struct Game2048 {
    board: [[usize; 4]; 4],
    cheat: bool,
    cheat_add: usize,
    rng: ChaChaRng,
    msg: Option<Msg>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Game2048 {
    pub fn new() -> Self {
        let mut rng = ChaChaRng::from_seed(crate::get_seed());
        let mut board = [[0; 4]; 4];
        let first_1: usize = rng.gen_range(0..16);
        let mut second_1: usize = rng.gen_range(0..15);
        if second_1 >= first_1 {
            second_1 += 1;
        }
        let first_1_index = (first_1 / 4, first_1 % 4);
        let second_1_index = (second_1 / 4, second_1 % 4);
        board[first_1_index.0][first_1_index.1] = 1;
        board[second_1_index.0][second_1_index.1] = 1;
        Self {
            board,
            cheat: false,
            cheat_add: 0,
            rng,
            msg: None,
        }
    }
    fn move_up(&self) -> [[usize; 4]; 4] {
        let mut board_new = [[0; 4]; 4];
        for x in 0..4 {
            let mut filled = Vec::new();
            for y in 0..4 {
                let value = self.board[x][y];
                if value > 0 {
                    if *filled.last().unwrap_or(&0) == value {
                        let last = filled.pop().unwrap();
                        filled.push(last << 1);
                    } else {
                        filled.push(value);
                    }
                }
            }
            for y in 0..4 {
                if let Some(value) = filled.get(y) {
                    board_new[x][y] = *value;
                }
            }
        }
        board_new
    }
    fn move_down(&self) -> [[usize; 4]; 4] {
        let mut board_new = [[0; 4]; 4];
        for x in 0..4 {
            let mut filled = Vec::new();
            for y in (0..4).rev() {
                let value = self.board[x][y];
                if value > 0 {
                    if *filled.last().unwrap_or(&0) == value {
                        let last = filled.pop().unwrap();
                        filled.push(last << 1);
                    } else {
                        filled.push(value);
                    }
                }
            }
            for y in 0..4 {
                if let Some(value) = filled.get(3 - y) {
                    board_new[x][y] = *value;
                }
            }
        }
        board_new
    }
    fn move_left(&self) -> [[usize; 4]; 4] {
        let mut board_new = [[0; 4]; 4];
        for y in 0..4 {
            let mut filled = Vec::new();
            for x in 0..4 {
                let value = self.board[x][y];
                if value > 0 {
                    if *filled.last().unwrap_or(&0) == value {
                        let last = filled.pop().unwrap();
                        filled.push(last << 1);
                    } else {
                        filled.push(value);
                    }
                }
            }
            for x in 0..4 {
                if let Some(value) = filled.get(x) {
                    board_new[x][y] = *value;
                }
            }
        }
        board_new
    }
    fn move_right(&self) -> [[usize; 4]; 4] {
        let mut board_new = [[0; 4]; 4];
        for y in 0..4 {
            let mut filled = Vec::new();
            for x in (0..4).rev() {
                let value = self.board[x][y];
                if value > 0 {
                    if *filled.last().unwrap_or(&0) == value {
                        let last = filled.pop().unwrap();
                        filled.push(last << 1);
                    } else {
                        filled.push(value);
                    }
                }
            }
            for x in 0..4 {
                if let Some(value) = filled.get(3 - x) {
                    board_new[x][y] = *value;
                }
            }
        }
        board_new
    }
    fn add_new(&mut self) {
        let mut emptys = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                if self.board[x][y] == 0 {
                    emptys.push((x, y));
                }
            }
        }
        if emptys.is_empty() {
            return;
        }
        let to_add = self.random_addable();
        let index = self.rng.gen_range(0..emptys.len());
        let (x, y) = emptys[index];
        self.board[x][y] = to_add;
    }
    fn random_addable(&mut self) -> usize {
        let random_num: usize = self.rng.gen();
        random_num & (!random_num + 1)
    }
    fn get_color(&mut self, value: usize) -> egui::Color32 {
        match value {
            1 => egui::Color32::WHITE,
            2 => egui::Color32::LIGHT_BLUE,
            4 => egui::Color32::LIGHT_GREEN,
            8 => egui::Color32::LIGHT_RED,
            16 => egui::Color32::LIGHT_YELLOW,
            32 => egui::Color32::RED,
            64 => egui::Color32::GREEN,
            128 => egui::Color32::YELLOW,
            256 => egui::Color32::KHAKI,
            512 => egui::Color32::GOLD,
            _ => egui::Color32::DEBUG_COLOR,
        }
    }
    fn move_board(&mut self, direct: Direction) {
        let board_up = self.move_up();
        let board_down = self.move_down();
        let board_left = self.move_left();
        let board_right = self.move_right();
        let desire_board = match direct {
            Direction::Up => board_up,
            Direction::Down => board_down,
            Direction::Left => board_left,
            Direction::Right => board_right,
            _ => return,
        };
        if board_up == self.board
            && board_down == self.board
            && board_left == self.board
            && board_right == self.board
        {
            self.msg = Some(Msg::new("Game Over".to_string(), MsgType::Info));
            return;
        }
        if desire_board != self.board {
            self.board = desire_board;
            self.add_new();
        }
    }
}

impl ToolComponent for Game2048 {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Clear").clicked() {
                *self = Self::new();
            }
            ui.checkbox(&mut self.cheat, "Cheat Mode")
                .on_hover_ui(|ui| {
                    if self.cheat {
                        ui.label(
                            egui::RichText::new("Cheat Mode On").color(egui::Color32::LIGHT_RED),
                        );
                        ui.label(
                            egui::RichText::new("Left Click to Remove")
                                .color(egui::Color32::LIGHT_GREEN),
                        );
                        ui.label(
                            egui::RichText::new("Right Click to Replace")
                                .color(egui::Color32::LIGHT_BLUE),
                        );
                    } else {
                        ui.label(egui::RichText::new("Cheat Mode Off").color(egui::Color32::GOLD));
                    }
                });
            if self.cheat {
                ui.label("2 ^");
                ui.add(
                    egui::DragValue::new(&mut self.cheat_add)
                        .speed(1.0)
                        .clamp_range(0..=(usize::BITS / 2)),
                )
                .on_hover_text((1usize << self.cheat_add).to_string());
            }
        });
        let direct = ui.input_mut(|input| {
            let up_pressed = input.key_released(egui::Key::ArrowUp);
            let down_pressed = input.key_released(egui::Key::ArrowDown);
            let left_pressed = input.key_released(egui::Key::ArrowLeft);
            let right_pressed = input.key_released(egui::Key::ArrowRight);
            if up_pressed && !down_pressed && !left_pressed && !right_pressed {
                Direction::Up
            } else if down_pressed && !up_pressed && !left_pressed && !right_pressed {
                Direction::Down
            } else if left_pressed && !up_pressed && !down_pressed && !right_pressed {
                Direction::Left
            } else if right_pressed && !up_pressed && !down_pressed && !left_pressed {
                Direction::Right
            } else {
                Direction::None
            }
        });
        self.move_board(direct);
        // draw board
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            const SIZE: f32 = 64.0;
            let sense = if self.cheat {
                // if cheat mode is on, allow click
                egui::Sense::hover().union(egui::Sense::click())
            } else {
                egui::Sense::hover()
            };
            let (rect, response) = ui.allocate_exact_size(egui::Vec2::splat(SIZE * 4.0), sense);
            let line_stroke = egui::Stroke::new(2f32, egui::Color32::WHITE);
            let painter = ui.painter();
            // paint vertical lines
            for i in 1..4 {
                let i = i as f32;
                painter.vline(
                    rect.left() + i * SIZE,
                    rect.top()..=rect.bottom(),
                    line_stroke,
                );
            }
            // paint horizontal lines
            for i in 1..4 {
                let i = i as f32;
                painter.hline(
                    rect.left()..=rect.right(),
                    rect.top() + i * SIZE,
                    line_stroke,
                );
            }
            for x in 0..4 {
                for y in 0..4 {
                    let value = self.board[x][y];
                    if value > 0 {
                        let color = self.get_color(value);
                        let value = value.to_string();
                        let x = x as f32 + 0.5;
                        let y = y as f32 + 0.5;
                        let pos = egui::Pos2::new(rect.left() + x * SIZE, rect.top() + y * SIZE);

                        painter.text(
                            pos,
                            egui::Align2::CENTER_CENTER,
                            value,
                            egui::FontId::default(),
                            color,
                        );
                    }
                }
            }
            // paint hover
            if response.hovered() {
                if let Some(pos) = response.hover_pos() {
                    painter.circle_filled(pos, 2f32, egui::Color32::RED);
                    let stroke = egui::Stroke::new(1f32, egui::Color32::LIGHT_RED);
                    painter.circle_stroke(pos, 4f32, stroke);
                }
            }
            if response.clicked() {
                if let Some(pos) = response.hover_pos() {
                    let x = ((pos.x - rect.left()) / SIZE).floor() as usize;
                    let y = ((pos.y - rect.top()) / SIZE).floor() as usize;
                    self.board[x][y] = 0;
                }
            }
            if response.secondary_clicked() {
                if let Some(pos) = response.hover_pos() {
                    let x = ((pos.x - rect.left()) / SIZE).floor() as usize;
                    let y = ((pos.y - rect.top()) / SIZE).floor() as usize;
                    self.board[x][y] = 1 << self.cheat_add;
                }
            }
        });
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
}
