use macroquad::{
	prelude::*,
	math::Vec2,
};

use crate::{ paddle::Paddle, ball::Ball };

pub struct GameState {
	player_one: Paddle,
	player_two: Paddle,
	ball: Ball,
	ball_collides_with_player: bool,
}

impl GameState {
	pub fn new(
		player_offset_from_screen_edge: f32,
		player_size: Vec2, speed: f32,
		ball_radius: f32, ball_speed: f32,
	) -> Self {
		Self {
			// Setup player one
			player_one: Paddle::new(
				Vec2::new(
					player_offset_from_screen_edge,
					crate::CANVAS_SIZE.y / 2.,
				),
				player_size,
				speed,
				KeyCode::W,
				KeyCode::S,
			),
			// Setup player two
			player_two: Paddle::new(
				Vec2::new(
					crate::CANVAS_SIZE.x - player_offset_from_screen_edge,
					crate::CANVAS_SIZE.y / 2.,
				),
				player_size,
				speed,
				KeyCode::Up,
				KeyCode::Down,
			),
			// Setup ball
			ball: Ball::new(
				crate::CANVAS_SIZE / 2.,
				Vec2::ONE,
				ball_radius,
				ball_speed,
			),
			ball_collides_with_player: false,
		}
	}

	pub fn update(&mut self) {
		self.player_one.update();
		self.player_two.update();
		self.ball.update();

		let ball_rect = self.ball.get_rect();

		let new_ball_collides_with_player = 
			ball_rect.overlaps(&self.player_one.get_rect()) ||
			ball_rect.overlaps(&self.player_two.get_rect());

		// Began ball overlap with any player
		if new_ball_collides_with_player
		&& !self.ball_collides_with_player {
			self.ball.invert_x_axis();
		}

		self.ball_collides_with_player = new_ball_collides_with_player;
	}

	pub fn draw(&self) {
		clear_background(BLACK);

		self.player_one.draw();
		self.player_two.draw();
		self.ball.draw();
	}
}
