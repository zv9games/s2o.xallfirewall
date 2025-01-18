use eframe::egui;

pub struct Bullet {
    pub position: f32,
    pub y: f32,
    pub speed: f32,
    pub color: egui::Color32,  // Add a color field to the Bullet struct
}

impl Bullet {
    pub fn new(start_position: f32, start_y: f32, color: egui::Color32) -> Self {
        Self {
            position: start_position,
            y: start_y,
            speed: 10.0,
            color,  // Initialize the color
        }
    }

    pub fn update(&mut self) {
        self.y -= self.speed;
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.position, self.y),
                egui::vec2(5.0, 10.0),
            ),
            0.0,
            self.color,  // Use the bullet's color
        );
    }
}
