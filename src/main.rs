use macroquad::prelude::*;

#[macroquad::main("Rusty Planets")]
async fn main() {
   loop {
       clear_background(BLACK);
       next_frame().await;
   } 
}
