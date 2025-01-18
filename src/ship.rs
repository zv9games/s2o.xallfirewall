use eframe::egui;
use crate::bullets::Bullet;

pub struct Ship {
    position: f32,
    speed: f32,
    width: f32,
    height: f32,
}

impl Ship {
    pub fn new(screen_width: f32) -> Self {
        Self {
            position: screen_width / 2.0 - 25.0,  // Center the ship horizontally
            speed: 15.0,
            width: 50.0,
            height: 60.0,
        }
    }

    pub fn update(&mut self, input: &egui::InputState) {
        if input.key_down(egui::Key::ArrowLeft) {
            self.position -= self.speed;
        }
        if input.key_down(egui::Key::ArrowRight) {
            self.position += self.speed;
        }
    }

    pub fn shoot(&self, base_y: f32, bullet_v_offset: f32) -> Bullet {
        Bullet::new(
            self.position + self.width / 2.0 - 2.5,  // Center the bullet horizontally
            base_y - bullet_v_offset,  // Adjust entry position based on vertical offset
            egui::Color32::RED,  // Set the bullet color to red
        )
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let base_x = self.position;
        let base_y = ui.min_rect().bottom() - self.height;

        // Draw the main body of the ship
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(base_x, base_y),
                egui::vec2(self.width, self.height),
            ),
            0.0,
            egui::Color32::from_rgb(128, 0, 128),  // Purple main body
        );

        // Draw the left wing
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(base_x - 20.0, base_y + 30.0),
                egui::vec2(20.0, 10.0),
            ),
            0.0,
            egui::Color32::from_rgb(0, 255, 0),  // Green left wing
        );

        // Draw the right wing
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(base_x + self.width, base_y + 30.0),
                egui::vec2(20.0, 10.0),
            ),
            0.0,
            egui::Color32::from_rgb(0, 255, 0),  // Green right wing
        );

        // Draw the tail fin
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(base_x + 15.0, base_y - 20.0),
                egui::vec2(20.0, 20.0),
            ),
            0.0,
            egui::Color32::from_rgb(255, 0, 0),  // Red tail fin
        );

        // Draw some alien-like eyes
        ui.painter().circle_filled(
            egui::pos2(base_x + 20.0, base_y + 10.0),
            5.0,
            egui::Color32::from_rgb(255, 255, 0),  // Yellow left eye
        );

        ui.painter().circle_filled(
            egui::pos2(base_x + 30.0, base_y + 10.0),
            5.0,
            egui::Color32::from_rgb(255, 255, 0),  // Yellow right eye
        );
    }
}
