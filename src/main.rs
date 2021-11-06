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
const BOX_SIZE: f32 = 22.0;
const XBOX_SIZE: f32 = 25.0;
const ONES_COLOR: Color = Color::new(0.0, 0.6, 1.0, 1.0);
const NEG_ONES_COLOR: Color = Color::new(0.0, 1.0, 1.0, 1.0);
const XBOX_COLOR: Color = Color::new(0.2, 0.8, 0.2, 1.0);

fn window_conf() -> Conf {
    Conf {
        window_title: "Equations".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        high_dpi: true,
        ..Default::default()
    }
}

pub const WINDOW_WIDTH: usize = 1600;
pub const WINDOW_HEIGHT: usize = 1200;
pub const ORIGO: [f32; 2] = [50.0,50.0];

#[macroquad::main(window_conf)]
async fn main() {
	
	let fontfile = load_ttf_font_from_bytes(include_bytes!("OpenSans.ttf"));
	let font = fontfile.unwrap();
	let text_params_scale = TextParams {
		font: font,
		font_size: 40,
		font_scale: 1.0,
		font_scale_aspect: 1.0,
		color: BLACK
		};
	
	let text_params_menu = TextParams {
		font: font,
		font_size: 100,
		font_scale: 1.0,
		font_scale_aspect: 1.0,
		color: BLACK
		};
	let text_params_menutext = TextParams {
		font: font,
		font_size: 50,
		font_scale: 1.0,
		font_scale_aspect: 1.0,
		color: BLACK
		};				
	
	const ONES_START: f32 = RECTANGLE[0]/2.;
	let mut xval = 0;
	let mut balance = 0;
	let mut left_boxes = 0;
    let mut right_boxes = 0;
    let mut neg_right_boxes = 0;
    let mut neg_left_boxes = 0;
    let mut left_xboxes = 0;
    let mut right_xboxes = 0;
	
	let mut scale = Scale::new();
	
	let btn_dif = 10.0;
	let btn_x = 200.0;
	let btn_y = 700.0;
	let btn_dy = 100.0;
	
	let btn_x_right = 1200.0;
	
	// BUTTONS	
	let mut ones_left = OnesButton::new(btn_x, btn_y, ONES_COLOR, -1, true);
	let mut ones_left_remove = OnesButton::new(btn_x, btn_y+btn_dif, ONES_COLOR, 1, true);
	let mut ones_right = OnesButton::new(btn_x_right, btn_y, ONES_COLOR, -1, false);
	let mut ones_right_remove = OnesButton::new(btn_x_right, btn_y+btn_dif, ONES_COLOR, 1, false);	
	
	let mut neg_ones_left = OnesButton::new(btn_x, btn_y + btn_dy, NEG_ONES_COLOR, -1, true);
	let mut neg_ones_left_remove = OnesButton::new(btn_x, btn_y + btn_dy+btn_dif, NEG_ONES_COLOR, 1, true);
	let mut neg_ones_right = OnesButton::new(btn_x_right, btn_y + btn_dy, NEG_ONES_COLOR, -1, false);
	let mut neg_ones_right_remove = OnesButton::new(btn_x_right, btn_y + btn_dy+btn_dif, NEG_ONES_COLOR, 1, false);
	
	let mut xbox_left = OnesButton::new(btn_x, btn_y + 2.0*btn_dy, XBOX_COLOR, -1, true);
	let mut xbox_left_remove = OnesButton::new(btn_x, btn_y + 2.0*btn_dy+btn_dif, XBOX_COLOR, 1, true);
	let mut xbox_right = OnesButton::new(btn_x_right, btn_y + 2.0*btn_dy, XBOX_COLOR, -1, false);
	let mut xbox_right_remove = OnesButton::new(btn_x_right, btn_y + 2.0*btn_dy+btn_dif, XBOX_COLOR, 1, false);
	
	let mut ones_vec: Vec<Box> = Vec::new();
	let mut ones_right_vec: Vec<Box> = Vec::new();
	
	let mut neg_ones_vec: Vec<Box> = Vec::new();
	let mut neg_ones_right_vec: Vec<Box> = Vec::new();
	
	let mut xbox_vec: Vec<Box> = Vec::new();
	let mut xbox_right_vec: Vec<Box> = Vec::new();
	


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
	let mut menu = true;
	let mut draw_line = false;
	
	loop {
	
	let (x, y) = mouse_position();
    let mouse = Point::new(x, y);
	
	if menu {
		clear_background(WHITE);
		let s = 200.0;
		let x = 50.0;
		let y = 50.0;
		let a = Coordinate { x: x, y: y};
		let b = a + Coordinate { x: 2.0*s, y: 0.0 };
		let c = b + Coordinate { x: 0.0, y: s };
		let d = a + Coordinate { x: 0.0, y: s };
		let start_ls = LineString(vec![a, b, c, d, a]);
		
		let start_poly = Polygon::new(
    		start_ls,
    		vec![],
		);
		
		draw_rectangle(x, y, 2.0*s, s, ONES_COLOR);
		draw_text_ex("START", 110.0, 175.0, text_params_menu);
		draw_text_ex("If x is set to zero, x is not present.", 700.0, 625.0, text_params_menutext);
		
		let s = 200.0;
		let x = 50.0;
		let y = 500.0;
		
		
		draw_rectangle(x, y, 2.0*s, s, ONES_COLOR);
		
		let x = 470.0;
		let a = Coordinate { x: x, y: y};
		let b = a + Coordinate { x: 2.0*s, y: 0.0 };
		let c = b + Coordinate { x: 0.0, y: s };
		let d = a + Coordinate { x: 0.0, y: s };
		
		let xbox_val_ls = LineString(vec![a, b, c, d, a]);
		
		draw_rectangle(x, y, s, s, ONES_COLOR);
		
		let xbox_val_poly = Polygon::new(
    		xbox_val_ls,
    		vec![],
		);
		
		if xbox_val_poly.contains(&mouse) && is_mouse_button_pressed(MouseButton::Left) {
			draw_line = true;
		}
		
		if draw_line{ draw_rectangle_lines(470.0, 500.0, s, s, 10.0, BLACK); }
		if xbox_val_poly.contains(&mouse) == false 
			&& is_mouse_button_pressed(MouseButton::Left) { 
			draw_line = false;
		}
		
		if draw_line {
			if is_key_pressed(KeyCode::Key0) { xval = 0i8; }
			if is_key_pressed(KeyCode::Key1) { xval = 1i8; }
			if is_key_pressed(KeyCode::Key2) { xval = 2i8; }
			if is_key_pressed(KeyCode::Key3) { xval = 3i8; }
			if is_key_pressed(KeyCode::Key4) { xval = 4i8; }
			if is_key_pressed(KeyCode::Key5) { xval = 5i8; }
			if is_key_pressed(KeyCode::Key6) { xval = 6i8; }
			if is_key_pressed(KeyCode::Key7) { xval = 7i8; }
			if is_key_pressed(KeyCode::Key8) { xval = 8i8; }
			if is_key_pressed(KeyCode::Key9) { xval = 9i8; }
		}
		
		draw_text_ex(&format!("{}", xval), 545.0, 625.0, text_params_menu);
		draw_text_ex("x-value", 90.0, 625.0, text_params_menu);
			
		if start_poly.contains(&mouse) && is_mouse_button_pressed(MouseButton::Left) {
			menu = false;
		}
		
	}
	
	if menu == false {
    	clear_background(WHITE);
    	
    	scale.render(
    		balance, 
    		left_boxes,
    		neg_left_boxes,
    		left_xboxes,
    		right_boxes,
    		neg_right_boxes,
    		right_xboxes,
    		text_params_scale
    		);
        
        
        // make buttons
        ones_left.render();
        ones_left_remove.render();
        
        ones_right.render();
        ones_right_remove.render();
        
        neg_ones_left.render();
        neg_ones_left_remove.render();
        
        neg_ones_right.render();
        neg_ones_right_remove.render();
        
        if xval != 0 {
        xbox_left.render();
        xbox_left_remove.render();
        xbox_right.render();
        xbox_right_remove.render();
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
        
        if xbox_left_remove.update(mouse) { xbox_vec.pop(); }
        if xbox_right_remove.update(mouse) { xbox_right_vec.pop(); }
        
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
		}
       
        
        if ones_left.update(mouse) {
        	ones_vec.push(
        		Box::new(
        		false,
        		true,
        		true,
        		ONES_START, 
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
        		ONES_START, 
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
        			ONES_START,
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
        			ONES_START,
        			scale.get_c().x, 
        			scale.get_c().y, 
        			-1)
        			); 	
        }
        
        if ones_left_remove.update(mouse) { ones_vec.pop(); }
        if ones_right_remove.update(mouse) { ones_right_vec.pop(); }
        
        if neg_ones_left_remove.update(mouse) { neg_ones_vec.pop(); }
        if neg_ones_right_remove.update(mouse) { neg_ones_right_vec.pop(); }
        
     
        
    
        
		let (vec1, vec2, i) = update_box(
							ones_vec,
							ones_right_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							false,
							ONES_START,
							1	
						);
		ones_vec = vec1;
		ones_right_vec = vec2;
		left_boxes = i;				        
        
        
        let (vec1, vec2, i) = update_box(
							ones_right_vec,
							ones_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							true,
							ONES_START,
							1,	
						);
		ones_right_vec = vec1;
		ones_vec = vec2;
		right_boxes = i;
		
		
        let (vec1, vec2, i) = update_box(
							neg_ones_vec,
							neg_ones_right_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							false,
							ONES_START,
							-1,	
						);
		neg_ones_vec = vec1;
		neg_ones_right_vec = vec2;
		neg_left_boxes = i;	
		
		
        let (vec1, vec2, i) = update_box(
							neg_ones_right_vec,
							neg_ones_vec,
							scale.get_c(),
							scale.get_c_right(),
							mouse,
							false,
							true,
							ONES_START,
							-1,	
						);
		neg_ones_right_vec = vec1;
		neg_ones_vec = vec2;
		neg_right_boxes = i;
		
		
        balance = left_boxes - right_boxes 
        			- neg_left_boxes + neg_right_boxes 
        			+ xval*(left_xboxes - right_xboxes);
        			//- neg_left_xboxes + neg_right_xboxes;
    	
    	// Handling the scale
    	scale.update( balance);
     }   
        next_frame().await;
    }
}
