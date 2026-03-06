mod planet;
use macroquad::prelude::*;
use planet::Planet;
use planet::gravity;

#[macroquad::main("Rusty Planets")]
async fn main() {
    let mut sun = Planet::new("Sun", vec2(0.0, 0.0), vec2(0.0, 0.0), 255.0, YELLOW);
    sun.set_fixed();
    let earth = Planet::new("Earth", vec2(150.0, 0.0), vec2(0.0, 30.0), 10.0, BLUE);
    let mars = Planet::new("Mars", vec2(280.0, 0.0), vec2(0.0, 1.5), 6.0, RED);
    let jupiter = Planet::new("Jupiter", vec2(450.0, 0.0), vec2(0.0, 19.1), 80.0, ORANGE);

    let mut bodies = vec![sun, earth, mars, jupiter];

    loop {
        clear_background(BLACK);

        let forces: Vec<Vec2> = (0..bodies.len())
            .map(|i| {
                let mut total = Vec2::ZERO;
                for j in 0..bodies.len() {
                    if i != j {
                        total += gravity(&bodies[i], &bodies[j]);
                    }
                }
                total
            })
            .collect();

        for (body, force) in bodies.iter_mut().zip(forces.iter()) {
            body.update_velocity(force);
            body.update_position();
            body.draw();
        }

        next_frame().await;
    }
}
