
extern crate rand;

use mathis_engine::*;
use scene_templates::*;
use piston_window::*;
use graphics::math::*;
use rand::*;
use rand::distributions::{IndependentSample, Range};

pub struct MathisApp {
    pub engine : MathisEngine,
    pub frame : i32,
    pub drag : bool,
    pub last_pos : [f64;2],
    pub view_origin : [f64;2],
	pub leave_traces : bool,
	pub show_influence : bool,
	pub follow_object: bool,
	pub target_object_idx: usize,
}

impl MathisApp {

    pub fn new_from_engine(the_engine : MathisEngine) -> MathisApp {
        MathisApp { 
			engine: the_engine, 
			frame: 1, 
			drag: false, 
			last_pos: [-10000.0,-4000.0], 
			view_origin: [-10000.0,-4000.0], 
			leave_traces: false,
			show_influence: false,
			follow_object: true,
			target_object_idx: 0
		}
    }

    pub fn create_window(&self, w_h: [u32;2]) -> PistonWindow {
        WindowSettings::new(format!("Orbiter v{}", ::VERSION), w_h)
            .exit_on_esc(true).build().unwrap()
    }

    pub fn default_scene(&mut self) {

    	//single_star(&mut self.engine);
    	//build_planets_scene(&mut self.engine);
    	build_twin_stars(&mut self.engine, &[-40000.0, 0.0], &[0.0, -8.0]);
		build_twin_stars(&mut self.engine, &[40000.0, 0.0], &[0.0, 8.0]);
    	//build_planets_scene(&mut self.engine);
    	//some_particles(&mut self.engine, &250.0, &15.0);
		//some_particles(&mut self.engine, &4.5, &8.0);
		some_particles(&mut self.engine, &25, &20.1, &12.5, &[50000.0, 50000.0], &[-40000.0, 0.0]);
		some_particles(&mut self.engine, &25, &20.1, &12.5, &[50000.0, 50000.0], &[40000.0, 0.0]);
    	//lots_of_particles_close(&mut self.engine);

    	//generate_grid(&mut self.engine, &[-50750,-2000,50750,2000], &[6850,1900], &1245.1, &1.0);

    	//engine.objects[0].enable_accel = false;

        //generate_grid(&mut self.engine, &[-60000,-60000,60000,60000], &[19000,19000], &2.2);


		//generate_grid(&mut self.engine, &[-10000,-10000,10000,10000], &[4000,4000], &10.5);

		//  // generate_grid(&mut self.engine, &[-8761,-9213,8210,9712], &[3042,3117], &0.05);

        //build_prot_elec(&mut self.engine);

    	//generate_grid(&mut self.engine, &[-5000,-5000,5000,5000], &[1110,1110], &10.3);
    	 //generate_grid(&mut self.engine, &[-1000,-100,1001,1001], &[500,500], &70.5);

    }

	pub fn constrain_target_index(&mut self) {
		self.target_object_idx = self.target_object_idx % self.engine.objects.len();
	}

	pub fn center_on_object(object_loc: [f64;2]) -> [f64;2] {
		//if self.engine.objects.len() == 0 {
		//	panic!("Engine has no objects to center on");
		//	return;
		//}
		//self.constrain_target_index();
		//println!("Center: target object index {}", self.target_object_idx);

		//let ref target_obj = self.engine.objects[self.target_object_idx];
		let mut new_position = object_loc;
		new_position[0] = new_position[0];
		new_position[1] = new_position[1];
		return new_position;
	}

