use macroquad::prelude::*;

pub struct Planet {
    pub mass: f32,
    pub color: Color,
    pub radius: f32,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Planet {
    pub fn new(position: Vec2, velocity: Vec2, mass: f32, color: Color) -> Planet {
        Planet {
            position,
            velocity,
            mass,
            color,
            radius: mass.sqrt() * 2.0,
        }
    }

    pub fn draw(&self) {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        draw_circle(self.position.x + cx, self.position.y + cy, self.radius, self.color);
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
    }
}
