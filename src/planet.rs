use std::collections::VecDeque;

use macroquad::prelude::*;

pub struct Planet {
    pub mass: f32,
    pub color: Color,
    pub position: Vec2,
    pub velocity: Vec2,
    pub name: String,
    radius: f32,
    is_fixed: bool,
    trail: VecDeque<Vec2>,
}

impl Planet {
    pub fn new(name: &str, position: Vec2, velocity: Vec2, mass: f32, color: Color) -> Planet {
        Planet {
            name: name.to_string(),
            position,
            velocity,
            mass,
            color,
            radius: mass.sqrt() * 2.0,
            is_fixed: false,
            trail: VecDeque::new(),
        }
    }

    pub fn set_fixed(&mut self) {
        self.is_fixed = true;
    }

    pub fn draw(&self) {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        draw_circle(
            self.position.x + cx,
            self.position.y + cy,
            self.radius,
            self.color,
        );
        self.draw_label(cx, cy);

        self.draw_trail(cx, cy);
    }

    pub fn update_velocity(&mut self, force: &Vec2) {
        self.velocity += *force / self.mass;
    }

    pub fn update_position(&mut self) {
        if !self.is_fixed {
            self.position += self.velocity;
            self.update_trail();
        }
    }

    fn draw_label(&self, cx: f32, cy:f32) {
        let speed = self.velocity.length();
        let label = format!("{} ({:.1})", self.name, speed);

        draw_text(&label, self.position.x + cx + self.radius + 4.0, self.position.y + cy + self.radius, 14.0, self.color);
    }

    fn draw_trail(&self, cx: f32, cy: f32) {
        for (i,pos) in self.trail.iter().enumerate() {
            let alpha = i as f32 / self.trail.len() as f32;
            let trail_color = Color::new(self.color.r, self.color.g, self.color.b, alpha);

            draw_circle(cx + pos.x, cy + pos.y, 1.0, trail_color);
        }
    }

    fn update_trail(&mut self) {
        const TRAIL_LEN: usize = 120;
        self.trail.push_back(self.position);

        if self.trail.len() > TRAIL_LEN  {
            self.trail.pop_front();
        }
    }


}

pub fn gravity(first_body: &Planet, second_body: &Planet) -> Vec2 {
    const GRAVITATIONAL_CONSTANT: f32 = 500.0;
    let direction = second_body.position - first_body.position;
    let distance = direction.length().max(100.0);
    let force_value =
        GRAVITATIONAL_CONSTANT * first_body.mass * second_body.mass / distance.powf(2.0);

    direction.normalize() * force_value
}
