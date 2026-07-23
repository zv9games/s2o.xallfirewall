mod network_wireframe;

use std::error::Error;
use eframe::{epi, egui};

struct MyApp {
    connections: Vec<s2o_net_lib::telemetry::ActiveConnection>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            connections: s2o_net_lib::telemetry::get_active_tcp_connections(),
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Network Active Transactions (Driverless WFP / IP Helper)");

            if ui.button("Refresh Live Transactions").clicked() {
                self.connections = s2o_net_lib::telemetry::get_active_tcp_connections();
            }

            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(format!("Total Active Sockets: {}", self.connections.len()));
                ui.separator();

                for conn in &self.connections {
                    ui.horizontal(|ui| {
                        ui.label(format!("PID {:<6}", conn.pid));
                        ui.label(format!("{}:{} -> {}:{}", conn.local_addr, conn.local_port, conn.remote_addr, conn.remote_port));
                        ui.label(format!("[{}]", conn.state));
                    });
                }
            });
        });

        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Network Active Transactions"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
