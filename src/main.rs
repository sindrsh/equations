mod scale;
mod boxes;

mod prelude{
	pub use macroquad::prelude::*;
	pub use geo::{Point, Coordinate,LineString, Polygon, Triangle};
	pub use geo::algorithm::contains::Contains;
	pub use geo::algorithm::translate::Translate;
	pub use geo::algorithm::rotate::RotatePoint;
	pub use std::{thread, time};
	pub use crate::scale::*;
	pub use crate::boxes::*;
}
use prelude::*;

const RECTANGLE: [f32; 2] = [500.0, 15.0];
const BOX_SIZE: f32 = 15.0;
const XBOX_SIZE: f32 = 20.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Equations".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

pub const WINDOW_WIDTH: usize = 1600;
pub const WINDOW_HEIGHT: usize = 1200;
pub const ORIGO: [f32; 2] = [50.0,50.0];

#[macroquad::main(window_conf)]
async fn main() {
	const Ones_start: f32 = RECTANGLE[0]/2.;
	let xval = 4i8;
	let mut balance = 0;
	
	let mut scale = Scale::new();
	
	let btn_dif = 10.0;
	let btn_x = 50.0;
	let btn_y = 500.0;
	let btn_dy = 100.0;
	
	let btn_x_right = 1200.0;
	
	// BUTTONS	
	let mut ones_left = OnesButton::new(btn_x, btn_y, BLUE, -1);
	let mut ones_left_remove = OnesButton::new(btn_x, btn_y+btn_dif, BLUE, 1);
	let mut ones_right = OnesButton::new(btn_x_right, btn_y, BLUE, -1);
	let mut ones_right_remove = OnesButton::new(btn_x_right, btn_y+btn_dif, BLUE, 1);	
	
	let mut neg_ones_left = OnesButton::new(btn_x, btn_y + btn_dy, LIME, -1);
	let mut neg_ones_left_remove = OnesButton::new(btn_x, btn_y + btn_dy+btn_dif, LIME, 1);
	let mut neg_ones_right = OnesButton::new(btn_x_right, btn_y + btn_dy, LIME, -1);
	let mut neg_ones_right_remove = OnesButton::new(btn_x_right, btn_y + btn_dy+btn_dif, LIME, 1);
	
	let mut xbox_left = OnesButton::new(btn_x, btn_y + 2.0*btn_dy, GRAY, -1);
	let mut xbox_left_remove = OnesButton::new(btn_x, btn_y + 2.0*btn_dy+btn_dif, GRAY, 1);
	let mut xbox_right = OnesButton::new(btn_x_right, btn_y + 2.0*btn_dy, GRAY, -1);
	let mut xbox_right_remove = OnesButton::new(btn_x_right, btn_y + 2.0*btn_dy+btn_dif, GRAY, 1);
	
	let mut neg_xbox_left = OnesButton::new(btn_x, btn_y + 3.0*btn_dy, GRAY, -1);
	let mut neg_xbox_left_remove = OnesButton::new(
		btn_x, btn_y + 3.0*btn_dy+btn_dif, GRAY, 1);
	let mut neg_xbox_right = OnesButton::new(btn_x_right, btn_y + 3.0*btn_dy, GRAY, -1);
	let mut neg_xbox_right_remove = OnesButton::new(
		btn_x_right, btn_y + 3.0*btn_dy+btn_dif, GRAY, 1);	
	
	
	let mut ones_vec: Vec<Box> = Vec::new();
	let mut ones_right_vec: Vec<Box> = Vec::new();
	
	let mut neg_ones_vec: Vec<Box> = Vec::new();
	let mut neg_ones_right_vec: Vec<Box> = Vec::new();
	
	let mut xbox_vec: Vec<Box> = Vec::new();
	let mut xbox_right_vec: Vec<Box> = Vec::new();
	
	let mut neg_xbox_left_vec: Vec<Box> = Vec::new();
	let mut neg_xbox_right_vec: Vec<Box> = Vec::new();	

	fn update_box(
        	mut box1: Vec<Box>,
        	mut box2: Vec<Box>, 
        	c: Coordinate<f32>, 
        	c_r: Coordinate<f32>,
        	mouse: Point<f32>,
        	unknown: bool,
        	left: bool,
        	start: f32,
        	value: i8
        	) -> (Vec<Box>, Vec<Box>, i8) {
        	let mut boxes = 0;
        	let mut change_index: Option<usize> = None;
		    let mut out_index: Option<usize> = None;
		    for i in 0..box1.len() {
				if box1[i].contains_mouse(mouse) { 
					draw_text("INSIDE", 400.0, 100.0, 20.0, BLACK); 
				}
				let (cnt, change) = box1[i].update(
					mouse, 
					c,
					c_r,  
					boxes);
				if change{ change_index = Some(i); }
				else if box1[i].get_y() > 900.0 || box1[i].get_y() < 20.0 { out_index = Some(i) }
				else {
					boxes += cnt; 
					box1[i].render(); 
					}	
		    }
		    if change_index.is_some() {
		    	box1.swap_remove(change_index.unwrap());
		    	box2.push(
		    		Box::new(
		    		unknown,
		    		left,
		    		left,
		    		start,
		    		c.x, 
		    		c.y, 
		    		value)
		    		);  	
		    }
		    if out_index.is_some() { box1.swap_remove(out_index.unwrap()); }
		    (box1, box2, boxes)
        }
	
	loop {
    	clear_background(WHITE);
    	
    	// Handling the scale
    	scale.render();
        
        // <writing mouse position on screen 
    	let (x, y) = mouse_position();
    	let mouse = Point::new(x, y);
    	let mouse_x_and_y = format!("({}, {})", x, y);
    	draw_text(&mouse_x_and_y, 100.0, 100.0, 20.0, BLACK); 
        draw_text(&balance.to_string(), 400.0, 50.0, 20.0, BLACK); 
        
        // make left buttons
        ones_left.render();
        ones_left_remove.render();
        
        ones_right.render();
        ones_right_remove.render();
        
        neg_ones_left.render();
        neg_ones_left_remove.render();
        
        neg_ones_right.render();
        neg_ones_right_remove.render();
        
        
        xbox_left.render();
        xbox_left_remove.render();
        xbox_right.render();
        xbox_right_remove.render();
        
        neg_xbox_left.render();
        neg_xbox_left_remove.render();
        neg_xbox_right.render();
        neg_xbox_right_remove.render();
        
		// make buttons        
        if ones_left.update(mouse) {
        	ones_vec.push(
        		Box::new(
        		false,
        		true,
        		true,
        		Ones_start, 
        		scale.get_c().x, 
        		scale.get_c_right().y, 
        		1)
        		); 	
        }
        
         if ones_right.update(mouse) {
        	ones_right_vec.push(
        		Box::new(
        		false,
        		false,
        		false,
        		Ones_start, 
        		scale.get_c_right().x, 
        		scale.get_c_right().y, 
        		1)
        		); 	
        }
        
        if neg_ones_left.update(mouse) {
        	neg_ones_vec.push(
        		Box::new(
        			false,
        			true,
        			true,
        			Ones_start,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			-1)
        			); 	
        }
        
        if neg_ones_right.update(mouse) {
        	neg_ones_right_vec.push(
        		Box::new(
        			false,
        			false,
        			false,
        			Ones_start,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			-1)
        			); 	
        }
        
        if xbox_left.update(mouse) {
        	xbox_vec.push(
        		Box::new(
        			true,
        			true,
        			true,
        			0.,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			1)
        			); 	
        }
        if xbox_right.update(mouse) {
        	xbox_right_vec.push(
        		Box::new(
        			true,
        			false,
        			false,
        			0.,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			1)
        			); 	
        }
        
        
        if neg_xbox_left.update(mouse) {
        	neg_xbox_left_vec.push(
        		Box::new(
        			true,
        			true,
        			true,
        			0.0,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			-1)
        			); 	
        }
        
        if neg_xbox_right.update(mouse) {
        	neg_xbox_right_vec.push(
        		Box::new(
        			true,
        			false,
        			false,
        			0.0,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			-1)
        			); 	
        }
        
        if ones_left_remove.update(mouse) { ones_vec.pop(); }
        if ones_right_remove.update(mouse) { ones_right_vec.pop(); }
        
        if neg_ones_left_remove.update(mouse) { neg_ones_vec.pop(); }
        if neg_ones_right_remove.update(mouse) { neg_ones_right_vec.pop(); }
        
        if xbox_left_remove.update(mouse) { xbox_vec.pop(); }
        if xbox_right_remove.update(mouse) { xbox_right_vec.pop(); }
        
        if neg_xbox_left_remove.update(mouse) { neg_xbox_left_vec.pop(); }
        if neg_xbox_right_remove.update(mouse) { neg_xbox_right_vec.pop(); }
        
        // Handling the numbers
        let mut left_boxes = 0;
		let (vec1, vec2, i) = update_box(
							ones_vec,
							ones_right_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							false,
							Ones_start,
							1	
						);
		ones_vec = vec1;
		ones_right_vec = vec2;
		left_boxes = i;				        
        
        let mut right_boxes = 0;
        let (vec1, vec2, i) = update_box(
							ones_right_vec,
							ones_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							true,
							Ones_start,
							1,	
						);
		ones_right_vec = vec1;
		ones_vec = vec2;
		right_boxes = i;
		
		let mut neg_left_boxes = 0;
        let (vec1, vec2, i) = update_box(
							neg_ones_vec,
							neg_ones_right_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							false,
							Ones_start,
							-1,	
						);
		neg_ones_vec = vec1;
		neg_ones_right_vec = vec2;
		neg_left_boxes = i;	
		
		let mut neg_right_boxes = 0;
        let (vec1, vec2, i) = update_box(
							neg_ones_right_vec,
							neg_ones_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							true,
							Ones_start,
							-1,	
						);
		neg_ones_right_vec = vec1;
		neg_ones_vec = vec2;
		neg_right_boxes = i;
		
		let mut left_xboxes = 0;
        let (vec1, vec2, i) = update_box(
							xbox_vec,
							xbox_right_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							true,
							false,
							0.,
							1,	
						);
		xbox_vec = vec1;
		xbox_right_vec = vec2;
		left_xboxes = i;
		
		let mut right_xboxes = 0;
        let (vec1, vec2, i) = update_box(
							xbox_right_vec,
							xbox_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							true,
							true,
							0.0,
							1,	
						);
		xbox_right_vec = vec1;
		xbox_vec = vec2;
		right_xboxes = i;
		
		let mut neg_left_xboxes = 0;
        let (vec1, vec2, i) = update_box(
							neg_xbox_left_vec,
							neg_xbox_right_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							true,
							false,
							0.,
							-1,	
						);
		neg_xbox_left_vec = vec1;
		neg_xbox_right_vec = vec2;
		neg_left_xboxes = i;
		
		let mut neg_right_xboxes = 0;
        let (vec1, vec2, i) = update_box(
							neg_xbox_right_vec,
							neg_xbox_left_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							true,
							true,
							0.,
							-1,	
						);
		neg_xbox_right_vec = vec1;
		neg_xbox_left_vec = vec2;
		neg_right_xboxes = i;
		
        balance = left_boxes - right_boxes 
        			- neg_left_boxes + neg_right_boxes 
        			+ left_xboxes - right_xboxes
        			- neg_left_xboxes + neg_right_xboxes;
    	
    	scale.update(balance);
        //thread::sleep(time::Duration::from_millis(500));
        next_frame().await;
    }
}
