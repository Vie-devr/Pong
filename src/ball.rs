use macroquad::{
	prelude::*,
	math::{ Vec2, Rect },
	rand::gen_range,
};

use crate::CANVAS_SIZE;

pub struct Ball {
	pub pos: Vec2,
	pub dir: Vec2,
	pub radius: f32,
	speed: f32,
}

impl Ball {
	pub fn new(radius: f32, speed: f32) -> Self {
		Self {
			pos: CANVAS_SIZE / 2.,
			dir: Self::get_random_dir(),
			radius,
			speed,
		}
	}

	pub fn update(&mut self) {
		self.pos += self.dir * self.speed;

		// We are out of bounds by y axis
		if self.pos.y <= self.radius
		|| self.pos.y >= CANVAS_SIZE.y - self.radius {
			self.dir.y *= -1.;
		}
	}

	pub fn draw(&self) {
		draw_circle(
			self.pos.x,
			self.pos.y,
			self.radius,
			WHITE,
		);
	}

	pub fn get_rect(&self) -> Rect {
		Rect::new(
			self.pos.x - self.radius,
			self.pos.y - self.radius,
			self.radius * 2.,
			self.radius * 2.,
		)
	}

	pub fn reset(&mut self) {
		self.pos = CANVAS_SIZE / 2.;
		self.dir = Self::get_random_dir();
	}

	fn get_random_dir() -> Vec2 {
		let factor = if gen_range(0, 2) == 1 {1.} else {-1.};

		Vec2::new(
			factor,
			gen_range(0.4, 0.6) * factor,
		)
	}
}
