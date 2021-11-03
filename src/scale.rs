use crate::prelude::*;
use crate::WINDOW_WIDTH;
use crate::RECTANGLE;
use crate::BOX_SIZE;
//const SHIFT: Point<f32> = Point::new(-10.0,-10.0);

const SHIFT: [f32; 2] = [60.0, -60.0];
const SHIFT_RIGHT: f32 = 50.0;
const LINE_THICKNESS: f32 = 8.0;
const ANGLE_SCALE: f32 = 0.05;
const ANGLE_TOL: i16 = 250;
pub const ORIGO: [f32; 2] = [WINDOW_WIDTH as f32/2.0_f32,100.0];

pub struct Scale {
	scale_vec: [LineString<f32>; 6],
	left_topline: LineString<f32>,
	left_vline: LineString<f32>,
	left_lwire: LineString<f32>,
	left_rwire: LineString<f32>,
	left_rectangle: LineString<f32>,
	right_rectangle: LineString<f32>,
	angle: i16,
	color: Color,
}

impl Scale {
	pub fn new() -> Self {
		let topline_x = 500.0;
		let vline_y = 100.0; 
		let wire_y = 80.0;
		let o = Coordinate { x: ORIGO[0], y: ORIGO[1] }; 
		let a = o + Coordinate { x: -topline_x, y: 0. };
		let b = a + Coordinate { x: 0., y: vline_y };
		let c = b + Coordinate { x: -RECTANGLE[0]/2., y: wire_y };
		let d = b + Coordinate { x: RECTANGLE[0]/2., y: wire_y };
		let e = d + Coordinate { x: 0.0, y: RECTANGLE[1] };
		let f = c + Coordinate { x: 0.0, y: RECTANGLE[1] };
		let left_topline = LineString(vec![o, a]);
		let left_vline = LineString(vec![a, b]);
		let left_lwire = LineString(vec![b, c]);
		let left_rwire = LineString(vec![b, d]);
		let rectangle_line_string = LineString(vec![c, d, e, f, c]);
		let left_rectangle = LineString(vec![c, d, e, f, c]);
		let mut right_rectangle = rectangle_line_string.translate(SHIFT[0], SHIFT[1]); 
		right_rectangle = right_rectangle.translate(SHIFT_RIGHT, 0.0);
		
		Self {
			scale_vec: [ left_topline.clone(),
							left_vline.clone(),
							left_lwire.clone(),
							left_rwire.clone(),
							left_rectangle.clone(),
							right_rectangle.clone()
							],
			left_topline: left_topline,
			left_vline: left_vline,
			left_lwire: left_lwire,
			left_rwire: left_rwire,
			left_rectangle: left_rectangle,
			right_rectangle: right_rectangle,
			angle: 0,		
			color: GREEN,
		}
	}
	pub fn get_c(&self, pos: bool) -> Coordinate<f32> {
		if pos{
		return Coordinate { 
			x: self.left_rectangle[0].x, 
			y: self.left_rectangle[0].y
		};
		}
		Coordinate { 
			x: self.left_rectangle[0].x, 
			y: self.left_rectangle[0].y 
				+ RECTANGLE[1] + 2.0*BOX_SIZE
		}
	}
	
	
	pub fn render(&self) {
		draw_line( 
			self.left_topline[0].x, 
			self.left_topline[0].y,
			self.left_topline[1].x,
			self.left_topline[1].y,
			LINE_THICKNESS,
			GREEN,
			);
		draw_line( 
			self.left_vline[0].x, 
			self.left_vline[0].y,
			self.left_vline[1].x,
			self.left_vline[1].y,
			LINE_THICKNESS,
			GREEN,
			);
		draw_line( 
			self.left_lwire[0].x, 
			self.left_lwire[0].y,
			self.left_lwire[1].x,
			self.left_lwire[1].y,
			LINE_THICKNESS,
			GREEN,
			);
		draw_line( 
			self.left_rwire[0].x, 
			self.left_rwire[0].y,
			self.left_rwire[1].x,
			self.left_rwire[1].y,
			LINE_THICKNESS,
			GREEN,
			);						
		draw_line( 
			self.left_rectangle[0].x, 
			self.left_rectangle[0].y + RECTANGLE[1]/2.,
			self.left_rectangle[1].x,
			self.left_rectangle[1].y + RECTANGLE[1]/2.,
			RECTANGLE[1],
			RED,
			);	
	}
	
	pub fn update(&mut self, balance: i8) {
		if balance != 0 {
			let mut v = 1;
			draw_text(&format!("({})", self.angle), 400.0, 80.0, 20.0, BLACK); 
			if balance > 0 { v = -1; }
			if self.angle.abs() <= ANGLE_TOL {
				let o  = Point::new(ORIGO[0], ORIGO[1]);
				self.angle += v;
				let prev = self.left_topline[1];
				self.left_topline = self.left_topline.rotate_around_point(
					v as f32*ANGLE_SCALE,
					o,
				);
				let x = self.left_topline[1].x-prev.x;
				let y = self.left_topline[1].y-prev.y;
				self.left_vline = self.left_vline.translate(x, y);
				self.left_lwire = self.left_lwire.translate(x, y);
				self.left_rwire = self.left_rwire.translate(x, y);
				self.left_rectangle = self.left_rectangle.translate(x, y);
			}
	} else {
		self.angle = 0;	
		self.left_topline = self.scale_vec[0].clone();
		self.left_vline = self.scale_vec[1].clone();
		self.left_lwire = self.scale_vec[2].clone();
		self.left_rwire = self.scale_vec[3].clone();
		self.left_rectangle = self.scale_vec[4].clone();
	}

	}
}
