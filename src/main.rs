use eframe::egui;
use ropey::Rope;

use eframe::egui::{CentralPanel, ScrollArea, WidgetText};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Keyboard events",
        options,
        Box::new(|_cc| Box::new(State::default())),
    )
}

struct State {
    buffer: Vec<Rope>,
    current_line: usize,
    current_column: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            buffer: vec![Rope::new()],
            current_line: 0,
            current_column: 0,
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    let combined_text = self
                        .buffer
                        .iter()
                        .enumerate()
                        .map(|(line_index, rope)| {
                            let mut line = format!("{} {}", line_index + 1, rope.to_string());
                            if line_index == self.current_line {
                                if self.current_column + line_index.to_string().len() + 1
                                    < line.len()
                                {
                                    line.insert(
                                        self.current_column + line_index.to_string().len() + 1,
                                        '|',
                                    );
                                } else {
                                    line.push('|');
                                }
                            }
                            line
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    ui.label(WidgetText::RichText(combined_text.into()));
                });

            let events = ui.input(|i| i.events.clone());

            for event in events {
                match event {
                    egui::Event::Text(t) => {
                        if let Some(curr_line) = self.buffer.get_mut(self.current_line) {
                            curr_line.append(Rope::from_str(&t));
                            self.current_column += 1;
                        }
                    }
                    egui::Event::Key {
                        key,
                        pressed,
                        modifiers: _,
                        physical_key: _,
                        repeat: _,
                    } => match key {
                        egui::Key::Backspace => {
                            if !pressed {
                                return;
                            }

                            if self.current_column == 0 {
                                if self.current_line == 0 {
                                    return;
                                }
                                let curr_line = self.buffer.remove(self.current_line);
                                self.current_line -= 1;

                                self.current_column = self
                                    .buffer
                                    .get(self.current_line)
                                    .map_or(0, |line| line.len_chars());

                                if let Some(prev_line) = self.buffer.get_mut(self.current_line) {
                                    prev_line.append(curr_line);
                                }

                                return;
                            }

                            if let Some(curr_line) = self.buffer.get_mut(self.current_line) {
                                self.current_column -= 1;
                                curr_line.remove(self.current_column..=self.current_column);
                            }
                        }
                        egui::Key::Enter => {
                            if !pressed {
                                return;
                            }
                            self.buffer.insert(self.current_line + 1, Rope::new());
                            self.current_line += 1;
                            self.current_column = 0;
                        }
                        egui::Key::ArrowLeft => {
                            if !pressed {
                                return;
                            }

                            self.current_column = self.current_column.saturating_sub(1);
                        }
                        egui::Key::ArrowRight => {
                            if !pressed {
                                return;
                            }
                            if let Some(curr_line) = self.buffer.get_mut(self.current_line) {
                                self.current_column =
                                    std::cmp::min(self.current_column + 1, curr_line.len_chars())
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
    }
}