    pub fn event_loop(&mut self, window : PistonWindow) {
        //let window = self.window;
        //let mut window = window.unwrap();
        let mut wait_for_first_coord = true;
        let mut scale = 0.05; //0.7;
        let mut screen_width = 0.0_64;
        let mut screen_height = 0.0_64;

        for e in window {
    		//e.input()
            {
                let s_frame = self.frame;
                let s_engine = &self.engine;
                e.draw_2d(|c, g| {
					// Get new view origin if following

                    MathisApp::render(self.view_origin, s_frame, s_engine, c,g, scale, !self.leave_traces, self.show_influence);
                    let vp_rect = c.viewport.unwrap().rect;
                    screen_width = (vp_rect[2] - vp_rect[0]) as f64;
                    screen_height = (vp_rect[3] - vp_rect[1]) as f64;
                    //println!("Screen size: {}, {}", screen_width, screen_height);
                });
            }


            // When an update event is sent, advance the engine forward a frame
            if let Some(_) = e.update_args() {
                self.engine.advance();
                self.frame += 1;

				// After event loop handle follow per frame
				if self.follow_object {
					let new_offset = MathisApp::center_on_object(self.engine.objects[self.target_object_idx].position);
					//new_offset[0] -= 
					self.view_origin[0] = new_offset[0];
					self.view_origin[1] = new_offset[1];
				}

            }

            // Handle button press events
    		if let Some(button) = e.press_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    self.drag = true;
                    wait_for_first_coord = true;
                }
				// Toggle traces (not clearing screen on render)
				else if button == Button::Keyboard(Key::T) {
					self.leave_traces = !self.leave_traces;
				}
				// Toggle Influence lines
				else if button == Button::Keyboard(Key::I) {
					self.show_influence = !self.show_influence;
				}
				// Toggle Influence lines
				else if button == Button::Keyboard(Key::F) {
					self.follow_object = !self.follow_object;

					self.constrain_target_index();
					if self.follow_object {
						println!("Following: Target object index {}", self.target_object_idx);
					}
					else {
						println!("Not following");
					}
					
				}
				// Toggle Influence lines
				else if button == Button::Keyboard(Key::Comma) || button == Button::Keyboard(Key::Period) {
					if button == Button::Keyboard(Key::Comma) {
						if self.target_object_idx > 0 {
							self.target_object_idx -= 1;
						}
						else {
							self.target_object_idx = self.engine.objects.len() - 1;
						}
					}
					else {
						self.target_object_idx += 1;
					}

					self.constrain_target_index();
					println!("New target object index {}", self.target_object_idx);
					
				}
				// Adjust engine tick rate
                else if button == Button::Keyboard(Key::PageDown) || button == Button::Keyboard(Key::PageUp) {
                    if button == Button::Keyboard(Key::PageUp) {
                        self.engine.tick_rate *= 1.05;
                    }
                    else {
                        self.engine.tick_rate /= 1.05;
                    }
                    println!("New tick rate {}", self.engine.tick_rate);
                }
				// Adjust Gravity and Speed of light parameters
				else if button == Button::Keyboard(Key::P) || button == Button::Keyboard(Key::O) 
					|| button == Button::Keyboard(Key::K) || button == Button::Keyboard(Key::L) {
					let mut new_grav = self.engine.G;
					let mut new_speed = self.engine.C;
                    if button == Button::Keyboard(Key::P) {
						new_grav *= 1.05;
					}
					else if button == Button::Keyboard(Key::O) {
						new_grav /= 1.05;
					}
					else if button == Button::Keyboard(Key::K) {
						new_speed *= 1.05;
					}
					else if button == Button::Keyboard(Key::L) {
						new_speed /= 1.05;
					}
					self.engine.set_g_c(new_grav, new_speed);
				}
            };
            // Handle button release events
            if let Some(button) = e.release_args() {
                if button == Button::Mouse(MouseButton::Left) {
                    self.drag = false;
                }
            };
            // Handle scroll wheel events
    		if let Some(scroll) = e.mouse_scroll_args() {
                let orig_scale = scale;
    			// Downward scrolling - increase scale
                if scroll[1] > 0.0 {
                    let curr_origin = self.view_origin;
					// Increase scale by a percentage
                    scale *= 1.05;
                    let scale_ratio = scale / orig_scale;
					// Correct view origin to keep same location (only poorly works now TODO: make really work to center on mouse or center of viewport)
                    self.view_origin[0] = curr_origin[0] + (self.last_pos[0]) * (1.0  / orig_scale - 1.0 / scale);
                    self.view_origin[1] = curr_origin[1] + (self.last_pos[1]) * (1.0  / orig_scale - 1.0 / scale);
                    /*self.view_origin[0] = curr_origin[0] - 0.5 * screen_width * (scale_ratio);
                    self.view_origin[1] = curr_origin[1] - 0.5 * screen_height * (scale_ratio);
                    println!("S0 = {}, X0 = {}, Y0 = {}, S1 = {}, W = {}, H = {}, S_ratio = {}, X1 = {}, Y1 = {}",
                        orig_scale, curr_origin[0], curr_origin[1], scale, screen_width, screen_height, scale_ratio, self.view_origin[0], self.view_origin[1]);*/
                    //self.view_origin[0] = curr_origin[0] + (-self.last_pos[0] / (scale / 1.05)) + (self.last_pos[0] / scale);
                    //self.view_origin[1] = curr_origin[1] + (-self.last_pos[1] / (scale / 1.05)) + (self.last_pos[1] / scale);
                }
                // Upward scrolling - decrease scale
                else if scroll[1] < 0.0 {
                    let curr_origin = self.view_origin;
					// Increase scale by a percentage
                    scale /= 1.05;
                    let scale_ratio = scale / orig_scale;
					// Correct view origin to keep same location (only poorly works now TODO: make really work to center on mouse or center of viewport)
                    self.view_origin[0] = curr_origin[0] + (self.last_pos[0]) * (1.0  / orig_scale - 1.0 / scale);
                    self.view_origin[1] = curr_origin[1] + (self.last_pos[1]) * (1.0  / orig_scale - 1.0 / scale);
                    /*self.view_origin[0] = curr_origin[0] - 0.5 * screen_width * (scale_ratio);
                    self.view_origin[1] = curr_origin[1] - 0.5 * screen_height * (scale_ratio);
                    println!("S0 = {}, X0 = {}, Y0 = {}, S1 = {}, W = {}, H = {}, S_ratio = {}, X1 = {}, Y1 = {}",
                        orig_scale, curr_origin[0], curr_origin[1], scale, screen_width, screen_height, scale_ratio, self.view_origin[0], self.view_origin[1]);*/
                    //self.view_origin[0] = curr_origin[0] + (-self.last_pos[0] / (scale * 1.05)) + (self.last_pos[0] / scale);
                    //self.view_origin[1] = curr_origin[1] + (-self.last_pos[1] / (scale * 1.05)) + (self.last_pos[1] / scale);
                }
    		}
            // Handle mouse move events to capture coordinates when dragging
			if let Some(pos) = e.mouse_cursor_args() {
                //let pos : [f64;2] = pos;
                if self.drag {
                    if !wait_for_first_coord {
                        let diff_x = pos[0] - self.last_pos[0];
                        let diff_y = pos[1] - self.last_pos[1];
                        self.view_origin[0] -= diff_x / scale;
                        self.view_origin[1] -= diff_y / scale;
                    }
                    else {
                        wait_for_first_coord = false;
                    }
    			}
                self.last_pos = pos;
            }
        }

        println!("Exited event loop");
    }

    fn render(mut center_offset : [f64;2], frame: i32, engine : &MathisEngine, c : Context, g : &mut G2d, scale : f64,
		clear_display: bool, show_influence: bool ) {
            //let engine = &self.engine;
        	let vp_rect = c.viewport.unwrap().rect;
        	let screen_rect = [vp_rect[0] as f64, vp_rect[1] as f64, vp_rect[2] as f64, vp_rect[3] as f64];
        	let transform = c.transform.scale(scale,scale);

        	if clear_display {
        		rectangle( [0.0,0.0,0.0,1.0], screen_rect, c.transform, g);
        	}
			else {
				rectangle( [0.0,0.0,0.0,0.01], screen_rect, c.transform, g);
			}

        	//Decide if we should draw the background rectangle
        	{/* //if frame % 11 == 0
        		let unit_range = Range::<f32>::new(0.0, 1.0);
        		let mut rng = rand::thread_rng();

        		//let mut [x, y, width, height] = screen_rect;
        		let width = unit_range.ind_sample(&mut rng) as f64 * screen_rect[2] as f64;
        		let height = unit_range.ind_sample(&mut rng) as f64 * screen_rect[3] as f64;
        		let x = screen_rect[0] as f64 + unit_range.ind_sample(&mut rng) as f64 * screen_rect[2] as f64 - width * 0.5;
        		let y = screen_rect[1] + unit_range.ind_sample(&mut rng) as f64 * screen_rect[3] as f64 - height * 0.5;
        		rectangle([unit_range.ind_sample(&mut rng), unit_range.ind_sample(&mut rng), unit_range.ind_sample(&mut rng), unit_range.ind_sample(&mut rng) * 0.03], //color
        			[x, y, width, height], c.transform, g);
        	*/}



        	//Tell engine to renders its objects
        	//engine.render(c, g);
        	//let center_offset = [0.0_f64, 0.0_f64]; //engine.calc_center_of_mass(); //[0.0_f64, 0.0_f64]; //
        	/*if frame % 100 == 0 {
        		println!("Center of Mass {:?}", center_offset);
        		println!("Object count {:?} Min/Max Star Distance ({},{})", engine.objects.len(), engine.min_max_star_distance.0, engine.min_max_star_distance.1);
        	}*/
        	let offset_x = center_offset[0] - ((screen_rect[2] - screen_rect[0]) / 2.0 / scale);
        	let offset_y = center_offset[1] - ((screen_rect[3] - screen_rect[1]) / 2.0 / scale);

        	//rectangle( [0.0,0.0,0.0,0.9], screen_rect, c.transform, g);

        	// if self.frame % 25 == 1
        	// {
        	// 	rectangle( [0.0,0.0,0.0,0.5], screen_rect, c.transform, g)
        	// }
        	// else
        	// {
        	// 	rectangle( [0.0,0.0,0.0,0.01], screen_rect, c.transform, g)
        	// }

        	let mut rng = thread_rng();

        	for i in 0..engine.objects.len() {
        		let r = engine.objects[i].radius;
        		let obj = &engine.objects[i];

                // calculate a 5x velocity line and draw with center through object, double its width = 10x
                let vel_x5 = [obj.velocity[0] * 1.0, obj.velocity[1] * 1.0];
                let start_vel = [obj.position[0] + vel_x5[0] - offset_x, obj.position[1] + vel_x5[1] - offset_y];
                let end_vel = [obj.position[0] - vel_x5[0] - offset_x, obj.position[1] - vel_x5[1] - offset_y];

                line([0.5,0.5,0.5,0.5], 1.0 / scale, [start_vel[0], start_vel[1], end_vel[0], end_vel[1]], transform, g);

				// If there is a "most accelerated by" object id, show influence line
                if obj.max_accel_id > 0 && show_influence {
                    let mut obj_2_idx = 0;
                    for j in 0..engine.objects.len() {
                        if engine.objects[j].obj_id == obj.max_accel_id {
                            let obj2 = &engine.objects[j];

                            if obj.max_accel > 0.0 {
                                line([0.8,0.3,0.3,0.15], 1.0 / scale,
                                    [obj.position[0] - offset_x, obj.position[1] - offset_y,
                                    obj2.position[0] - offset_x, obj2.position[1] - offset_y], transform, g);
                            }
                            else {
                                line([0.2,0.4,0.8,0.05], 1.0 / scale,
                                    [obj.position[0] - offset_x, obj.position[1] - offset_y,
                                    obj2.position[0] - offset_x, obj2.position[1] - offset_y], transform, g);
                            }
                            break;
                        }
                    }
                }
                /*else {
                    line([0.2,0.5,0.2,0.02], 1.0 / scale,
                        [obj.position[0] - offset_x, obj.position[1] - offset_y,
                        obj2.position[0] - offset_x, obj2.position[1] - offset_y], transform, g);
                }*/


                let mut draw_r = r * 2.0;
                if draw_r * scale < 1.0 { draw_r = 0.8 / scale; }

        		//println!("Render object {}, radius {}", i, r);
                circle_arc(engine.objects[i].color, 1.0 / scale, //color, radius of line drawn
                    0.0, 6.29,
                    [engine.objects[i].position[0]  - (draw_r * 0.5) - offset_x,
                    engine.objects[i].position[1]  - (draw_r * 0.5) - offset_y, draw_r, draw_r], //rectangle
                    transform, //transform
                    g);
                /*
        		ellipse(engine.objects[i].color, //color
        			[engine.objects[i].position[0] - (draw_r * 0.5) - offset_x,
        			engine.objects[i].position[1] - (draw_r * 0.5) - offset_y, draw_r, draw_r], //rectangle
        			transform, //transform
        			g);
                */
        		// let mut j = 0;
        		//
        		// for other_obj in engine.objects.iter() {
        		// 	if j > i {
        		// 		let choice = choice_range.ind_sample(&mut rng);
        		// 		if choice > 0.9 {
        		// 			line([0.5,0.5,0.5,0.25], 1.0,
        		// 				[obj.position[0] - offset_x, obj.position[1] - offset_y, other_obj.position[0] - offset_x, other_obj.position[1] - offset_y,],
        		// 				transform, g);
        		// 		}
        		// 	}
        		// 	j += 1;
        		// }

                // If visible object radius is greater than x, draw the circle's eyes
        		if obj.radius * scale > 9.0 {
        			let color : [f32; 4] = [0.0,0.0,0.0,0.5];
        			let r_x = r * 0.17;
        			let r_y = r * 0.40;
        			let pos_x_left = obj.position[0] - r * 0.3;
        			let pos_y = obj.position[1] + r * 0.1;
        			let pos_x_right = obj.position[0] + r * 0.3;

        			let unit_range = Range::<f32>::new(0.0, 1.0);
        			if unit_range.ind_sample(&mut rng) < 0.05 {
        				line([1.0,1.0,1.0,0.75], //color
        				1.5 / scale, //radius
        				[pos_x_left - r_x - offset_x,
        				pos_y - offset_y,
        				pos_x_left + r_x - offset_x,
        				pos_y - offset_y,], //line
        				transform, //transform
        				g);

        				line([1.0,1.0,1.0,0.75], //color
        				1.5 / scale, //radius
        				[pos_x_right - r_x - offset_x,
        				pos_y - offset_y,
        				pos_x_right + r_x - offset_x,
        				pos_y - offset_y,], //line
        				transform, //transform
        				g);
        			}
        			else {
                        circle_arc([1.0,1.0,1.0,0.75], 1.0 / scale, //color, radius of line drawn
                            0.0, 6.29,
                            [pos_x_left - r_x - offset_x,
            				pos_y - r_y - offset_y, r_x*2.0, r_y*2.0], //rectangle
                            transform, //transform
                            g);

        				/*ellipse(color, //color
        				[pos_x_left - r_x - offset_x,
        				pos_y - r_y - offset_y, r_x*2.0, r_y*2.0], //rectangle
        				transform, //transform
        				g);*/

                        circle_arc([1.0,1.0,1.0,0.75], 1.0 / scale, //color, radius of line drawn
                            0.0, 6.29,
                            [pos_x_right - r_x - offset_x,
            				pos_y - r_y - offset_y, r_x*2.0, r_y*2.0], //rectangle
                            transform, //transform
                            g);

        				/*ellipse(color, //color
        				[pos_x_right - r_x - offset_x,
        				pos_y - r_y - offset_y, r_x*2.0, r_y*2.0], //rectangle
        				transform, //transform
        				g);*/
        			}
        		}
        	}

            // Draw a fixed grid
            /*
        	for x in (-1000..1001).filter(|a| a % 250 == 0) {
        		for y in (-1000..1001).filter(|a| a % 250 == 0) {
        			//println!("grid point at {},{}", x, y);
        			line([0.3_f32,0.3_f32,0.3_f32,0.3_f32], 1.0, [(x as f64) - offset_x, (y as f64) - offset_y, (x as f64) - offset_x + 100.0, (y as f64) - offset_y], transform, g);
        			line([0.3_f32,0.3_f32,0.3_f32,0.3_f32], 1.0, [(x as f64) - offset_x, (y as f64) - offset_y, (x as f64) - offset_x, (y as f64)  - offset_y + 100.0], transform, g);
        		}
        	}*/

        	// if self.frame % 1000 == 0 {
        	// 	engine.min_max_star_distance = (1_000_000.0, 0.0);
        	// }



        	//self.frame += 1;
        	/*
        	let mut x_total : f64 = 0.0;
        	let mut y_total : f64 = 0.0;
        	//let mut x_min_max : [f64; 2] = [MAX, MIN];
        	//let mut y_min_max : [f64; 2] = [MAX, MIN];
        	let mut mass_total = 0.0;
        	for i in 0..size {
        		x_total += mass[i] * position[i*2]; //mass[i] *
        		y_total += mass[i] * position[i*2+1]; //mass[i] *
        		mass_total += mass[i];
        		/ *  if mass[i] > 1.0 {
        			//mass_total += 1;
        			if position[i*2] < x_min_max[0] { x_min_max[0] = position[i*2] }
        			else if position[i*2] > x_min_max[1] { x_min_max[1] = position[i*2] }
        			if position[i*2+1] < y_min_max[0] { y_min_max[0] = position[i*2+1] }
        			else if position[i*2+1] > y_min_max[1] { y_min_max[1] = position[i*2+1] }
        		} * /
        	}
        	/ *
        	let mut x_span = x_min_max[1] - x_min_max[0];
        	let mut y_span = y_min_max[1] - y_min_max[0];
        	//get ratio to multiply radius and position by to fit within screen
        	x_span = 0.5 * 1680.0 / x_span.max(1.0);
        	y_span = 1.0 * 1050.0 / y_span.max(1.0);
        	* /
        	let biggest_span_ratio = 1.0; //x_span.min(y_span);
        	//x_total = (x_min_max[1] - x_min_max[0]) / 2.0;
        	//y_total = (y_min_max[1] - y_min_max[0]) / 2.0;  //y_total / mass_total as f64;// / mass_total as f64;
        	//x_total *= biggest_span_ratio;
        	//y_total *= biggest_span_ratio;
        	x_total = x_total / mass_total;
        	y_total = y_total / mass_total;

        	for i in 0..size {
        		let r = radius[i] * biggest_span_ratio;
        		ellipse(color[i], //color
        			[position[i*2] * biggest_span_ratio - r - x_total + 840.0,
        			position[i*2+1] * biggest_span_ratio - r - y_total + 525.0, r*2.0, r*2.0], //rectangle
        			c.transform, //transform
        			g);
        	}

        	if frame % 100 == 0 {
        		//println!("Draw State {:?}", c.draw_state);
        	}*/
    }
}
