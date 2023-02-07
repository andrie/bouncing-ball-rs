use egui::*;

mod bouncing_ball;

// const DARK_BLUE:Color32 = Color32::DARK_BLUE;
// const DARK_RED:Color32 = Color32::DARK_RED;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    ball: bouncing_ball::Ball,
    animation: bouncing_ball::AnimationState,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            ball: bouncing_ball::Ball::new(),
            animation: bouncing_ball::AnimationState::Paused,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { ball, animation } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Animation control");

            let button_text = match *animation {
                bouncing_ball::AnimationState::Paused => "Start Animation",
                bouncing_ball::AnimationState::Active => "Pause Animation",
            };

            if ui.button(button_text).clicked() {
                *animation = match *animation {
                    bouncing_ball::AnimationState::Paused => bouncing_ball::AnimationState::Active,
                    bouncing_ball::AnimationState::Active => bouncing_ball::AnimationState::Paused,
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("central panel");
            ui.hyperlink("https://github.com/emilk/bouncing_ball");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/bouncing_ball/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
            let rect = ui.max_rect();
            let painter = &ui.painter_at(rect);
            // draw background
            painter.rect_filled(
                Rect::from_min_max(
                    painter.clip_rect().left_top() + vec2(0.0, 0.0),
                    painter.clip_rect().right_bottom() + vec2(0.0, -0.0),
                ),
                3.0,
                Color32::LIGHT_GRAY,
            );
            // update ball position only if animation is active
            match *animation {
                bouncing_ball::AnimationState::Active => {
                    ball.update(&painter);
                    ui.ctx().request_repaint();
                }
                _ => (),
            }
            ball.draw(painter);

            // close the app if esc key pressed
            #[cfg(target_arch = "wasm32")]
            if ui.input().key_pressed(egui::Key::Escape) {
                _frame.close();
            }
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
