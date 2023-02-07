// pub mod bouncing_ball;
use egui::*;
use random_color::RandomColor;

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
    let color = RandomColor::new()
        .luminosity(random_color::Luminosity::Dark)
        .to_rgb_array();
    Color32::from_rgb(color[0], color[1], color[2])
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
            animation: AnimationState::Paused,
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
