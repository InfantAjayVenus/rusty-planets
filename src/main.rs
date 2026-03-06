mod planet;
use macroquad::prelude::*;
use planet::Planet;

#[macroquad::main("Rusty Planets")]
async fn main() {
    let cx = screen_width() / 2.0;
    let cy = screen_height() / 2.0;
    let sun = Planet::new(vec2(cx, cy), vec2(0.0, 0.0), 255.0, YELLOW);
    let earth = Planet::new(vec2(cx + 150.0, cy + 150.0), vec2(0.0, 0.0), 10.0, BLUE);

    loop {
        clear_background(BLACK);

        sun.draw();
        earth.draw();

        next_frame().await;
    }
}
