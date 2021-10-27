mod scale;
mod boxes;

mod prelude{
	pub use macroquad::prelude::*;
	pub use geo::{Point, Coordinate,LineString, Polygon};
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
	
	let mut balance = 0;
	
	
	let mut scale = Scale::new();
	//let mut one1 = Ones::new();
	
	let mut ones_vec: Vec<Ones> = Vec::new();
	ones_vec.push(Ones::new(50.0,50.0));
	ones_vec.push(Ones::new(50.0,200.0)); 
	
	loop {
    	clear_background(WHITE);
        
        // <writing mouse position on screen 
    	let (x, y) = mouse_position();
    	let mouse_x_and_y = format!("({}, {})", x, y);
    	draw_text(&mouse_x_and_y, 100.0, 100.0, 20.0, BLACK); 
        draw_text(&balance.to_string(), 400.0, 50.0, 20.0, BLACK); 
        
        // Handling the scale
        scale.render();
        scale.update(balance);
        
        // Handling the numbers
        let mut left_boxes = 0;
        for i in 0..ones_vec.len() {
		    ones_vec[i].render();
		    if ones_vec[i].contains_mouse(mouse_position()) { 
		    	draw_text("INSIDE", 400.0, 100.0, 20.0, BLACK); 
		    }
		    left_boxes += ones_vec[i].update(
		    	mouse_position(), 
		    	scale.get_c(), 
		    	left_boxes);
        }
        balance = left_boxes;
    	
        next_frame().await;
    }
}
