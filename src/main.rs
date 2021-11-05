mod scale;
mod boxes;

mod prelude{
	pub use macroquad::prelude::*;
	pub use geo::{Point, Coordinate,LineString, Polygon, Triangle};
	pub use geo::algorithm::contains::Contains;
	pub use geo::algorithm::translate::Translate;
	pub use geo::algorithm::rotate::RotatePoint;
	pub use mathrs::trigonometry::{cos, sin};
	pub use std::f64::consts::PI;
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
	const ones_start: f32 = RECTANGLE[0]/2.;
	let xval = 4i8;
	let mut balance = 0;
	
	let mut scale = Scale::new();
	
	let btn_dif = 10.0;
	let btn_x = 50.0;
	let btn_y = 400.0;
	let btn_dy = 100.0;
	
	let btn_x_right = 600.0;
		
	let mut ones_left = OnesButton::new(btn_x, btn_y, BLUE, -1);
	let mut ones_left_remove = OnesButton::new(btn_x, btn_y+btn_dif, BLUE, 1);
	/*
	let mut neg_ones_left = OnesButton::new(btn_x, btn_y + btn_dy, LIME, -1);
	let mut neg_ones_left_remove = OnesButton::new(btn_x, btn_y + btn_dy+btn_dif, LIME, 1);
	let mut xbox_left = OnesButton::new(btn_x, btn_y + 2.0*btn_dy, GRAY, -1);
	let mut xbox_left_remove = OnesButton::new(btn_x, btn_y + 2.0*btn_dy+btn_dif, GRAY, 1);
	let mut neg_xbox_left = OnesButton::new(btn_x, btn_y + 3.0*btn_dy, GRAY, -1);
	let mut neg_xbox_left_remove = OnesButton::new(
		btn_x, btn_y + 3.0*btn_dy+btn_dif, GRAY, 1);
	
	*/	
	let mut ones_right = OnesButton::new(btn_x_right, btn_y, BLUE, -1);
	let mut ones_right_remove = OnesButton::new(btn_x_right, btn_y+btn_dif, BLUE, 1);	
	
	let mut ones_vec: Vec<Box> = Vec::new();
	//let mut xbox_vec: Vec<Box> = Vec::new();
	//let mut neg_ones_vec: Vec<Box> = Vec::new();
	//let mut neg_xbox_vec: Vec<Box> = Vec::new();
	
	let mut ones_right_vec: Vec<Box> = Vec::new();
	
	loop {
    	clear_background(WHITE);
    	
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
        /*
        neg_ones_left_remove.render();
        neg_ones_left.render();
        xbox_left.render();
        xbox_left_remove.render();
        neg_xbox_left.render();
        neg_xbox_left_remove.render();
        */
		// make right buttons        
        ones_right.render();
        ones_right_remove.render();
        
        if ones_left.update(mouse) {
        	ones_vec.push(
        		Box::new(
        		false,
        		true,
        		true,
        		ones_start, 
        		scale.get_c().x, 
        		scale.get_c_right().y, 
        		1)
        		); 	
        }
        /*
        if neg_ones_left.update(mouse) {
        	neg_ones_vec.push(
        		Box::new(
        			false,
        			ones_start,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			-1)
        			); 	
        }
        if xbox_left.update(mouse) {
        	xbox_vec.push(
        		Box::new(
        			true,
        			0.,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			1)
        			); 	
        }
        
        if neg_xbox_left.update(mouse) {
        	neg_xbox_vec.push(
        		Box::new(
        			true,
        			0.,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			-1)
        			); 	
        }
        */
        if ones_right.update(mouse) {
        	ones_right_vec.push(
        		Box::new(
        		false,
        		false,
        		false,
        		ones_start, 
        		scale.get_c_right().x, 
        		scale.get_c_right().y, 
        		1)
        		); 	
        }
        
        if ones_left_remove.update(mouse) { ones_vec.pop(); }
        //if neg_ones_left_remove.update(mouse) { neg_ones_vec.pop(); }
        //if xbox_left_remove.update(mouse) { xbox_vec.pop(); }
        //if neg_xbox_left_remove.update(mouse) { neg_xbox_vec.pop(); }
        if ones_right_remove.update(mouse) { ones_right_vec.pop(); }
        
        fn update_box(
        	mut box1: Vec<Box>,
        	mut box2: Vec<Box>, 
        	c: Coordinate<f32>, 
        	c_r: Coordinate<f32>,
        	mouse: Point<f32>
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
				boxes += cnt;
				if change{ change_index = Some(i); }
				else if box1[i].get_y() > 900.0 { out_index = Some(i) }
				else { box1[i].render(); }	
		    }
		    if change_index.is_some() {
		    	box1.swap_remove(change_index.unwrap());
		    	box2.push(
		    		Box::new(
		    		false,
		    		false,
		    		false,
		    		ones_start, 
		    		c.x, 
		    		c.y, 
		    		1)
		    		);  	
		    }
		    if out_index.is_some() { box1.swap_remove(out_index.unwrap()); }
		    (box1, box2, boxes)
        }
        
