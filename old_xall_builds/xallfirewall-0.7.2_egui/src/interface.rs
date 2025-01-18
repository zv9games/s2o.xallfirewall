use crossbeam_channel::Sender;
use eframe::{egui, epi};
use std::time::Duration;

pub fn create_output_window(transition_sender: Sender<()>) {
    println!("Debug: Creating output window...");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(300.0, 300.0)),
        initial_window_pos: Some(egui::Pos2::new(100.0, 100.0)),
        ..Default::default()
    };

    println!("Debug: About to run native...");
    eframe::run_native(
        Box::new(OutputApp::new(transition_sender)),
        options,
    );
}

pub fn transform_to_platform_window() {
    // Update the window content and resize it
    println!("Updating window to platform content...");

    eframe::run_native(
        Box::new(PlatformApp::new()),
        eframe::NativeOptions {
            initial_window_size: Some(egui::Vec2::new(800.0, 600.0)),
            initial_window_pos: Some(egui::Pos2::new(100.0, 100.0)),
            ..Default::default()
        }
    );
}

pub struct OutputApp {
    bg_color: egui::Color32,
    messages: Vec<String>,
    transition_sender: Option<Sender<()>>,
}

impl OutputApp {
    pub fn new(transition_sender: Sender<()>) -> Self {
        Self {
            bg_color: egui::Color32::from_rgb(0, 0, 0),
            messages: vec![
                "Starting XALLFIREWALL...".to_string(),
                "Debug: Initializing library...".to_string(),
                "Debug: Library initialized.".to_string(),
                "Debug: Initializing modules...".to_string(),
                "Core processing logic initialized.".to_string(),
                "Sent: interface1 module loaded.".to_string(),
                "Debug: Modules initialized.".to_string(),
                "Debug: Starting output window...".to_string(),
                "Debug: Creating output window...".to_string(),
                "Debug: About to run native...".to_string(),
            ],
            transition_sender: Some(transition_sender),
        }
    }

    fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }
}

impl epi::App for OutputApp {
    fn setup(&mut self, _ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        // Adding dynamic initialization messages
        self.add_message("Initialization complete. Launching platform...".to_string());

        // Automatically signal completion after 4 seconds
        let sender = self.transition_sender.take().unwrap();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(4));
            println!("Output window signaling completion...");
            sender.send(()).expect("Failed to send transition signal");
        });
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().frame(egui::Frame::default().fill(self.bg_color)).show(ctx, |ui| {
            for msg in &self.messages {
                ui.label(egui::RichText::new(msg).color(egui::Color32::WHITE));
            }
        });

        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Output Window"
    }
}

pub struct PlatformApp;

impl PlatformApp {
    pub fn new() -> Self {
        Self
    }
}

impl epi::App for PlatformApp {
    fn setup(&mut self, _ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        // Setup logic for platform window
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to the platform window!");
        });

        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Platform Window"
    }
}
