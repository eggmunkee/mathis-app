extern crate piston_window;
extern crate vecmath;
extern crate graphics;
extern crate clock_ticks;
extern crate rand;
extern crate piston;

mod mathis_engine;
mod mathis_object;
mod mathis_app;
mod scene_templates;

use mathis_engine::*;
//use mathis_object::*;
use mathis_app::*;

use piston_window::PistonWindow;

//use std::io::prelude::*;
//use std::io::BufReader;
//use std::fs::File;

//let mut f = try!(File::open("log.txt"));
//let mut reader = BufReader::new(f);

//let mut line = String::new();
//let len = try!(reader.read_line(&mut line));
//println!("First line is {} bytes long", len);

// piston2d-graphics = { git = "https://github.com/PistonDevelopers/graphics.git" }

static VERSION : &'static str = "0.2";
//static C : Scalar = 10000.0;
//static MAX_DIST : Scalar = 100_000.0;
//static G : Scalar = 100.0;
//static PI : Scalar = 3.14159;

/*
// Repulsion field strength
// E = (GmM/R2 )(1 – 2R/ct ) – m(a + A + 2AR/ct )
fn funcE(G: Scalar, M: Scalar, m: Scalar, R: Scalar, A: Scalar, a: Scalar, t: Scalar) {
	(GmM/R2 ) * (1.0 – 2.0 * R / (C*t) ) – m * (a + A + 2.0 *A * R / (C*t) )
}

// "Attraction field strength
// H = m(a + A + 2AR/ct )
fn funcH(m: Scalar, R: Scalar, A: Scalar, a: Scalar, t: Scalar) {
	m * (a + A + 2.0 * A * R / (C * t))
}
*/

// Combined field Force
// F = (GmM/R2) – (2GmM/Rct )
/*
fn funcF(g: Scalar, M: Scalar, m: Scalar, R: Scalar, t: Scalar) -> Scalar {
	(g * m * M / R.powi(2)) - (2.0 * g * m * M / ( R * C * t ))
}

fn translate_vec(vec: Vec2d, vec2: Vec2d) -> Vec2d {
	//Return updated position
	[vec[0] + vec2[0], vec[1] + vec2[1]]
}

fn apply_velocity(pos: Vec2d, vel: Vec2d, t_delta: Scalar) -> Vec2d {
	//Return updated position
	[pos[0] + vel[0] * t_delta, pos[1] + vel[1] * t_delta]
}
*/