        // Handling the numbers
        let mut left_boxes = 0;
		let (vec1, vec2, i) = update_box(
							ones_vec,
							ones_right_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse	
						);
		ones_vec = vec1;
		ones_right_vec = vec2;
		left_boxes = i;				        
        /*
        let mut left_neg_boxes = 0;
        for i in 0..neg_ones_vec.len() {
		    if neg_ones_vec[i].contains_mouse(mouse) { 
		    	draw_text("INSIDE", 400.0, 600.0, 20.0, BLACK); 
		    }
		    left_neg_boxes += neg_ones_vec[i].update(
		    	mouse, 
		    	scale.get_c(), 
		    	left_neg_boxes);
		    neg_ones_vec[i].render();	
        }
        
        let mut left_xboxes = 0;
        for i in 0..xbox_vec.len() {
		    if xbox_vec[i].contains_mouse(mouse) { 
		    	draw_text("INSIDE", 400.0, 100.0, 20.0, BLACK); 
		    }
		    left_xboxes += xbox_vec[i].update(
		    	mouse, 
		    	scale.get_c(), 
		    	left_xboxes);
		    xbox_vec[i].render();	
        }
        
        let mut left_neg_xboxes = 0;
        for i in 0..neg_xbox_vec.len() {
		    if neg_xbox_vec[i].contains_mouse(mouse) { 
		    	draw_text("INSIDE", 400.0, 100.0, 20.0, BLACK); 
		    }
		    left_neg_xboxes += neg_xbox_vec[i].update(
		    	mouse, 
		    	scale.get_c(), 
		    	left_neg_xboxes);
		    neg_xbox_vec[i].render();	
        }
        */
        let mut right_boxes = 0;
        let mut change_index: Option<usize> = None;
        for i in 0..ones_right_vec.len() {
		    if ones_right_vec[i].contains_mouse(mouse) { 
		    	draw_text("INSIDE", 400.0, 100.0, 20.0, BLACK); 
		    }
		    let (cnt, change) = ones_right_vec[i].update(
		    	mouse, 
		    	scale.get_c(),
		    	scale.get_c_right(),  
		    	right_boxes);
		    right_boxes += cnt;
		    if change{ change_index = Some(i) }
		    else { ones_right_vec[i].render(); }
        }
        if change_index.is_some() {
        	ones_right_vec.swap_remove(change_index.unwrap());
        	ones_vec.push(
        		Box::new(
        		false,
        		true,
        		true,
        		ones_start, 
        		scale.get_c().x, 
        		scale.get_c_right().y, 
        		1)
        		);  	
        }
        //balance = left_boxes+xval*left_xboxes-left_neg_boxes-xval*left_neg_xboxes;
        balance = left_boxes;
    	
    	// Handling the scale
        scale.update(balance);
    		
        next_frame().await;
    }
}
