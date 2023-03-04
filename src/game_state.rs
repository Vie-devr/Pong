use macroquad::{
	prelude::*,
	math::Vec2,
};

use crate::{ paddle::Paddle, ball::Ball, CANVAS_SIZE };

pub struct GameState {
	player_one: Paddle,
	player_two: Paddle,
	ball: Ball,
	ball_collides_with_player: bool,
	player_one_score: usize,
	player_two_score: usize,
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
				player_offset_from_screen_edge,
				player_size,
				speed,
				KeyCode::W,
				KeyCode::S,
			),
			// Setup player two
			player_two: Paddle::new(
				CANVAS_SIZE.x - player_offset_from_screen_edge,
				player_size,
				speed,
				KeyCode::Up,
				KeyCode::Down,
			),
			// Setup ball
			ball: Ball::new(
				ball_radius,
				ball_speed,
			),
			ball_collides_with_player: false,
			player_one_score: 0,
			player_two_score: 0,
		}
	}

	pub fn update(&mut self) {
		self.player_one.update();
		self.player_two.update();
		self.ball.update();

		self.check_ball_collision();
	}

	pub fn draw(&self) {
		clear_background(BLACK);

		self.player_one.draw();
		self.player_two.draw();
		self.ball.draw();

		self.draw_score_text();
	}

	fn check_ball_collision(&mut self) {
		let ball_rect = self.ball.get_rect();
		
		let new_ball_collides_with_player = 
			ball_rect.overlaps(&self.player_one.get_rect()) ||
			ball_rect.overlaps(&self.player_two.get_rect());

		// Began ball overlap with any player
		if new_ball_collides_with_player
		&& !self.ball_collides_with_player {
			self.ball.dir.x *= -1.;
		}

		self.ball_collides_with_player = new_ball_collides_with_player;

		// Player two wins
		if self.ball.pos.x <= self.ball.radius {
			self.player_two_score += 1;

			self.player_one.reset();
			self.player_two.reset();
			self.ball.reset();
		}

		// Player one wins
		if self.ball.pos.x >= CANVAS_SIZE.x - self.ball.radius {
			self.player_one_score += 1;

			self.player_one.reset();
			self.player_two.reset();
			self.ball.reset();
		}
	}

	fn draw_score_text(&self) {
		let (pos, pts) = (self.player_one_score, self.player_two_score);
		let score_text = &format!("{pos} : {pts}");
		let dimensions = measure_text(score_text, None, 1, 40.);

		draw_text(
			score_text,
			CANVAS_SIZE.x / 2. - dimensions.width / 2.,
			40.,
			40.,
			WHITE,
		);
	}
}
