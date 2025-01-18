use eframe::egui;
use crate::bullets::Bullet;
use std::time::{Duration, Instant};

pub struct Target {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub state: bool,
    pub last_hit_time: Option<Instant>,
    pub cooldown: Duration,
}

impl Target {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: 30.0,  // Make the target skinnier
            height: 60.0,
            state: false,
            last_hit_time: None,
            cooldown: Duration::from_millis(500),  // Adjust the cooldown duration as needed
        }
    }

    pub fn create_targets() -> Vec<Self> {
        vec![
            Self::new(150.0, 100.0),
            Self::new(400.0, 100.0),
            Self::new(650.0, 100.0),
        ]
    }

    pub fn update(&mut self) {
        // No need to reset the state automatically
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        // Draw the main target (light gray)
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.x, self.y),
                egui::vec2(self.width, self.height),
            ),
            0.0,
            egui::Color32::LIGHT_GRAY,
        );

        // Draw the red and green squares based on the state
        let indicator_size = self.width / 2.0;
        if self.state {
            // Draw green square at the top
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(self.x + indicator_size / 2.0, self.y + 10.0),
                    egui::vec2(indicator_size, indicator_size),
                ),
                0.0,
                egui::Color32::GREEN,
            );
        } else {
            // Draw red square at the bottom
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(self.x + indicator_size / 2.0, self.y + self.height - indicator_size - 10.0),
                    egui::vec2(indicator_size, indicator_size),
                ),
                0.0,
                egui::Color32::RED,
            );
        }
    }

    pub fn check_collision(&self, bullet: &Bullet) -> bool {
        let bullet_rect = egui::Rect::from_min_size(
            egui::pos2(bullet.position, bullet.y),
            egui::vec2(5.0, 10.0),
        );

        let target_rect = egui::Rect::from_min_size(
            egui::pos2(self.x, self.y),
            egui::vec2(self.width, self.height),
        );

        bullet_rect.intersects(target_rect)
    }

    pub fn toggle(&mut self) {
        self.state = !self.state;
        self.last_hit_time = Some(Instant::now());
    }

    pub fn can_toggle(&self) -> bool {
        if let Some(last_hit_time) = self.last_hit_time {
            return Instant::now().duration_since(last_hit_time) > self.cooldown;
        }
        true
    }
}
