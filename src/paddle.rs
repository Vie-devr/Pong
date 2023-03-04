use macroquad::{
	prelude::*,
	math::{ Vec2, Rect },
};

pub struct Paddle {
	pos: Vec2,
	size: Vec2,
	speed: f32,
	move_up_key: KeyCode,
	move_down_key: KeyCode,
}

impl Paddle {
	pub fn new(
		pos: Vec2, size: Vec2, speed: f32,
		move_up_key: KeyCode, move_down_key: KeyCode,
	) -> Self {
		Self {
			pos,
			size,
			speed,
			move_up_key,
			move_down_key,
		}
	}

	pub fn update(&mut self) {
		let move_dir = is_key_down(self.move_down_key) as i32 as f32
					 - is_key_down(self.move_up_key) as i32 as f32;
	
		self.pos.y += move_dir * self.speed;
		self.pos.y = self.pos.y.clamp(
			self.size.y / 2.,
			crate::CANVAS_SIZE.y - self.size.y / 2.
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
			self.pos.x - self.size.x / 2.,
			self.pos.y - self.size.y / 2.,
			self.size.x,
			self.size.y,
		)
	}
}
