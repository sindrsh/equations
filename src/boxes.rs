use crate::prelude::*;
use crate::RECTANGLE;

const BOX_SIZE: f32 = 15.0;

pub struct Ones {
	s: f32,
	position: Coordinate<f32>,
	square: LineString<f32>,
	on_scale_left: bool,
	active: bool,
}

impl Ones {
	pub fn new(x: f32, y: f32) -> Self {
		let position = Coordinate{ x: x, y: y };
		let s = BOX_SIZE;
		let a = position + Coordinate{ x: -s, y: s };
		let b = a + Coordinate{ x: 2.0*s, y: 0.0};
		let c = b - Coordinate{ x: 0.0, y: 2.0*s};
		let d = a - Coordinate{ x: 0.0, y: 2.0*s}; 
		let square = LineString(vec![a, b, c, d, a]);
		Self {
			s: s,
			position: position,
			square: square,
			on_scale_left: true,
			active: false,
		}
	}
	
	fn get_square(&self) -> LineString<f32> {
		let o = self.position;
		let s = BOX_SIZE;
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
		);
		draw_rectangle_lines(
			self.position.x-BOX_SIZE, 
			self.position.y-BOX_SIZE, 
			2.0*BOX_SIZE, 
			2.0*BOX_SIZE, 
			5.0, 
			BLACK
		);	
	}
	
	pub fn update(
		&mut self, 
		mouse: Point<f32>, 
		c: Coordinate<f32>,
		boxes: i8,
		) -> i8 {
		if self.contains_mouse(mouse) {
			if is_mouse_button_pressed(MouseButton::Left) {
				self.active= true; 
				}
			
			if is_mouse_button_down(MouseButton::Left) 
			&& self.active {
				self.on_scale_left = false;
				let (x, y) = (self.position.x, self.position.y);
				self.position = Coordinate { 
					x: mouse.x(), 
					y: mouse.y() 
					};
			}
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
	
	pub fn contains_mouse(&self, mouse: Point<f32>) -> bool {
		let square = Polygon::new(
				self.get_square(),
				vec![],
				);
		square.contains(&mouse)	
	}
}

pub struct OnesButton {
	square_ls: LineString<f32>,
	triangle_vec2: Vec<Vec2>,
	triangle: Triangle<f32>,
	left: bool,
	color: Color, 
}

impl OnesButton {

	pub fn new() -> Self {
		let pos_square = Coordinate{ x: 50.0, y: 200.0 };
		let s = BOX_SIZE;
		let a = pos_square + Coordinate{ x: 2.0*s, y: 0.0};
		let b = a + Coordinate{ x: 0.0, y: 2.0*s};
		let c = b - Coordinate{ x: 2.0*s, y: 0.0}; 
		let square = LineString(vec![a, b, c, a]);
		
		let k = 20.0;
		let pos_triangle = Coordinate{ x: 90.0, y: 200.0 };
		let d = pos_triangle + Coordinate{ x: 2.0*k, y: 0.0};
		let e = d + Coordinate{ x: -k, y: -10.0 };
		let triangle = Triangle(pos_triangle, d, e);
		let triangle_vec2 = vec![
			Vec2::new(pos_triangle.x, pos_triangle.y), 
			Vec2::new(d.x, d.y),
			Vec2::new(e.x, e.y)
			];
		 
		Self {
			square_ls: square,
			triangle_vec2: triangle_vec2,
			triangle: triangle,
			left: true,
			color: GREEN,
		}
	}
	
	pub fn update(&mut self, mouse: Point<f32>) -> bool {
		self.color = GREEN;
		if self.triangle.contains(&mouse) {
			draw_text("INSIDE", 400.0, 100.0, 20.0, BLACK);
			if is_mouse_button_pressed(MouseButton::Left){
				self.color = PURPLE;
				return true;
			}
			false  	
		}
		else{ false }	
	}
	
	pub fn render(&self) {
		draw_triangle(
			self.triangle_vec2[0],
			self.triangle_vec2[1],
			self.triangle_vec2[2],
			self.color,
		);
	}
}
