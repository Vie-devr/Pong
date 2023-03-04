use macroquad::{
	prelude::*,
	math::{ Vec2, Rect },
};

use crate::CANVAS_SIZE;

pub struct Paddle {
	screen_edge_offset: f32,
	pos_y: f32,
	size: Vec2,
	speed: f32,
	move_up_key: KeyCode,
	move_down_key: KeyCode,
}

impl Paddle {
	pub fn new(
		screen_edge_offset: f32, size: Vec2, speed: f32,
		move_up_key: KeyCode, move_down_key: KeyCode,
	) -> Self {
		Self {
			screen_edge_offset,
			pos_y: CANVAS_SIZE.y / 2.,
			size,
			speed,
			move_up_key,
			move_down_key,
		}
	}

	pub fn update(&mut self) {
		let move_dir = is_key_down(self.move_down_key) as i32 as f32
					 - is_key_down(self.move_up_key) as i32 as f32;
	
		self.pos_y += move_dir * self.speed;
		self.pos_y = self.pos_y.clamp(
			self.size.y / 2.,
			CANVAS_SIZE.y - self.size.y / 2.,
		);
	}

	pub fn draw(&self) {
		let rect = self.get_rect();

		draw_rectangle(
			rect.x,
			rect.y,
			rect.w,
			rect.h,
			WHITE,
		);
	}

	pub fn get_rect(&self) -> Rect {
		Rect::new(
			self.screen_edge_offset - self.size.x / 2.,
			self.pos_y - self.size.y / 2.,
			self.size.x,
			self.size.y,
		)
	}

	pub fn reset(&mut self) {
		self.pos_y = CANVAS_SIZE.y / 2.;
	}
}
