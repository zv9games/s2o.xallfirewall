extern crate network_wireframe;

use std::error::Error;
use eframe::{epi, egui};

struct MyApp;

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Network Packet Data");

            if ui.button("Start Packet Capture").clicked() {
                match network_wireframe::capture_network_packets() {
                    Ok(_) => ui.label("Packet capture started successfully."),
                    Err(e) => ui.label(format!("Failed to start packet capture: {}", e)),
                }
            }
        });
    }

    fn name(&self) -> &str {
        "Network Packet GUI"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = MyApp;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
    Ok(())
}
