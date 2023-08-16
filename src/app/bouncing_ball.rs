// pub mod bouncing_ball;
use colors_transform::Color;
use egui::*;
use rand::Rng;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum AnimationState {
    Paused,
    Active,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Ball {
    position: Vec2,
    radius: f32,
    fill_color: Color32,
    velocity: Vec2,
    acceleration: Vec2,
    elasticity: f32,
    rolling_friction: f32,
    animation: AnimationState,
}

fn random_color() -> ecolor::Color32 {
    let h = rand::thread_rng().gen_range(0.0..=359.0);
    let s = 90.0;
    let l = 25.0;
    let rgb = colors_transform::Hsl::from(h, s, l).to_rgb();

    Color32::from_rgb(
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
    )
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: vec2(50.0, 400.0),
            radius: 10.0,
            // fill_color: Color32::DARK_BLUE,
            fill_color: random_color(),
            velocity: vec2(3.0, 2.0),
            acceleration: vec2(0.0, -0.5),
            elasticity: 0.85,
            rolling_friction: 0.05,
            animation: AnimationState::Active,
        }
    }

    pub fn change_elastity_friction(&mut self, elasticity: f32, rolling_friction: f32) {
        self.elasticity = elasticity;
        self.rolling_friction = rolling_friction;
    }

    pub fn update(&mut self, painter: &Painter) {
        // println!("ball is at {:?} with color {:?}", self.position, self.fill_color);
        self.position += self.velocity;
        // position can't be below 0
        if self.position.y <= self.radius {
            self.position.y = self.radius;
            self.velocity.x *= 1.0 - self.rolling_friction;
            self.velocity.y *= -self.elasticity;
        }
        // position can't be right of frame
        if self.position.x >= painter.clip_rect().width() - self.radius {
            self.position.x = painter.clip_rect().width() - self.radius;
            self.velocity.x *= -1.0;
        }
        if self.position.x <= self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        }
        self.velocity += self.acceleration;
    }

    pub fn draw(&self, painter: &Painter) {
        painter.circle_filled(
            painter.clip_rect().left_bottom() + vec2(self.position.x, -self.position.y),
            self.radius,
            self.fill_color,
        );
    }
}
