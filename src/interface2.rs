mod ship;
mod bullets;
mod targets;

use eframe::{egui, epi};
use ship::Ship;
use bullets::Bullet;
use targets::Target;
use std::time::{Duration, Instant};

fn main() {
    eframe::run_native(
        Box::new(PlatformApp::default()),
        eframe::NativeOptions::default(),
    );
}

struct PlatformApp {
    // Adjustable parameters
    top_text: String,
    top_text_color: egui::Color32,
    top_text_size: f32,
    top_text_v_offset: f32,

    bottom_text: String,
    bottom_text_color: egui::Color32,
    bottom_text_size: f32,
    bottom_text_v_offset: f32,

    bullet_v_offset: f32,  // Adjustable parameter for bullet vertical position

    // Christmas tree parameters
    tree_base_x: f32,
    tree_base_y: f32,
    trunk_height: f32,
    trunk_width: f32,

    // Star and squares parameters
    star_x: f32,
    star_y: f32,
    star_size: f32,
    square_offset: f32,

    // Game elements
    ship: Ship,
    bullets: Vec<Bullet>,
    targets: Vec<Target>,
    shooting: bool, // Track if the ship is currently shooting

    // Blinking state
    blinking: bool,
    blink_timer: Instant,
    blink_interval: Duration,

    // Text states for switches
    show_merry: bool,
    show_christmas: bool,

    // Blinking colors
    blinking_colors: Vec<egui::Color32>,
    current_color_index: usize,
}

impl Default for PlatformApp {
    fn default() -> Self {
        let screen_width = 800.0;  // Adjust the screen width as needed
        Self {
            top_text: "POWERED BY SPLIT2OPS SOFTWARE".to_string(),
            top_text_color: egui::Color32::WHITE,
            top_text_size: 20.0,
            top_text_v_offset: 10.0,

            bottom_text: "XALLFIREWALL".to_string(),
            bottom_text_color: egui::Color32::GREEN,
            bottom_text_size: 120.0,
            bottom_text_v_offset: 10.0,

            bullet_v_offset: 80.0,  // Set default vertical offset for bullets

            // Initialize Christmas tree parameters
            tree_base_x: 390.0,
            tree_base_y: 245.0,
            trunk_height: 100.0,
            trunk_width: 40.0,

            // Initialize star and squares parameters
            star_x: 390.0,
            star_y: 190.0,
            star_size: 20.0,
            square_offset: 24.0,

            // Initialize game elements
            ship: Ship::new(screen_width),
            bullets: Vec::new(),
            targets: vec![
                Target::new(150.0, 100.0),
                Target::new(400.0, 100.0),
                Target::new(650.0, 100.0),
            ],
            shooting: false, // Initialize shooting as false

            // Initialize blinking state
            blinking: false,
            blink_timer: Instant::now(),
            blink_interval: Duration::from_millis(250),

            // Initialize text states for switches
            show_merry: false,
            show_christmas: false,

            // Initialize blinking colors
            blinking_colors: vec![egui::Color32::from_rgb(255, 0, 0), egui::Color32::from_rgb(255, 255, 255), egui::Color32::from_rgb(0, 0, 255)],
            current_color_index: 0,
        }
    }
}

