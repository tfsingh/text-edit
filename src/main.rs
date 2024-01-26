use eframe::egui;
use eframe::egui::{CentralPanel, ScrollArea, WidgetText};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "notepad",
        options,
        Box::new(|_cc| Box::new(State::default())),
    )
}

struct State {
    buffer: Vec<String>,
    current_line: usize,
    current_column: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            buffer: vec![String::new()],
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
                        .map(|(line_index, line)| {
                            let mut display_line = format!("{} {}", line_index + 1, line);
                            if line_index == self.current_line {
                                if self.current_column + line_index.to_string().len() + 1
                                    < display_line.len()
                                {
                                    display_line.insert(
                                        self.current_column + line_index.to_string().len() + 1,
                                        '|',
                                    );
                                } else {
                                    display_line.push('|');
                                }
                            }
                            display_line
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
                            curr_line.insert_str(self.current_column, &t);
                            self.current_column += t.len();
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
                                    .map_or(0, |line| line.len());

                                if let Some(prev_line) = self.buffer.get_mut(self.current_line) {
                                    prev_line.push_str(&curr_line);
                                }

                                return;
                            }

                            if let Some(curr_line) = self.buffer.get_mut(self.current_line) {
                                self.current_column = self.current_column.saturating_sub(1);
                                curr_line.remove(self.current_column);
                            }
                        }
                        egui::Key::Enter => {
                            if !pressed {
                                return;
                            }

                            let new_line =
                                if let Some(curr_line) = self.buffer.get_mut(self.current_line) {
                                    curr_line.split_off(self.current_column)
                                } else {
                                    String::new()
                                };
                            self.buffer.insert(self.current_line + 1, new_line);

                            self.current_column = 0;
                            self.current_line += 1;
                        }
                        egui::Key::ArrowLeft => {
                            if !pressed {
                                return;
                            }

                            if self.current_column > 0 {
                                self.current_column -= 1;
                            } else if self.current_line != 0 {
                                self.current_line -= 1;
                                self.current_column =
                                    self.buffer.get(self.current_line).unwrap().len();
                            }
                        }
                        egui::Key::ArrowRight => {
                            if !pressed {
                                return;
                            }

                            if self.current_column
                                != self.buffer.get(self.current_line).unwrap().len()
                            {
                                self.current_column += 1;
                            } else if self.current_line != self.buffer.len() - 1 {
                                self.current_line += 1;
                                self.current_column = 0;
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
