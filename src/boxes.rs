use crate::prelude::*;
use crate::RECTANGLE;

const BOX_SIZE: f32 = 15.0;

pub struct Ones {
	s: f32,
	position: Coordinate<f32>,
	square: LineString<f32>,
	on_scale_left: bool,
}

impl Ones {
	pub fn new(x: f32, y: f32) -> Self {
		let position = Coordinate{ x: x, y: y };
		let s = 15.0;
		let a = position + Coordinate{ x: -s, y: s };
		let b = a + Coordinate{ x: 2.0*s, y: 0.0};
		let c = b - Coordinate{ x: 0.0, y: 2.0*s};
		let d = a - Coordinate{ x: 0.0, y: 2.0*s}; 
		let square = LineString(vec![a, b, c, d, a]);
		Self {
			s: s,
			position: position,
			square: square,
			on_scale_left: false,
		}
	}
	
	fn get_square(&self) -> LineString<f32> {
		let o = self.position;
		let s = 15.0;
		let a = o + Coordinate{ x: -s, y: s };
		let b = a + Coordinate{ x: 2.0*s, y: 0.0};
		let c = b - Coordinate{ x: 0.0, y: 2.0*s};
		let d = a - Coordinate{ x: 0.0, y: 2.0*s}; 
		LineString(vec![a, b, c, d, a])	
	}
	
	pub fn render(&self){
		draw_poly(
			self.position.x,
			self.position.y,
			4,
			2_f32.sqrt()*self.s,
			45.0,
			BLUE
		)	
	}
	
	pub fn update(
		&mut self, 
		mouse_pos: (f32, f32), 
		c: Coordinate<f32>,
		boxes: i8,
		) -> i8 {
		if self.contains_mouse(mouse_pos) 
		&& is_mouse_button_down(MouseButton::Left) {
			self.on_scale_left = false;
			let (x, y) = (self.position.x, self.position.y);
			self.position = Coordinate { x: mouse_pos.0, y: mouse_pos.1 };
		}
		if is_mouse_button_down(MouseButton::Left) != true {
			if c.x < self.position.x 
			&& self.position.x < c.x + RECTANGLE[0]
			&& c.y-BOX_SIZE < self.position.y
			&& self.position.y < c.y {
				self.on_scale_left = true;
			}    
		}
		if self.on_scale_left {
			self.position = c + Coordinate{ x: BOX_SIZE + boxes as f32*(2.0*BOX_SIZE+10.0), y: -BOX_SIZE };
		}
		if self.on_scale_left == true { return 1; }
		0
	}
	
	pub fn contains_mouse(&self, mouse_pos: (f32, f32)) -> bool {
		let mouse = Point::new(mouse_pos.0, mouse_pos.1);
		let square = Polygon::new(
				self.get_square(),
				vec![],
				);
		square.contains(&mouse)	
	}
}
