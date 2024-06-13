use super::*;
use chrono::Timelike;
use eframe::egui;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct TimeTool {
    date: chrono::NaiveDate,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
    microsecond: u32,
    nanosecond: u32,
    timestamp: i64,
    millisecond_timestamp: i64,
    microsecond_timestamp: i64,
    nanosecond_timestamp: i64,
    msg: Option<Msg>,
}

impl TimeTool {
    pub fn new() -> Self {
        let now = chrono::Local::now().to_utc();
        let date = now.date_naive();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        let millisecond = now.timestamp_subsec_millis();
        let microsecond = now.timestamp_subsec_micros() % 1000;
        let nanosecond = now.timestamp_subsec_nanos() % 1000_000 % 1000;
        let timestamp = now.timestamp();
        let millisecond_timestamp = now.timestamp_millis();
        let microsecond_timestamp = now.timestamp_micros();
        let (nanosecond_timestamp, msg) = match now.timestamp_nanos_opt() {
            Some(nanos) => (nanos, None),
            None => (
                0,
                Some(Msg::new(
                    "Error: timestamp_nanos_opt() failed".into(),
                    MsgType::Error,
                )),
            ),
        };
        Self {
            date,
            hour,
            minute,
            second,
            millisecond,
            microsecond,
            nanosecond,
            timestamp,
            millisecond_timestamp,
            microsecond_timestamp,
            nanosecond_timestamp,
            msg,
        }
    }
}

impl ToolComponent for TimeTool {
    fn paint_ui(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        self.hour = self.hour.min(23);
        self.minute = self.minute.min(59);
        self.second = self.second.min(59);
        self.millisecond = self.millisecond.min(999);
        self.microsecond = self.microsecond.min(999);
        self.nanosecond = self.nanosecond.min(999);
        let mut datetime_tochange = None;
        if ui
            .horizontal(|ui| {
                let date_change = ui
                    .add(egui_extras::DatePickerButton::new(&mut self.date))
                    .changed();
                let hour_change = ui
                    .add(egui::DragValue::new(&mut self.hour).clamp_range(0..=23))
                    .changed();
                ui.label(":");
                let minute_change = ui
                    .add(egui::DragValue::new(&mut self.minute).clamp_range(0..=59))
                    .changed();
                ui.label(":");
                let second_change = ui
                    .add(egui::DragValue::new(&mut self.second).clamp_range(0..=59))
                    .changed();
                ui.label(".");
                let millisecond_change = ui
                    .add(egui::DragValue::new(&mut self.millisecond).clamp_range(0..=999))
                    .changed();
                ui.label(".");
                let microsecond_change = ui
                    .add(egui::DragValue::new(&mut self.microsecond).clamp_range(0..=999))
                    .changed();
                ui.label(".");
                let nanosecond_change = ui
                    .add(egui::DragValue::new(&mut self.nanosecond).clamp_range(0..=999))
                    .changed();
                if ui.button("Now").clicked() {
                    datetime_tochange = Some(chrono::Local::now().to_utc());
                }
                date_change
                    || hour_change
                    || minute_change
                    || second_change
                    || millisecond_change
                    || microsecond_change
                    || nanosecond_change
            })
            .inner
        {
            // Update timestamp
            let total_nano =
                self.nanosecond + self.microsecond * 1000 + self.millisecond * 1000 * 1000;
            let time = self
                .date
                .and_hms_nano_opt(self.hour, self.minute, self.second, total_nano);
            match time {
                Some(time) => {
                    datetime_tochange = Some(time.and_utc());
                }
                None => {
                    self.msg = Some(Msg::new("Error: Invalid time".into(), MsgType::Error));
                }
            }
        }
        if ui
            .horizontal(|ui| {
                ui.label("Timestamp: ");
                let change = ui
                    .add(
                        egui::DragValue::new(&mut self.timestamp)
                            .clamp_range(0..=i64::MAX)
                            .speed(1.0),
                    )
                    .changed();
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = self.timestamp.to_string();
                    });
                }
                change
            })
            .inner
        {
            let time = chrono::DateTime::from_timestamp(self.timestamp, 0);
            match time {
                Some(time) => {
                    datetime_tochange = Some(time);
                }
                None => {
                    self.msg = Some(Msg::new("Error: Invalid timestamp".into(), MsgType::Error));
                }
            }
        }
        if ui
            .horizontal(|ui| {
                ui.label("Millisecond timestamp: ");
                let change = ui
                    .add(
                        egui::DragValue::new(&mut self.millisecond_timestamp)
                            .clamp_range(0..=i64::MAX)
                            .speed(10.0),
                    )
                    .changed();
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = self.millisecond_timestamp.to_string();
                    });
                }
                change
            })
            .inner
        {
            let time = chrono::DateTime::from_timestamp_millis(self.millisecond_timestamp);
            match time {
                Some(time) => {
                    datetime_tochange = Some(time);
                }
                None => {
                    self.msg = Some(Msg::new(
                        "Error: Invalid millisecond timestamp".into(),
                        MsgType::Error,
                    ));
                }
            }
        }
        if ui
            .horizontal(|ui| {
                ui.label("Microsecond timestamp: ");
                let change = ui
                    .add(
                        egui::DragValue::new(&mut self.microsecond_timestamp)
                            .clamp_range(0..=i64::MAX)
                            .speed(100.0),
                    )
                    .changed();
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = self.microsecond_timestamp.to_string();
                    });
                }
                change
            })
            .inner
        {
            let time = chrono::DateTime::from_timestamp_micros(self.microsecond_timestamp);
            match time {
                Some(time) => {
                    datetime_tochange = Some(time);
                }
                None => {
                    self.msg = Some(Msg::new(
                        "Error: Invalid microsecond timestamp".into(),
                        MsgType::Error,
                    ));
                }
            }
        }
        if ui
            .horizontal(|ui| {
                ui.label("Nanosecond timestamp: ");
                let change = ui
                    .add(
                        egui::DragValue::new(&mut self.nanosecond_timestamp)
                            .clamp_range(0..=i64::MAX)
                            .speed(1000.0),
                    )
                    .changed();
                if ui.button("Copy").clicked() {
                    ui.output_mut(|output| {
                        output.copied_text = self.nanosecond_timestamp.to_string();
                    });
                }
                change
            })
            .inner
        {
            datetime_tochange = Some(chrono::DateTime::from_timestamp_nanos(self.nanosecond_timestamp));
        }
        if let Some(ts) = datetime_tochange {
            self.date = ts.date_naive();
            self.hour = ts.hour();
            self.minute = ts.minute();
            self.second = ts.second();
            self.millisecond = ts.timestamp_subsec_millis();
            self.microsecond = ts.timestamp_subsec_micros() % 1000;
            self.nanosecond = ts.timestamp_subsec_nanos() % 1000_000 % 1000;
            self.timestamp = ts.timestamp();
            self.millisecond_timestamp = ts.timestamp_millis();
            self.microsecond_timestamp = ts.timestamp_micros();
            match ts.timestamp_nanos_opt() {
                Some(nanos) => {
                    self.nanosecond_timestamp = nanos;
                }
                None => {
                    self.msg = Some(Msg::new(
                        "Error: timestamp_nanos_opt() failed".into(),
                        MsgType::Error,
                    ));
                }
            }
        }
    }
    fn get_msg(&mut self) -> Option<Msg> {
        self.msg.take()
    }
}
