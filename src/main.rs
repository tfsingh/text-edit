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
    state: Vec<Rope>,
    current_line: usize,
    current_column: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            state: vec![Rope::new()],
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
                        .state
                        .iter()
                        .map(|rope| rope.to_string())
                        .collect::<Vec<_>>()
                        .join("\n");
                    ui.label(WidgetText::RichText(combined_text.into()));
                });

            let events = ui.input(|i| i.events.clone());

            for event in events {
                match event {
                    egui::Event::Text(t) => {
                        if let Some(curr_line) = self.state.get_mut(self.current_line) {
                            curr_line.append(Rope::from_str(&t));
                            self.current_column += 1;
                        }
                    }
                    egui::Event::Key {
                        key,
                        pressed,
                        modifiers,
                        physical_key,
                        repeat,
                    } => match key {
                        egui::Key::Backspace => {
                            if let Some(curr_line) = self.state.get_mut(self.current_line) {
                                if pressed && curr_line.len_chars() > 0 {
                                    curr_line.remove(self.current_column - 1..self.current_column);
                                    self.current_column -= 1;
                                }
                            }
                        }
                        egui::Key::Enter => {}
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
    }
}
