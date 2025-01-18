use eframe::{egui, epi};
use std::time::{Instant, Duration};

fn main() {
    // Set the window options
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(300.0, 300.0)),
        ..Default::default()
    };

    // Run the app with the specified options
    eframe::run_native(
        Box::new(LoaderApp::default()),
        options,
    );
}

struct LoaderApp {
    start_time: Instant,
    progress: f32,
    load_duration: f32,       // Adjustable: duration for the loading process
    text_size: f32,           // Adjustable: size of the "LOADING" text
    text_offset: f32,         // Adjustable: vertical offset of the "LOADING" text
    bar_height: f32,          // Adjustable: height of the loading bar
    bar_offset: f32,          // Adjustable: vertical offset of the loading bar
    text_color: egui::Color32,// Adjustable: color of the "LOADING" text
    bar_color: egui::Color32, // Adjustable: color of the loading bar
    increment_speed: f32,     // Adjustable: speed of the loading bar increments
    refresh_rate: Duration,   // Adjustable: refresh rate of the loading screen
    last_update: Instant,     // Time of the last update
}

impl Default for LoaderApp {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            progress: 0.0,
            load_duration: 5.0,      // Default: complete loading in 5 seconds
            text_size: 32.0,         // Default: size of the "LOADING" text
            text_offset: 80.0,       // Default: vertical offset of the "LOADING" text from the center
            bar_height: 40.0,        // Default: height of the loading bar
            bar_offset: 80.0,        // Default: vertical offset of the loading bar from the bottom
            text_color: egui::Color32::GREEN, // Default: green "LOADING" text
            bar_color: egui::Color32::GREEN,  // Default: green loading bar
            increment_speed: 7.1,    // Default: speed of loading bar increments
            refresh_rate: Duration::from_millis(16), // Default: ~60 frames per second
            last_update: Instant::now(),
        }
    }
}

impl epi::App for LoaderApp {
    fn name(&self) -> &str {
        "Loader"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        // Set the visuals to have a black background
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(self.text_color);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::BLACK; // Set the background to black
        ctx.set_visuals(visuals);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // Update the progress based on the elapsed time
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.refresh_rate {
            self.last_update = now;

            let elapsed = self.start_time.elapsed().as_secs_f32();
            self.progress = (elapsed / self.load_duration).min(1.0); // Adjust loading duration

            // Central panel with text
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(self.text_offset); // Adjust text position vertically
                    ui.label(egui::RichText::new("LOADING").color(self.text_color).size(self.text_size));
                });
            });

            // Bottom panel with moving loading bar
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                ui.add_space(self.bar_offset); // Adjust loading bar position vertically

                // Draw the moving loading bar
                let desired_width = ui.available_width() * self.progress;
                let total_width = ui.available_width();
                let step_width = self.increment_speed; // Width of each moving step

                for i in 0..(total_width / step_width).ceil() as i32 {
                    let x_position = i as f32 * step_width;
                    if x_position < desired_width {
                        let rect = egui::Rect::from_min_size(
                            ui.min_rect().min + egui::Vec2::new(x_position, 0.0),
                            egui::vec2(step_width - 1.0, self.bar_height),
                        );
                        ui.painter().rect_filled(rect, 0.0, self.bar_color);
                    }
                }
            });

            // Request a new frame
            frame.request_repaint();
        }

        // Check if loading is complete and close the app
        if self.progress >= 1.0 {
            frame.quit();
        }
    }
}
