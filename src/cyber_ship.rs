use eframe::egui;

pub struct CyberLaser {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub color: egui::Color32,
}

impl CyberLaser {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: 1.7,
            color: egui::Color32::from_rgb(0, 255, 235),
        }
    }

    pub fn update(&mut self) {
        self.y -= self.speed;
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        // Outer neon glow
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.x - 3.0, self.y - 2.0),
                egui::vec2(6.0, 14.0),
            ),
            2.0,
            egui::Color32::from_rgba_unmultiplied(0, 255, 255, 100),
        );
        // Inner core
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.x - 1.5, self.y),
                egui::vec2(3.0, 10.0),
            ),
            1.0,
            egui::Color32::WHITE,
        );
    }
}

pub struct CyberShip {
    pub position: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
}

impl CyberShip {
    pub fn new(screen_width: f32) -> Self {
        Self {
            position: screen_width / 2.0 - 30.0,
            speed: 1.7,
            width: 60.0,
            height: 50.0,
        }
    }

    pub fn update(&mut self, input: &egui::InputState, screen_width: f32) {
        if input.key_down(egui::Key::ArrowLeft) || input.key_down(egui::Key::A) {
            self.position -= self.speed;
        }
        if input.key_down(egui::Key::ArrowRight) || input.key_down(egui::Key::D) {
            self.position += self.speed;
        }

        // Dynamically clamp ship within screen width bounds
        let max_x = (screen_width - self.width - 10.0).max(10.0);
        self.position = self.position.clamp(10.0, max_x);
    }

    pub fn shoot(&self, base_y: f32) -> CyberLaser {
        CyberLaser::new(self.position + self.width / 2.0, base_y - 60.0)
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let base_x = self.position;
        let base_y = ui.min_rect().bottom() - self.height - 30.0;
        let center_x = base_x + self.width / 2.0;

        let painter = ui.painter();

        // Neon thruster aura
        painter.circle_filled(
            egui::pos2(center_x, base_y + self.height + 5.0),
            12.0,
            egui::Color32::from_rgba_unmultiplied(0, 200, 255, 80),
        );
        painter.circle_filled(
            egui::pos2(center_x, base_y + self.height + 2.0),
            6.0,
            egui::Color32::from_rgb(0, 255, 200),
        );

        // Cyber fuselage (sharp angular polygon hull)
        let nose = egui::pos2(center_x, base_y);
        let left_wing = egui::pos2(base_x - 15.0, base_y + self.height);
        let left_inner = egui::pos2(base_x + 10.0, base_y + self.height - 10.0);
        let right_inner = egui::pos2(base_x + self.width - 10.0, base_y + self.height - 10.0);
        let right_wing = egui::pos2(base_x + self.width + 15.0, base_y + self.height);

        // Main hull
        painter.add(egui::Shape::convex_polygon(
            vec![nose, right_wing, right_inner, left_inner, left_wing],
            egui::Color32::from_rgb(20, 25, 45),
            egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(0, 255, 235)),
        ));

        // Cockpit canopy (glowing violet)
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(center_x - 6.0, base_y + 15.0),
                egui::vec2(12.0, 16.0),
            ),
            3.0,
            egui::Color32::from_rgb(200, 50, 255),
        );
    }
}
