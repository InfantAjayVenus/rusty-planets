use macroquad::prelude::*;

#[macroquad::main("Rusty Planets")]
async fn main() {
    loop {
        clear_background(BLACK);

        let sun_radius = 30.0;
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;

        draw_circle(cx, cy, sun_radius, YELLOW);

        next_frame().await;
    }
}
