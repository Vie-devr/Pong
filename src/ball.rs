use macroquad::{
	prelude::*,
	math::{ Vec2, Rect },
};

pub struct Ball {
	pos: Vec2,
	dir: Vec2,
	radius: f32,
	speed: f32,
}

impl Ball {
	pub fn new(pos: Vec2, dir: Vec2, radius: f32, speed: f32) -> Self {
		Self {
			pos,
			dir,
			radius,
			speed,
		}
	}

	pub fn update(&mut self) {
		self.pos += self.dir * self.speed;

		// We are out of bounds by x axis
		if self.pos.x <= self.radius
		|| self.pos.x >= crate::CANVAS_SIZE.x - self.radius {
			self.invert_x_axis();
		}

		// We are out of bounds by y axis
		if self.pos.y <= self.radius
		|| self.pos.y >= crate::CANVAS_SIZE.y - self.radius {
			self.invert_y_axis();
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

	pub fn invert_x_axis(&mut self) {
		self.dir.x *= -1.;
	}

	pub fn invert_y_axis(&mut self) {
		self.dir.y *= -1.;
	}
}
