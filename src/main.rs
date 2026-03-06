mod planet;
use macroquad::prelude::*;
use planet::Planet;
use planet::gravity;

#[macroquad::main("Rusty Planets")]
async fn main() {
    let sun = Planet::new(vec2(0.0, 0.0), vec2(0.0, 0.0), 255.0, YELLOW);
    let mut earth = Planet::new(vec2(150.0, 150.0), vec2(1.0, 0.5), 10.0, BLUE);

    loop {
        clear_background(BLACK);

        sun.draw();

        let force = gravity(&earth, &sun);
        earth.update_velocity(&force);
        earth.update_position();
        earth.draw();

        next_frame().await;
    }
}
