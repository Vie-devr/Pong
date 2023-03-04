use macroquad::{
	prelude::*,
	math::Vec2,
};
use macroquad_canvas::Canvas2D;

use crate::game_state::GameState;

mod game_state;
mod paddle;
mod ball;

const CANVAS_SIZE: Vec2 = Vec2::new(800., 600.);

#[macroquad::main("Pong!")]
async fn main() {
	let mut state = GameState::new(
		50.,                  // Player offset from screen edge
		Vec2::new(20., 120.), // Player size
		5.,                   // Player speed
		15.,                  // Ball radius
		7.5,                  // Ball speed
	);
	
	let canvas = Canvas2D::new(
		CANVAS_SIZE.x,
		CANVAS_SIZE.y,
	);

    loop {
		set_camera(&canvas.camera);

		state.update();
		state.draw();

		set_default_camera();
		canvas.draw();
    
        next_frame().await
    }
}