#[allow(non_snake_case)]
fn main() {
	//let between = Range::new(-PI, PI);
	//let mut rng = rand::thread_rng();

	//Create instance of mathis engine, set constants G and C for physics equations
	let mut engine : MathisEngine = MathisEngine::new_with_g_c(5000.0, 250000.0, 0.1);  //(1500.0, 2000.0);
	engine.distance_scale = 1.0; //Scale the distance between points which begin in pixels


	let mut app : MathisApp = MathisApp::new_from_engine(engine);
	let window : PistonWindow = app.create_window([1100,700]);
	app.default_scene();
	app.event_loop(window);

	//let mut pos1 : Vec2d = [400.0, 350.0];

	/*
	let mut position : [Scalar; 40] = [800.0, 550.0,
										350.0, 550.0,
										600.0, 550.0,
										600.0, 530.0,
										600.0, 500.0,
										1300.0, 550.0,
										1500.0, 550.0,
										1680.0 - 540.0, 590.0,
										850.0, 550.0,
										600.0, 300.0,
				910.0, 700.0, 940.0, 700.0, 900.0, 720.0, 900.0, 760.0,300.0, 700.0, 250.0, 700.0,350.0, 300.0,500.0, 300.0,200.0, 300.0, 600.0, 300.0,
	];
	let mass : [Scalar; 20] = [900.0, 400.0, 10.0, 400.0, 400.0, 400.0, 10.0, 10.0, 400.0, 400.0,
							10.0, 400.0, 400.0, 10.0, 400.0, 400.0, 10.0, 10.0, 10.0, 50.0,];
	let radius : [Scalar; 20] = [40.0,22.0, 6.0, 22.0, 22.0, 22.0, 6.0, 6.0, 22.0, 22.0,
							6.0, 22.0, 22.0, 6.0, 22.0, 22.0, 6.0, 6.0, 6.0, 8.0,];
	let color : [[f32; 4]; 20] = [[1.0,0.0,0.0,0.3],[0.5,0.5,0.0,0.6],[0.5,0.5,0.0,0.6],[0.5,0.5,0.0,0.6],[0.5,0.5,0.0,0.6],
									[0.2,1.0,0.6,0.3],[0.2,0.5,1.0,0.6],[0.5,0.0,0.5,0.6],[0.0,0.5,1.0,0.6],[1.0,0.5,0.4,0.3],
									[0.2,1.0,0.6,0.3],[0.2,0.5,1.0,0.6],[0.5,0.0,0.5,0.6],[0.0,0.5,1.0,0.6],[1.0,0.5,0.4,0.3],
									[0.2,1.0,0.6,0.3],[0.2,0.5,1.0,0.6],[0.5,0.0,0.5,0.6],[0.0,0.5,1.0,0.6],[1.0,0.5,0.4,0.3],];
	let mut velocity : [Vec2d; 20] = [[30.0, 0.0],[-30.0, 0.0],[20.0, 60.0],[60.0, -10.0],[-60.0, 0.0],
								[0.0, 88.0],[0.0, 90.0],[60.0, 0.0],[-60.0, 0.0],[0.0, 60.0],
								[0.0, 60.0],[20.0, -60.0],[0.0, 60.0],[0.0, 60.0],[0.0, 60.0],
								[0.0, 60.0],[0.0, 60.0],[0.0, 60.0],[0.0, 60.0],[-0.0, -20.0],];

	let mut t : Scalar = 0.0;
	//let mut frame_since_reset = 0;
	let mut frame = 1;
	let mut frame_since_report = 0;
	let mut time_prev : Scalar = -1.0;
	//let mut p1_min : Vec2d = [std::f64::MAX, std::f64::MAX];
	//let mut p1_max : Vec2d = [std::f64::MIN, std::f64::MIN];
	//let mut p2_min : Vec2d = [std::f64::MAX, std::f64::MAX];
	//let mut p2_max : Vec2d = [std::f64::MIN, std::f64::MIN];
	let mut t_sum : Scalar = 0.0;
	//let mut c1 : Vec2d = [0.0, 0.0];
	let size = 20; // mass.len();
	*/

	//let mut frame = 1;
	//let mut drag = false;

    // for e in window {
	// 	//e.input()
    //     e.draw_2d(|c, g| {
	// 		render(c,g,frame);
	// 	});
	//
	// 	if let Some(text) = e.text_args() {
	// 		println!("Text args: {}", text);
	// 	}
	// 	if let Some(button) = e.press_args() {
    //         if button == Button::Mouse(MouseButton::Left) {
    //             drag = true;
    //             //last_pos = e.mouse_cursor_args()
	// 			println!("Dragging");
    //         }
    //     };
    //     if let Some(button) = e.release_args() {
    //         if button == Button::Mouse(MouseButton::Left) {
    //             drag = false;
    //             //last_pos = None
	// 			println!("Done dragging");
    //         }
    //     };
	// 	if let Some(scroll) = e.mouse_scroll_args() {
	// 		println!("Mouse scroll {},{}", scroll[0], scroll[1]);
	// 	}
    //     if drag {
	// 		if let Some(pos) = e.mouse_relative_args() {
	// 			println!("Mouse drag {},{}", pos[0], pos[1]);
	//
	// 		}
			/*
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0] as f32, pos[1] as f32);

                if let Some(p) = last_pos {
                    let (last_x, last_y) = (p[0] as f32, p[1] as f32);
                    let distance = vec2_len(vec2_sub(p, pos)) as u32;

                    for i in 0..distance {
                        let diff_x = x - last_x;
                        let diff_y = y - last_y;
                        let delta = i as f32 / distance as f32;
                        let new_x = (last_x + (diff_x * delta)) as u32;
                        let new_y = (last_y + (diff_y * delta)) as u32;
                        if new_x < width && new_y < height {
                            canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
                            texture.update(&mut*e.factory.borrow_mut(), &canvas).unwrap();
                        };
                    };
                };

                last_pos = Some(pos)
            };
			*/
        // }


		/*
		//Perform timer and acceleration calculations after a small time-frame
		if time_prev >= 0.0 {
			let t_delta = 0.000_5; //precise_time_s() - time_prev;
			//t_delta *= 1.2;

			t += t_delta;
			t_sum += t_delta;
			frame += 1;
			frame_since_report += 1;
			//frame_since_reset += 1;

			//let c1 = [ 10.0 * (t * 1.2).sin(), 10.0 * (t * 1.2).cos() ];
			//let c2 = [ 450.0 * (t * -1.49).sin(), 450.0 * (t * -1.49).cos() ];
			if t_sum > 0.005 {

				for i in 0 .. size {

					// i=0 j=none, i=1, j=0, i=2, j=0,1, i=3, j=0,1,2, i=4, j=0,1,2,3,
					//let pos_slice = &position;
					//println!("Position slice: {:?} Length {}", pos_slice, pos_slice.len());
					for j in 0 .. size {
						if j < i {
							let x1 = position[j*2];
							let y1 = position[j*2+1];
							let x2 = position[i*2];
							let y2 = position[i*2+1];

							//get result of (x1,y1) - (x2,y2)
							let pos_diff = translate_vec([x1,y1], [x2 * -1.0, y2 * -1.0]);
							let x_diff = pos_diff[0] * 0.1;
							let y_diff = pos_diff[1] * 0.1;

							//Calculate hypotenuse of x_diff and y_diff R for formula
							let R = (x_diff.abs().powi(2) + y_diff.abs().powi(2)).sqrt();
							let x_diff_unit = x_diff / R;
							let y_diff_unit = y_diff / R;

							/ *
							if radius[j] + radius[i] >= R / 2.0 {
								if frame_since_report >= 100 {
									println!("Collision. R1 = {:?}, R2 = {:?}, R = {}", radius[j], radius[i], R);
									frame_since_report = 0;
								}
								let radius_portion1 = (R - radius[j]);
								let radius_portion2 = (R - radius[i]);
								velocity[j] = translate_vec(velocity[j], [x_diff_unit * radius_portion1, y_diff_unit * radius_portion1]);
								velocity[i] = translate_vec(velocity[i], [-x_diff_unit * radius_portion2, -y_diff_unit * radius_portion2]);
							}
							else * /
							{
								let M = mass[j];
								let m = mass[i];

								let F = funcF(G, M, m, R, t_sum);
								let A = F / M;
								let a = F / m;

								//apply relative acceleration to each pair
								velocity[j] = translate_vec(velocity[j], [t_sum * A * x_diff_unit, t_sum * A * y_diff_unit]);
								velocity[i] = translate_vec(velocity[i], [t_sum * a * -x_diff_unit, t_sum * a * -y_diff_unit]);
							}
						}
					}
				}

				//println!(" t_sum: {}", t_sum);
				t_sum = 0.0;
			}

			//Update positions from velocity vector
			//pos1 = velocity(pos1, c1, t_delta);
			for i in 0..size {
				let vel = apply_velocity([position[i*2], position[i*2+1]], velocity[i], t_delta);
				position[i*2] = vel[0];
				position[i*2+1] = vel[1];
			}


			{
				//frame_since_reset = 0;

				for i in 1..size {
					if mass[i] < 1.0 {
						if i%2 == 0 {
							let x_dist_sq = (position[i*2] - position[0]).abs().powi(2);
							let y_dist_sq = (position[i*2+1] - position[1]).abs().powi(2);

							if (2000.0_f64).powi(2) < x_dist_sq + y_dist_sq {

								//position at a random point on a circle around main mass
								let rad = between.ind_sample(&mut rng);
								position[i*2] = position[0] + 32.0 * rad.sin();
								position[i*2+1] = position[1] + 32.0 * rad.cos();
								//start with no velocity
								velocity[i][0] = 0.0;
								velocity[i][1] = 0.0;
							}

						}
						else {
							let x_dist_sq = (position[i*2] - position[5*2]).abs().powi(2);
							let y_dist_sq = (position[i*2+1] - position[5*2+1]).abs().powi(2);

							if (2000.0_f64).powi(2) < x_dist_sq + y_dist_sq {

								//position at a random point on a circle around main mass
								let rad = between.ind_sample(&mut rng);
								position[i*2] = position[5*2] + 32.0 * rad.sin();
								position[i*2+1] = position[5*2+1] + 32.0 * rad.cos();
								//start with no velocity
								velocity[i][0] = 0.0;
								velocity[i][1] = 0.0;
							}
						}
					}
				}
			}
		}
		//Save last timestamp
		//time_prev = precise_time_s();
		*/
    //}
}

