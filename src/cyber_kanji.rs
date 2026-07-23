use eframe::egui;
use rand::Rng;

pub const KANJI_CHARACTERS: &[char] = &['知', '慧', '悟', '愛', '和', '報', '済', '神', '網', '電', '光', '守', '脈', '火', '壁'];

pub struct KanjiDrop {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub size: f32,
    pub chars: Vec<char>,
    pub opacity: f32,
}

pub struct MatrixRain {
    pub drops: Vec<KanjiDrop>,
    pub width: f32,
    pub height: f32,
}

impl MatrixRain {
    pub fn new(width: f32, height: f32) -> Self {
        let mut rng = rand::thread_rng();
        let drop_count = 60;
        let mut drops = Vec::with_capacity(drop_count);

        let initial_width = width.max(300.0);
        let initial_height = height.max(300.0);

        for _ in 0..drop_count {
            drops.push(Self::create_drop(&mut rng, initial_width, initial_height));
        }

        Self {
            drops,
            width: initial_width,
            height: initial_height,
        }
    }

    fn create_drop(rng: &mut impl Rng, width: f32, _height: f32) -> KanjiDrop {
        let x = rng.gen_range(15.0..(width - 15.0).max(30.0));
        let y = rng.gen_range(-600.0..0.0);
        let speed = if rng.gen_bool(0.07) {
            rng.gen_range(0.4..0.8) // Occasional ultra-slow floating fall (~2-3 columns)
        } else {
            rng.gen_range(1.8..4.2) // Standard matrix rain speed
        };
        let size = rng.gen_range(16.0..28.0);
        let opacity = rng.gen_range(0.35..0.85);

        let char_count = rng.gen_range(5..12);
        let mut chars = Vec::with_capacity(char_count);
        for _ in 0..char_count {
            let idx = rng.gen_range(0..KANJI_CHARACTERS.len());
            chars.push(KANJI_CHARACTERS[idx]);
        }

        KanjiDrop {
            x,
            y,
            speed,
            size,
            chars,
            opacity,
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        self.width = width.max(300.0);
        self.height = height.max(300.0);

        let mut rng = rand::thread_rng();
        for drop in &mut self.drops {
            drop.y += drop.speed;

            if drop.y > self.height + 200.0 {
                drop.y = rng.gen_range(-400.0..-50.0);
                drop.x = rng.gen_range(15.0..(self.width - 15.0).max(30.0));
                drop.speed = if rng.gen_bool(0.07) {
                    rng.gen_range(0.4..0.8) // Occasional ultra-slow floating fall (~2-3 columns)
                } else {
                    rng.gen_range(1.8..4.2) // Standard matrix rain speed
                };

                // Mutate characters on respawn
                for ch in &mut drop.chars {
                    if rng.gen_bool(0.3) {
                        let idx = rng.gen_range(0..KANJI_CHARACTERS.len());
                        *ch = KANJI_CHARACTERS[idx];
                    }
                }
            }
        }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        // Use full unclipped canvas painter
        let painter = ui.painter_at(ui.max_rect());

        for drop in &self.drops {
            for (idx, &ch) in drop.chars.iter().enumerate() {
                let char_y = drop.y - (idx as f32 * (drop.size + 4.0));
                if char_y < -50.0 || char_y > self.height + 50.0 {
                    continue;
                }

                // Lead character glows bright cyan/white, tail is neon green
                let color = if idx == 0 {
                    egui::Color32::from_rgba_unmultiplied(180, 255, 240, (255.0 * drop.opacity) as u8)
                } else {
                    let fade = (1.0 - (idx as f32 / drop.chars.len() as f32)).max(0.15);
                    egui::Color32::from_rgba_unmultiplied(0, 240, 120, (255.0 * drop.opacity * fade) as u8)
                };

                painter.text(
                    egui::pos2(drop.x, char_y),
                    egui::Align2::CENTER_CENTER,
                    ch.to_string(),
                    egui::FontId::proportional(drop.size),
                    color,
                );
            }
        }
    }
}
