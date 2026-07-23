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

#[derive(PartialEq, Debug, Clone, Copy)]
enum MenuLevel {
    Level1Firewall,
    Level2Defender,
    Level3Telemetry,
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

    bullet_v_offset: f32,

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
    shooting: bool,

    // Menu level & status feedback
    current_level: MenuLevel,
    status_message: String,

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

impl PlatformApp {
    fn load_targets_for_level(level: MenuLevel) -> Vec<Target> {
        match level {
            MenuLevel::Level1Firewall => vec![
                Target::new(150.0, 100.0, "MASTER FW"),
                Target::new(400.0, 100.0, "SHIELD MODE"),
                Target::new(650.0, 100.0, "NEXT LVL >>"),
            ],
            MenuLevel::Level2Defender => vec![
                Target::new(150.0, 100.0, "QUICK SCAN"),
                Target::new(400.0, 100.0, "UPDATE DEFS"),
                Target::new(650.0, 100.0, "NEXT LVL >>"),
            ],
            MenuLevel::Level3Telemetry => vec![
                Target::new(150.0, 100.0, "ADAPTERS"),
                Target::new(400.0, 100.0, "SOCKETS"),
                Target::new(650.0, 100.0, "LVL 1 ↺"),
            ],
        }
    }
}

impl Default for PlatformApp {
    fn default() -> Self {
        let screen_width = 800.0;
        Self {
            top_text: "LEVEL 1: FIREWALL MASTER CONTROL".to_string(),
            top_text_color: egui::Color32::WHITE,
            top_text_size: 22.0,
            top_text_v_offset: 10.0,

            bottom_text: "XALLFIREWALL ARCADE".to_string(),
            bottom_text_color: egui::Color32::GREEN,
            bottom_text_size: 60.0,
            bottom_text_v_offset: 10.0,

            bullet_v_offset: 80.0,

            tree_base_x: 390.0,
            tree_base_y: 245.0,
            trunk_height: 100.0,
            trunk_width: 40.0,

            star_x: 390.0,
            star_y: 190.0,
            star_size: 20.0,
            square_offset: 24.0,

            ship: Ship::new(screen_width),
            bullets: Vec::new(),
            targets: Self::load_targets_for_level(MenuLevel::Level1Firewall),
            shooting: false,

            current_level: MenuLevel::Level1Firewall,
            status_message: "Use Left/Right Arrows to move, Space to shoot target switches!".to_string(),

            blinking: false,
            blink_timer: Instant::now(),
            blink_interval: Duration::from_millis(250),

            show_merry: false,
            show_christmas: false,

            blinking_colors: vec![
                egui::Color32::from_rgb(255, 0, 0),
                egui::Color32::from_rgb(255, 255, 255),
                egui::Color32::from_rgb(0, 0, 255),
            ],
            current_color_index: 0,
        }
    }
}

impl epi::App for PlatformApp {
    fn name(&self) -> &str {
        "XAllFirewall Arcade Platform"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let visuals = egui::Visuals {
            dark_mode: true,
            ..Default::default()
        };
        ctx.set_visuals(visuals);

        let input = ctx.input().clone();
        self.ship.update(&input);

        let bottom = ctx.available_rect().bottom();

        if input.key_pressed(egui::Key::Space) {
            let bullet = self.ship.shoot(bottom, self.bullet_v_offset);
            self.bullets.push(bullet);
            self.shooting = true;
        } else {
            self.shooting = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_christmas_tree(ui);

            ui.vertical_centered_justified(|ui| {
                ui.add_space(self.top_text_v_offset);
                ui.label(egui::RichText::new(&self.top_text).color(self.top_text_color).size(self.top_text_size));
                ui.label(egui::RichText::new(&self.status_message).color(egui::Color32::YELLOW).size(14.0));
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(self.bottom_text_v_offset);
                ui.label(egui::RichText::new(&self.bottom_text).color(self.bottom_text_color).size(self.bottom_text_size));
            });

            ui.add_space(ui.available_height());

            for bullet in &mut self.bullets {
                bullet.update();
                bullet.render(ui);
            }

            for target in self.targets.iter_mut() {
                target.update();
                target.render(ui);
            }

            self.ship.render(ui);

            let mut level_changed = false;

            for bullet in &self.bullets {
                for (i, target) in self.targets.iter_mut().enumerate() {
                    if target.check_collision(bullet) && target.can_toggle() {
                        target.toggle();
                        match (self.current_level, i) {
                            // LEVEL 1: FIREWALL CONTROL
                            (MenuLevel::Level1Firewall, 0) => {
                                if target.state {
                                    let _ = s2o_net_lib::firewall::FirewallController::enable_firewall();
                                    self.status_message = "Action: Enabled Windows Firewall".to_string();
                                } else {
                                    let _ = s2o_net_lib::firewall::FirewallController::disable_firewall();
                                    self.status_message = "Action: Disabled Windows Firewall".to_string();
                                }
                            }
                            (MenuLevel::Level1Firewall, 1) => {
                                if target.state {
                                    let _ = s2o_net_lib::firewall::FirewallController::airplane_mode_enable();
                                    self.status_message = "Action: Enabled Shield Mode (Outbound Blocked)".to_string();
                                } else {
                                    let _ = s2o_net_lib::firewall::FirewallController::airplane_mode_disable();
                                    self.status_message = "Action: Disabled Shield Mode".to_string();
                                }
                            }
                            (MenuLevel::Level1Firewall, 2) => {
                                self.current_level = MenuLevel::Level2Defender;
                                self.top_text = "LEVEL 2: WINDOWS DEFENDER CONTROL".to_string();
                                self.status_message = "Advanced to Level 2: Windows Defender".to_string();
                                level_changed = true;
                            }

                            // LEVEL 2: DEFENDER CONTROL
                            (MenuLevel::Level2Defender, 0) => {
                                std::thread::spawn(|| {
                                    let _ = s2o_net_lib::defender::DefenderController::run_scan_native();
                                });
                                self.status_message = "Action: Triggered Defender Quick Scan".to_string();
                            }
                            (MenuLevel::Level2Defender, 1) => {
                                std::thread::spawn(|| {
                                    let _ = s2o_net_lib::defender::DefenderController::update_defender_native();
                                });
                                self.status_message = "Action: Triggered Defender Signature Update".to_string();
                            }
                            (MenuLevel::Level2Defender, 2) => {
                                self.current_level = MenuLevel::Level3Telemetry;
                                self.top_text = "LEVEL 3: ADAPTERS & TELEMETRY".to_string();
                                self.status_message = "Advanced to Level 3: Telemetry & Adapters".to_string();
                                level_changed = true;
                            }

                            // LEVEL 3: TELEMETRY & ADAPTERS
                            (MenuLevel::Level3Telemetry, 0) => {
                                s2o_net_lib::util2::show_active_adapters();
                                self.status_message = "Action: Listed Active Adapters in Console".to_string();
                            }
                            (MenuLevel::Level3Telemetry, 1) => {
                                let conns = s2o_net_lib::telemetry::get_active_tcp_connections();
                                self.status_message = format!("Action: Scanned {} Active TCP Sockets", conns.len());
                            }
                            (MenuLevel::Level3Telemetry, 2) => {
                                self.current_level = MenuLevel::Level1Firewall;
                                self.top_text = "LEVEL 1: FIREWALL MASTER CONTROL".to_string();
                                self.status_message = "Returned to Level 1: Firewall Control".to_string();
                                level_changed = true;
                            }
                            _ => {}
                        }
                    }
                }
            }

            if level_changed {
                self.targets = Self::load_targets_for_level(self.current_level);
            }
        });

        ctx.request_repaint();

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
        let tree_base_color = egui::Color32::from_rgb(34, 139, 34);
        let base_decoration_color = egui::Color32::from_rgb(255, 0, 0);

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

        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.tree_base_x - self.trunk_width / 2.0, self.tree_base_y + 200.0),
                egui::vec2(self.trunk_width, self.trunk_height),
            ),
            0.0,
            egui::Color32::from_rgb(139, 69, 19),
        );

        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(self.star_x - self.star_size / 2.0, self.star_y - self.star_size / 2.0),
                egui::vec2(self.star_size, self.star_size),
            ),
            0.0,
            egui::Color32::WHITE,
        );

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
                egui::Color32::WHITE,
            );
        }
    }
}