// fn render(c : Context, g : &mut G2d, frame: &mut i32) {
//
// 	let vp_rect = c.viewport.unwrap().rect;
// 	let screen_rect = [vp_rect[0] as f64, vp_rect[1] as f64, vp_rect[2] as f64, vp_rect[3] as f64];
// 	let transform = c.transform.trans(100.0, 50.0).scale(1.2,1.2);
//
// 	if frame == 1 {
// 		rectangle( [0.0,1.0,0.0,1.0], screen_rect, c.transform, g);
// 	}
//
// 	//Decide if we should draw the background rectangle
// 	{/* //if frame % 11 == 0
// 		let unit_range = Range::<f32>::new(0.0, 1.0);
// 		let mut rng = rand::thread_rng();
//
// 		//let mut [x, y, width, height] = screen_rect;
// 		let width = unit_range.ind_sample(&mut rng) as f64 * screen_rect[2] as f64;
// 		let height = unit_range.ind_sample(&mut rng) as f64 * screen_rect[3] as f64;
// 		let x = screen_rect[0] as f64 + unit_range.ind_sample(&mut rng) as f64 * screen_rect[2] as f64 - width * 0.5;
// 		let y = screen_rect[1] + unit_range.ind_sample(&mut rng) as f64 * screen_rect[3] as f64 - height * 0.5;
// 		rectangle([unit_range.ind_sample(&mut rng), unit_range.ind_sample(&mut rng), unit_range.ind_sample(&mut rng), unit_range.ind_sample(&mut rng) * 0.03], //color
// 			[x, y, width, height], c.transform, g);
// 	*/}
//
//
//
// 	//Tell engine to renders its objects
// 	//engine.render(c, g);
// 	let center_offset = [0.0_f64, 0.0_f64]; //engine.calc_center_of_mass(); //[0.0_f64, 0.0_f64]; //
// 	if engine.frame % 100 == 0 {
// 		println!("Center of Mass {:?}", center_offset);
// 		println!("Object count {:?} Min/Max Star Distance ({},{})", engine.objects.len(), engine.min_max_star_distance.0, engine.min_max_star_distance.1);
// 	}
// 	let offset_x = center_offset[0] - 840.0;
// 	let offset_y = center_offset[1] - 525.0;
//
// 	rectangle( [0.0,0.0,0.0,0.9], screen_rect, c.transform, g);
//
// 	// if frame % 25 == 1
// 	// {
// 	// 	rectangle( [0.0,0.0,0.0,0.5], screen_rect, c.transform, g)
// 	// }
// 	// else
// 	// {
// 	// 	rectangle( [0.0,0.0,0.0,0.01], screen_rect, c.transform, g)
// 	// }
//
// 	let choice_range = Range::new(0.0, 1.0);
// 	let mut rng = rand::thread_rng();
//
// 	for i in 0..engine.objects.len() {
// 		let r = engine.objects[i].radius;
// 		let obj = &engine.objects[i];
// 		//println!("Render object {}, radius {}", i, r);
// 		ellipse(engine.objects[i].color, //color
// 			[engine.objects[i].position[0] - r - offset_x,
// 			engine.objects[i].position[1] - r - offset_y, r*2.0, r*2.0], //rectangle
// 			transform, //transform
// 			g);
//
// 		// let mut j = 0;
// 		//
// 		// for other_obj in engine.objects.iter() {
// 		// 	if j > i {
// 		// 		let choice = choice_range.ind_sample(&mut rng);
// 		// 		if choice > 0.9 {
// 		// 			line([0.5,0.5,0.5,0.25], 1.0,
// 		// 				[obj.position[0] - offset_x, obj.position[1] - offset_y, other_obj.position[0] - offset_x, other_obj.position[1] - offset_y,],
// 		// 				transform, g);
// 		// 		}
// 		// 	}
// 		// 	j += 1;
// 		// }
//
// 		if obj.radius > 9.0 {
// 			let color : [f32; 4] = [0.0,0.0,0.0,0.5];
// 			let r_x = r * 0.17;
// 			let r_y = r * 0.40;
// 			let pos_x_left = obj.position[0] - r * 0.3;
// 			let pos_y = obj.position[1] + r * 0.1;
// 			let pos_x_right = obj.position[0] + r * 0.3;
//
//
// 			let unit_range = Range::<f32>::new(0.0, 1.0);
// 			let mut rng = rand::thread_rng();
// 			if unit_range.ind_sample(&mut rng) < 0.05 {
// 				line(color, //color
// 				1.5, //radius
// 				[pos_x_left - r_x - offset_x,
// 				pos_y - offset_y,
// 				pos_x_left + r_x - offset_x,
// 				pos_y - offset_y,], //line
// 				transform, //transform
// 				g);
//
// 				line(color, //color
// 				1.5, //radius
// 				[pos_x_right - r_x - offset_x,
// 				pos_y - offset_y,
// 				pos_x_right + r_x - offset_x,
// 				pos_y - offset_y,], //line
// 				transform, //transform
// 				g);
// 			}
// 			else {
// 				ellipse(color, //color
// 				[pos_x_left - r_x - offset_x,
// 				pos_y - r_y - offset_y, r_x*2.0, r_y*2.0], //rectangle
// 				transform, //transform
// 				g);
//
// 				ellipse(color, //color
// 				[pos_x_right - r_x - offset_x,
// 				pos_y - r_y - offset_y, r_x*2.0, r_y*2.0], //rectangle
// 				transform, //transform
// 				g);
// 			}
// 		}
// 	}
//
// 	for x in (-1000..1000).filter(|a| a % 100 == 0) {
// 		for y in (-1000..1000).filter(|a| a % 100 == 0) {
// 			//println!("grid point at {},{}", x, y);
// 			line([0.3_f32,0.3_f32,0.3_f32,0.3_f32], 1.0, [(x as f64) + offset_x, (y as f64) + offset_y, (x as f64) + offset_x + 100.0, (y as f64) + offset_y], transform, g);
// 			line([0.3_f32,0.3_f32,0.3_f32,0.3_f32], 1.0, [(x as f64) + offset_x, (y as f64) + offset_y, (x as f64) + offset_x, (y as f64)  + offset_y + 100.0], transform, g);
// 		}
// 	}
//
// 	if frame % 1000 == 0 {
// 		engine.min_max_star_distance = (1_000_000.0, 0.0);
// 	}
//
// 	engine.advance();
//
// 	frame += 1;
// 	/*
// 	let mut x_total : f64 = 0.0;
// 	let mut y_total : f64 = 0.0;
// 	//let mut x_min_max : [f64; 2] = [MAX, MIN];
// 	//let mut y_min_max : [f64; 2] = [MAX, MIN];
// 	let mut mass_total = 0.0;
// 	for i in 0..size {
// 		x_total += mass[i] * position[i*2]; //mass[i] *
// 		y_total += mass[i] * position[i*2+1]; //mass[i] *
// 		mass_total += mass[i];
// 		/ *  if mass[i] > 1.0 {
// 			//mass_total += 1;
// 			if position[i*2] < x_min_max[0] { x_min_max[0] = position[i*2] }
// 			else if position[i*2] > x_min_max[1] { x_min_max[1] = position[i*2] }
// 			if position[i*2+1] < y_min_max[0] { y_min_max[0] = position[i*2+1] }
// 			else if position[i*2+1] > y_min_max[1] { y_min_max[1] = position[i*2+1] }
// 		} * /
// 	}
// 	/ *
// 	let mut x_span = x_min_max[1] - x_min_max[0];
// 	let mut y_span = y_min_max[1] - y_min_max[0];
// 	//get ratio to multiply radius and position by to fit within screen
// 	x_span = 0.5 * 1680.0 / x_span.max(1.0);
// 	y_span = 1.0 * 1050.0 / y_span.max(1.0);
// 	* /
// 	let biggest_span_ratio = 1.0; //x_span.min(y_span);
// 	//x_total = (x_min_max[1] - x_min_max[0]) / 2.0;
// 	//y_total = (y_min_max[1] - y_min_max[0]) / 2.0;  //y_total / mass_total as f64;// / mass_total as f64;
// 	//x_total *= biggest_span_ratio;
// 	//y_total *= biggest_span_ratio;
// 	x_total = x_total / mass_total;
// 	y_total = y_total / mass_total;
//
// 	for i in 0..size {
// 		let r = radius[i] * biggest_span_ratio;
// 		ellipse(color[i], //color
// 			[position[i*2] * biggest_span_ratio - r - x_total + 840.0,
// 			position[i*2+1] * biggest_span_ratio - r - y_total + 525.0, r*2.0, r*2.0], //rectangle
// 			c.transform, //transform
// 			g);
// 	}
//
// 	if frame % 100 == 0 {
// 		//println!("Draw State {:?}", c.draw_state);
// 	}*/
// }