impl epi::App for PlatformApp {
    fn name(&self) -> &str {
        "Platform"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        // Set the visuals to have a black background
        let visuals = egui::Visuals {
            dark_mode: true,
            ..Default::default()
        };
        ctx.set_visuals(visuals);

        let input = ctx.input().clone(); // Clone the input to avoid conflicts

        // Handle movement independently
        self.ship.update(&input);

        // Get the bottom coordinate from the UI context
        let bottom = ctx.available_rect().bottom();

        // Check for shooting key independently
        if input.key_pressed(egui::Key::Space) {
            self.shooting = true;
        } else if input.key_released(egui::Key::Space) {
            self.shooting = false;
        }

        if self.shooting {
            let fired_bullet = self.ship.shoot(bottom, self.bullet_v_offset);
            self.bullets.push(fired_bullet);
        }

        // Create a central panel with customized labels and game elements
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render Christmas tree
            self.render_christmas_tree(ui);

            // Add space at the top for the top text
            ui.vertical_centered_justified(|ui| {
                ui.add_space(self.top_text_v_offset);
                ui.label(egui::RichText::new(&self.top_text).color(self.top_text_color).size(self.top_text_size));
            });

            // Add space at the bottom for the bottom text
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(self.bottom_text_v_offset);
                ui.label(egui::RichText::new(&self.bottom_text).color(self.bottom_text_color).size(self.bottom_text_size));
            });

            // Fill the rest of the space with black background
            ui.add_space(ui.available_height());

            // Update and render bullets
            for bullet in &mut self.bullets {
                bullet.update();
                bullet.render(ui);
            }

            // Update and render targets
            for (i, target) in self.targets.iter_mut().enumerate() {
                target.update();
                target.render(ui);

                // Check if the first or third target is hit to show text
                if i == 0 && target.state {
                    self.show_merry = true;
                }
                if i == 1 && target.state {
                    self.blinking = true;
                }
                if i == 2 && target.state {
                    self.show_christmas = true;
                }
            }

            // Render ship
            self.ship.render(ui);

            // Check for bullet-target collisions
            for bullet in &self.bullets {
                for target in &mut self.targets {
                    if target.check_collision(bullet) && target.can_toggle() {
                        target.toggle();
                    }
                }
            }

            // Render Merry and Christmas texts based on switch states
            if self.show_merry {
                ui.painter().text(
                    egui::pos2(self.tree_base_x - 250.0, self.tree_base_y),
                    egui::Align2::LEFT_CENTER,
                    "Merry",
                    egui::FontId::proportional(80.0),
                    egui::Color32::RED,
                );
            }

            if self.show_christmas {
                ui.painter().text(
                    egui::pos2(self.tree_base_x + 350.0, self.tree_base_y),
                    egui::Align2::RIGHT_CENTER,
                    "Christmas",
                    egui::FontId::proportional(80.0),
                    egui::Color32::RED,
                );
            }
        });

        // Control FPS by limiting frame rate
        ctx.request_repaint();

        // Update blinking state and colors
        if self.blinking {
            if self.blink_timer.elapsed() >= self.blink_interval {
                self.blink_timer = Instant::now();
                self.current_color_index = (self.current_color_index + 1) % self.blinking_colors.len();
            }
        }
    }
}

impl PlatformApp {
    fn render_christmas_tree(&self, ui: &mut egui::Ui) {
        let tree_base_color = egui::Color32::from_rgb(34, 139, 34); // Forest Green
        let base_decoration_color = egui::Color32::from_rgb(255, 0, 0); // Red as the default decoration color

        // Draw tree layers
        for i in (0..5).rev() {
            let width = 200.0 - 30.0 * i as f32;
            let height = 40.0;
            let y = self.tree_base_y + (4 - i) as f32 * height;
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(self.tree_base_x - width / 2.0, y),
                    egui::vec2(width, height),
                ),
                0.0,
                tree_base_color,
            );

            // Draw decorations
            for j in 0..4 {
                let decoration_color = if self.blinking {
                    self.blinking_colors[self.current_color_index]
                } else {
                    base_decoration_color
                };
                let x = self.tree_base_x - width / 2.0 + j as f32 * (width / 3.0);
                ui.painter().circle_filled(
                    egui::pos2(x, y + height / 2.0),
                    5.0,
                    decoration_color,
                );
            }
        }

        // Draw the tree trunk
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.tree_base_x - self.trunk_width / 2.0, self.tree_base_y + 200.0),
                egui::vec2(self.trunk_width, self.trunk_height),
            ),
            0.0,
            egui::Color32::from_rgb(139, 69, 19), // Saddle Brown
        );

        // Draw the star on top
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.star_x - self.star_size / 2.0, self.star_y - self.star_size / 2.0),
                egui::vec2(self.star_size, self.star_size),
            ),
            0.0,
            egui::Color32::WHITE, // White star
        );

        // Add squares around the star
        let star_coords = [
            (self.star_x - self.square_offset, self.star_y - self.square_offset),
            (self.star_x + self.square_offset, self.star_y - self.square_offset),
            (self.star_x - self.square_offset, self.star_y + self.square_offset),
            (self.star_x + self.square_offset, self.star_y + self.square_offset),
        ];
        for &(x, y) in &star_coords {
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(x, y),
                    egui::vec2(10.0, 10.0),
                ),
                0.0,
                egui::Color32::WHITE, // White squares around the star
            );
        }
    }
}
