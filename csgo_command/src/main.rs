use chrono::Local;
use clipboard::{ClipboardContext, ClipboardProvider};
use eframe::egui;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "CSGO Demo Recorder",
        options,
        Box::new(|_cc| Box::new(DemoRecorder::default())),
    )
}

#[derive(Default)]
struct DemoRecorder {
    selected_map: Option<String>,
    selected_mode: Option<String>,
}

impl eframe::App for DemoRecorder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CSGO Demo Recorder");

            ui.horizontal(|ui| {
                ui.label("Select Map:");
                for map in ["Nuke", "Ancient", "Vertigo", "Overpass", "Dust2", "Mirage", "Italy"] {
                    if ui.button(map).clicked() {
                        self.selected_map = Some(map.to_string());
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Select Mode:");
                for mode in ["Wingman", "Premier", "Arms Race"] {
                    if ui.button(mode).clicked() {
                        self.selected_mode = Some(mode.to_string());
                    }
                }
            });

            if ui.button("Generate Command").clicked() {
                if let Some(command) = generate_command(&self.selected_map, &self.selected_mode) {
                    copy_to_clipboard(&command);
                    ui.label(format!("Copied to clipboard: {}", command));
                } else {
                    ui.label("Please select both a map and a mode.");
                }
            }

            if let Some(map) = &self.selected_map {
                ui.label(format!("Selected Map: {}", map));
            }

            if let Some(mode) = &self.selected_mode {
                ui.label(format!("Selected Mode: {}", mode));
            }
        });
    }
}

fn generate_command(map: &Option<String>, mode: &Option<String>) -> Option<String> {
    if let (Some(map), Some(mode)) = (map, mode) {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let demo_name = format!("demo_{}_{}_{}", map.to_lowercase(), mode.to_lowercase(), date);
        Some(format!("record {}", demo_name))
    } else {
        None
    }
}

fn copy_to_clipboard(command: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(command.to_string()).unwrap();
}
