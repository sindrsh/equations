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
	let ones_start = RECTANGLE[0]/2.;
	let xval = 4i8;
	let mut balance = 0;
	
	let mut scale = Scale::new();
	
	let btn_dif = 10.0;
	
	let mut xbox_left = OnesButton::new(50.0, 1000.0, GRAY, -1);
	let mut xbox_left_remove = OnesButton::new(50.0, 1000.0+btn_dif, GRAY, 1);
	
	let mut ones_left = OnesButton::new(50.0, 600.0, BLUE, -1);
	let mut ones_left_remove = OnesButton::new(50.0, 600.0+btn_dif, BLUE, 1);
	let mut neg_ones_left = OnesButton::new(50.0, 800.0, LIME, -1);
	let mut neg_ones_left_remove = OnesButton::new(50.0, 800.0+btn_dif, LIME, 1);
	let mut ones_vec: Vec<Box> = Vec::new();
	let mut xbox_vec: Vec<Box> = Vec::new();
	let mut neg_ones_vec: Vec<Box> = Vec::new();
	
	loop {
    	clear_background(WHITE);
    	
    	scale.render();
        
        // <writing mouse position on screen 
    	let (x, y) = mouse_position();
    	let mouse = Point::new(x, y);
    	let mouse_x_and_y = format!("({}, {})", x, y);
    	draw_text(&mouse_x_and_y, 100.0, 100.0, 20.0, BLACK); 
        draw_text(&balance.to_string(), 400.0, 50.0, 20.0, BLACK); 
        
        // make buttons
        ones_left.render();
        neg_ones_left.render();
        ones_left_remove.render();
        neg_ones_left_remove.render();
        xbox_left.render();
        xbox_left_remove.render();
        
        if ones_left.update(mouse) {
        	ones_vec.push(
        		Box::new(
        		false,
        		ones_start, 
        		scale.get_c(true).x, 
        		scale.get_c(true).y, 
        		1)
        		); 	
        }
        if neg_ones_left.update(mouse) {
        	neg_ones_vec.push(
        		Box::new(
        			false,
        			ones_start,
        			scale.get_c(false).x, 
        			scale.get_c(false).y, 
        			-1)
        			); 	
        }
        if xbox_left.update(mouse) {
        	xbox_vec.push(
        		Box::new(
        			true,
        			0.,
        			scale.get_c(false).x, 
        			scale.get_c(false).y, 
        			1)
        			); 	
        }
        if ones_left_remove.update(mouse) { ones_vec.pop(); }
        if neg_ones_left_remove.update(mouse) { neg_ones_vec.pop(); }
        if xbox_left_remove.update(mouse) { xbox_vec.pop(); }
        
        // Handling the numbers
        let mut left_boxes = 0;
        for i in 0..ones_vec.len() {
		    if ones_vec[i].contains_mouse(mouse) { 
		    	draw_text("INSIDE", 400.0, 100.0, 20.0, BLACK); 
		    }
		    left_boxes += ones_vec[i].update(
		    	mouse, 
		    	scale.get_c(true), 
		    	left_boxes);
		    ones_vec[i].render();	
        }
        
        let mut left_neg_boxes = 0;
        for i in 0..neg_ones_vec.len() {
		    if neg_ones_vec[i].contains_mouse(mouse) { 
		    	draw_text("INSIDE", 400.0, 600.0, 20.0, BLACK); 
		    }
		    left_neg_boxes -= neg_ones_vec[i].update(
		    	mouse, 
		    	scale.get_c(false), 
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
		    	scale.get_c(true), 
		    	left_xboxes);
		    xbox_vec[i].render();	
        }
        
        balance = left_boxes+xval*left_xboxes+left_neg_boxes;
        
    	// Handling the scale
        scale.update(balance);
    		
        next_frame().await;
    }
}
