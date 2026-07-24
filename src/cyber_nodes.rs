use eframe::egui;
use crate::cyber_ship::CyberLaser;
use std::time::{Duration, Instant};

pub struct CyberNode {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub state: bool,
    pub is_busy: bool,
    pub label: String,
    pub sub_label: String,
    pub last_hit_time: Option<Instant>,
    pub cooldown: Duration,
}

impl CyberNode {
    pub fn new(x: f32, y: f32, label: impl Into<String>, sub_label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            width: 60.0,
            height: 60.0,
            state: false,
            is_busy: false,
            label: label.into(),
            sub_label: sub_label.into(),
            last_hit_time: None,
            cooldown: Duration::from_millis(2000), // Mandatory 2.0s OS transaction cooldown window
        }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let center_x = self.x + self.width / 2.0;
        let center_y = self.y + self.height / 2.0;
        let painter = ui.painter();

        // Node aura ring based on state (Busy -> Gold, Active -> Green, Off -> Red)
        let (ring_color, core_color, status_text) = if self.is_busy {
            (
                egui::Color32::from_rgb(255, 200, 0),
                egui::Color32::from_rgba_unmultiplied(255, 200, 0, 70),
                "[OS BUSY...]",
            )
        } else if self.state {
            (
                egui::Color32::from_rgb(0, 255, 120),
                egui::Color32::from_rgba_unmultiplied(0, 255, 120, 60),
                "[ACTIVE]",
            )
        } else {
            (
                egui::Color32::from_rgb(255, 40, 80),
                egui::Color32::from_rgba_unmultiplied(255, 40, 80, 50),
                "[OFF]",
            )
        };

        // Header label
        painter.text(
            egui::pos2(center_x, self.y - 22.0),
            egui::Align2::CENTER_CENTER,
            &self.label,
            egui::FontId::proportional(12.0),
            egui::Color32::from_rgb(0, 255, 235),
        );

        // Sub label
        painter.text(
            egui::pos2(center_x, self.y - 8.0),
            egui::Align2::CENTER_CENTER,
            &self.sub_label,
            egui::FontId::proportional(10.0),
            egui::Color32::LIGHT_GRAY,
        );

        // Outer Hex/Circle frame
        painter.circle_filled(egui::pos2(center_x, center_y), 26.0, core_color);
        painter.circle_stroke(
            egui::pos2(center_x, center_y),
            25.0,
            egui::Stroke::new(2.0_f32, ring_color),
        );

        // Inner status indicator dot
        painter.circle_filled(
            egui::pos2(center_x, center_y),
            8.0,
            ring_color,
        );

        // Status text below
        painter.text(
            egui::pos2(center_x, self.y + self.height + 12.0),
            egui::Align2::CENTER_CENTER,
            status_text,
            egui::FontId::proportional(11.0),
            ring_color,
        );
    }

    pub fn check_collision(&self, laser: &CyberLaser) -> bool {
        let laser_rect = egui::Rect::from_min_size(
            egui::pos2(laser.x - 3.0, laser.y - 2.0),
            egui::vec2(6.0, 14.0),
        );

        let node_rect = egui::Rect::from_min_size(
            egui::pos2(self.x, self.y),
            egui::vec2(self.width, self.height),
        );

        laser_rect.intersects(node_rect)
    }

    pub fn register_hit(&mut self) {
        self.last_hit_time = Some(Instant::now());
    }

    pub fn can_toggle(&self) -> bool {
        if let Some(last_hit_time) = self.last_hit_time {
            return Instant::now().duration_since(last_hit_time) > self.cooldown;
        }
        true
    }

    pub fn reset_cooldown(&mut self) {
        self.last_hit_time = None;
    }
}
