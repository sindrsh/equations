use crate::prelude::*;
use crate::WINDOW_WIDTH;
use crate::RECTANGLE;
use crate::BOX_SIZE;
//const SHIFT: Point<f32> = Point::new(-10.0,-10.0);

const SHIFT: [f32; 2] = [60.0, -60.0];
const SHIFT_RIGHT: f32 = 50.0;
const LINE_THICKNESS: f32 = 8.0;
const ANGLE_SCALE: f32 = 0.3;
const ANGLE_TOL: i16 = 50;
pub const ORIGO: [f32; 2] = [WINDOW_WIDTH as f32/2.0_f32,100.0];

pub struct Scale {
	scale_vec: [LineString<f32>; 9],
	topline: LineString<f32>,
	left_vline: LineString<f32>,
	left_lwire: LineString<f32>,
	left_rwire: LineString<f32>,
	left_rectangle: LineString<f32>,
	right_vline: LineString<f32>,
	right_lwire: LineString<f32>,
	right_rwire: LineString<f32>,
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
		let topline = LineString(vec![o + Coordinate { x: topline_x, y:0.0 }, a]);
		let left_vline = LineString(vec![a, b]);
		let left_lwire = LineString(vec![b, c]);
		let left_rwire = LineString(vec![b, d]);
		let left_rectangle = LineString(vec![c, d, e, f, c]);
		
		let a = o + Coordinate { x: topline_x, y: 0. };
		let b = a + Coordinate { x: 0., y: vline_y };
		let c = b + Coordinate { x: -RECTANGLE[0]/2., y: wire_y };
		let d = b + Coordinate { x: RECTANGLE[0]/2., y: wire_y };
		let e = d + Coordinate { x: 0.0, y: RECTANGLE[1] };
		let f = c + Coordinate { x: 0.0, y: RECTANGLE[1] };
		let right_vline = LineString(vec![a, b]);
		let right_lwire = LineString(vec![b, c]);
		let right_rwire = LineString(vec![b, d]);
		let right_rectangle = LineString(vec![c, d, e, f, c]);
		
		Self {
			scale_vec: [ topline.clone(),
							left_vline.clone(),
							left_lwire.clone(),
							left_rwire.clone(),
							left_rectangle.clone(),
							right_vline.clone(),
							right_lwire.clone(),
							right_rwire.clone(),
							right_rectangle.clone(),
							],
			topline: topline,
			left_vline: left_vline,
			left_lwire: left_lwire,
			left_rwire: left_rwire,
			left_rectangle: left_rectangle,
			right_vline: right_vline,
			right_lwire: right_lwire,
			right_rwire: right_rwire,
			right_rectangle: right_rectangle,
			angle: 0,		
			color: GREEN,
		}
	}
	
	pub fn get_c(&self) -> Coordinate<f32> {
		Coordinate { 
			x: self.left_rectangle[0].x, 
			y: self.left_rectangle[0].y + RECTANGLE[1]/2.0
		}
	}
	
	pub fn get_c_right(&self) -> Coordinate<f32> {
		Coordinate { 
			x: self.right_rectangle[0].x, 
			y: self.right_rectangle[0].y + RECTANGLE[1]/2.0
		}
	}
	
	pub fn render(&self) {
		draw_line( 
			self.topline[0].x, 
			self.topline[0].y,
			self.topline[1].x,
			self.topline[1].y,
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
			GRAY,
			);
		draw_line( 
			self.right_vline[0].x, 
			self.right_vline[0].y,
			self.right_vline[1].x,
			self.right_vline[1].y,
			LINE_THICKNESS,
			GREEN,
			);
		draw_line( 
			self.right_lwire[0].x, 
			self.right_lwire[0].y,
			self.right_lwire[1].x,
			self.right_lwire[1].y,
			LINE_THICKNESS,
			GREEN,
			);
		draw_line( 
			self.right_rwire[0].x, 
			self.right_rwire[0].y,
			self.right_rwire[1].x,
			self.right_rwire[1].y,
			LINE_THICKNESS,
			GREEN,
			);						
		draw_line( 
			self.right_rectangle[0].x, 
			self.right_rectangle[0].y + RECTANGLE[1]/2.,
			self.right_rectangle[1].x,
			self.right_rectangle[1].y + RECTANGLE[1]/2.,
			RECTANGLE[1],
			GRAY,
			);		
	}
	
	pub fn update(&mut self, balance: i8) {
		if balance != 0 {
			let mut v = 1;
			draw_text(&format!("({})", self.angle), 400.0, 80.0, 20.0, BLACK); 
			if balance > 0 { v = -1; }
			if self.angle == ANGLE_TOL + 1 
				&& balance > 0	{ 
					self.angle = ANGLE_TOL; 
				}
			if self.angle == -(ANGLE_TOL+1) 
				&& balance < 0 { 
					self.angle = -ANGLE_TOL; 
				}
			if self.angle.abs() <= ANGLE_TOL {
				let o  = Point::new(ORIGO[0], ORIGO[1]);
				self.angle += v;
				let prev_left = self.topline[1];
				let prev_right = self.topline[0];
				self.topline = self.topline.rotate_around_point(
					v as f32*ANGLE_SCALE,
					o,
				);
				let x = self.topline[1].x-prev_left.x;
				let y = self.topline[1].y-prev_left.y;
				self.left_vline = self.left_vline.translate(x, y);
				self.left_lwire = self.left_lwire.translate(x, y);
				self.left_rwire = self.left_rwire.translate(x, y);
				self.left_rectangle = self.left_rectangle.translate(x, y);
				let x = self.topline[0].x-prev_right.x;
				let y = self.topline[0].y-prev_right.y;
				self.right_vline = self.right_vline.translate(x, y);
				self.right_lwire = self.right_lwire.translate(x, y);
				self.right_rwire = self.right_rwire.translate(x, y);
				self.right_rectangle = self.right_rectangle.translate(x, y);
			}
	} else {
		self.angle = 0;	
		self.topline = self.scale_vec[0].clone();
		self.left_vline = self.scale_vec[1].clone();
		self.left_lwire = self.scale_vec[2].clone();
		self.left_rwire = self.scale_vec[3].clone();
		self.left_rectangle = self.scale_vec[4].clone();
		self.right_vline = self.scale_vec[5].clone();
		self.right_lwire = self.scale_vec[6].clone();
		self.right_rwire = self.scale_vec[7].clone();
		self.right_rectangle = self.scale_vec[8].clone();
	}

	}
}
