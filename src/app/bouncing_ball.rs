// pub mod bouncing_ball;
use egui::*;
// use ecolor::*;

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
    elasticity: Vec2,
    animation: AnimationState,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: vec2(50.0, 400.0),
            radius: 10.0,
            fill_color: Color32::DARK_BLUE,
            velocity: vec2(3.0, 2.0),
            acceleration: vec2(0.0, -0.5),
            elasticity: vec2(0.85, 0.85),
            animation: AnimationState::Paused,
        }
    }

    pub fn update(&mut self, painter: &Painter) {
        // println!("ball is at {:?} with color {:?}", self.position, self.fill_color);
        self.position += self.velocity;
        // position can't be below 0
        if self.position.y <= self.radius {
            self.position.y = self.radius;
            self.velocity.x *= self.elasticity.x;
            self.velocity.y *= -self.elasticity.y;
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
