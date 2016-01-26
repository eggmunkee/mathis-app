extern crate piston_window;
extern crate vecmath;
extern crate graphics;
extern crate clock_ticks;

use piston_window::*;
use clock_ticks::*;
use graphics::math::*;
use std::thread;
use std::f64::*;

mod mathis_engine;
mod mathis_object;

use mathis_engine::*;
use mathis_object::*;


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
static C : Scalar = 1000.0;
static MAX_DIST : Scalar = 100_000.0;

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
fn funcF(G: Scalar, M: Scalar, m: Scalar, R: Scalar, t: Scalar) -> Scalar {
	(G * m * M / R.powi(2)) - (2.0 * G * m * M / ( R * C * t ))
}

fn translate_vec(vec: Vec2d, vec2: Vec2d) -> Vec2d {
	//Return updated position
	[vec[0] + vec2[0], vec[1] + vec2[1]]
}

fn apply_velocity(pos: Vec2d, vel: Vec2d, t_delta: Scalar) -> Vec2d {
	//Return updated position
	[pos[0] + vel[0] * t_delta, pos[1] + vel[1] * t_delta]
}

fn main() {
	let engine : MathisEngine = MathisEngine::new();

    let window: PistonWindow =
        WindowSettings::new(format!("Orbiter v{}", VERSION), [1024, 768])
        .exit_on_esc(true).build().unwrap();
	//let mut pos1 : Vec2d = [400.0, 350.0];
	let mut position : [Scalar; 10] = [200.0, 250.0,
										460.0, 390.0,
										540.0, 390.0,
										700.0, 390.0,
										1020.0, 590.0];
	let mass : [Scalar; 5] = [100.0, 1.0, 2.0, 4.0, 8.0];
	let radius : [Scalar; 5] = [120.0, 1.2, 2.4, 4.8, 9.6];
	let mut t : Scalar = 0.0;
	let mut frame = 1;
	let mut time_prev : Scalar = -1.0;
	//let mut p1_min : Vec2d = [std::f64::MAX, std::f64::MAX];
	//let mut p1_max : Vec2d = [std::f64::MIN, std::f64::MIN];
	//let mut p2_min : Vec2d = [std::f64::MAX, std::f64::MAX];
	//let mut p2_max : Vec2d = [std::f64::MIN, std::f64::MIN];
	let mut t_sum : Scalar = 0.0;
	let mut c1 : Vec2d = [0.0, 0.0];
	let mut velocity : [Vec2d; 5] = [[0.0, 0.0],[0.0, -1.38],[0.0, -0.775],[0.0, -0.485],[0.0, -0.335],];
	let mut G = 40.0;

    for e in window {
        e.draw_2d(|c, g| {

			//c.scale(0.25, 0.25);
			//c.store_view();

			let vp_rect = c.viewport.unwrap().rect;
			let screen_rect = [vp_rect[0] as f64, vp_rect[1] as f64, vp_rect[2] as f64, vp_rect[3] as f64];

			rectangle([0.5, 0.5, 0.5, 0.02], //color
				screen_rect, c.transform, g);

			/*
            rectangle([0.5 + 0.5 * (t as f32 * 0.3).sin(), 0.5 + 0.5 * (t as f32 * 0.8).sin(), //color
				0.5 + 0.5 * (t as f32 * 1.1).sin(), 0.03 + 0.03 * (t as f32 * 0.1).sin()], //color
				screen_rect, c.transform, g);

			for i in 0..(vp_rect[2]/50)+1 {
				for j in 0..(vp_rect[3]/50)+1 {
					rectangle([0.5 + 0.5 * (t as f32 * 0.5).sin(), 0.5 + 0.5 * (t as f32 * 0.5).sin(), //color
						0.5 + 0.5 * (t as f32 * 0.5).sin(), 0.04 + 0.04 * (t as f32 * 0.15).sin()], //color
						[i as f64 * 50.0, j as f64 * 50.0, 48.0, 48.0], c.transform, g);
				}
			}

			//for _ in 0..1000 {
				for e in 0..20 {
					let e = (e as f32 * 0.5);
				    ellipse([0.5 + 0.5 * (t as f32 * 0.46 + e).sin(), 0.5 + 0.5 * (t as f32 * 0.56 + e).sin(), //color
							0.5 + 0.5 * (t as f32 * 0.9 + e).sin(), 0.5 + 0.5 * (t as f32 * 1.42 + e).sin()], //color
						[pos1[0] + (t * 0.9 + e as f64).cos() * 300.0, pos1[1] + (t * 0.9 + e as f64).sin() * 170.0, 200.0 + 100.0 * t.cos(), 200.0 + 100.0 * t.sin()], //rectangle
						c.transform, //transform
						g);
				}

				for e in 0..20 {
					let e = (e as f32 * 0.5);
					rectangle([0.5 + 0.5 * (t as f32 * 0.1 + e).sin(), 0.5 + 0.5 * (t as f32 * 0.9 + e).sin(), //color
							0.5 + 0.5 * (t as f32 * 1.7 + e).sin(), 0.5 + 0.5 * (t as f32 * 0.5 + e).sin()], //color
						[pos2[0] + (t * -1.76 + e as f64).cos() * 250.0, pos2[1] + (t * -1.76 + e as f64).sin() * 200.0, 200.0 + 100.0 * t.sin(), 200.0 + 100.0 * t.cos()], //rectangle
						c.transform, //transform
						g);
				}
			//}
			*/

			let r = 120.0;

			/*
		    ellipse([0.7, 0.8, 0.3, 0.8], //color
				[pos1[0] - (r * 0.5), pos1[1] - (r * 0.5), r, r], //rectangle
				c.transform, //transform
				g);
			*/
			
			for i in 0..5 {
				let r = 5.0 * (2.0_f64).powi((i) as i32);
				ellipse([0.6 + 0.4 * (-0.5 + (i as f32) / 5.0), 0.8 - 0.4 * (-0.5 + (i as f32) / 5.0), 0.8 - 0.6 * (-0.5 + (i as f32) / 5.0), 1.0], //color
					[position[i*2] - (r * 0.5), position[i*2+1] - (r * 0.5), r, r], //rectangle
					c.transform, //transform
					g);


			}

			if frame % 100 == 0 { 
				//println!("Draw State {:?}", c.draw_state);
			}
        });


		//Perform 
		if time_prev >= 0.0 {
			let mut t_delta = precise_time_s() - time_prev;
			t_delta *= 51.0;

			t += t_delta;
			t_sum += t_delta;
			frame += 1;

			//let c1 = [ 10.0 * (t * 1.2).sin(), 10.0 * (t * 1.2).cos() ];
			//let c2 = [ 450.0 * (t * -1.49).sin(), 450.0 * (t * -1.49).cos() ];
			if frame % 100 == 0 && t_sum > 0.0 {

				for i in 0..5 {

					/*
					let pos_diff = trans(pos1, [pos2[i*2] * -1.0, pos2[i*2+1] * -1.0]);
					let R = (pos_diff[0].abs().powi(2) + pos_diff[1].abs().powi(2)).sqrt();
					let M = 100.0;
					let m  = 0.01 * (2.0_f64).powi((i) as i32);
					let F = funcF(G, M, m, R, t_sum);
					let A = F / M;
					let a = F / m;

					//apply relative acceleration to each pair
					c1 = trans(c1, [A * (pos2[i*2] - pos1[0]) / R / t_sum, A * (pos2[i*2+1] - pos1[1]) / R / t_sum ]);
					c2[i] = trans(c2[i], [a * (pos1[0] - pos2[i*2]) / R / t_sum, a * (pos1[1] - pos2[i*2+1]) / R / t_sum ]);
					*/

					// i=0 j=none, i=1, j=0, i=2, j=0,1, i=3, j=0,1,2, i=4, j=0,1,2,3,
					let pos_slice = &position;
					//println!("Position slice: {:?} Length {}", pos_slice, pos_slice.len());
					for j in 0..pos_slice.len()/2 {
						if j != i {
							let pos_diff = translate_vec([position[j*2],position[j*2+1]], [position[i*2] * -1.0, position[i*2+1] * -1.0]);
							let R = (pos_diff[0].abs().powi(2) + pos_diff[1].abs().powi(2)).sqrt();
							let M = mass[i];
							let m = mass[j];

							let F = funcF(G, M, m, R, t_sum);
							let A = F / M;
							let a = F / m;

							//apply relative acceleration to each pair
							velocity[j] = translate_vec(velocity[j], [A * (position[j*2] - position[i*2]) / R / t_sum, A * (position[j*2+1] - position[i*2+1]) / R / t_sum ]);
							velocity[i] = translate_vec(velocity[i], [a * (position[i*2] - position[j*2]) / R / t_sum, a * (position[i*2+1] - position[j*2+1]) / R / t_sum ]);
						}
					}
				}

				//println!(" t_sum: {}, pos1: {:?}, pos2: {:?}, Diff: {:?}, R: {}, F: {}, c1: {:?}, c2: {:?}", t_sum, pos1, pos2, pos_diff, R, F, c1, c2);
			}

			//Update positions from velocity vector
			//pos1 = velocity(pos1, c1, t_delta);

			for i in 0..5 {
				let vel = apply_velocity([position[i*2], position[i*2+1]], velocity[i], t_delta);
				position[i*2] = vel[0];
				position[i*2+1] = vel[1];
			}

			//Get x mins and maxes
			//p1_min[0] = pos1[0].min(p1_min[0]);
			//p1_max[0] = pos1[0].max(p1_max[0]);
			//p2_min[0] = pos2[0].min(p2_min[0]);
			//p2_max[0] = pos2[0].max(p2_max[0]);
			//Get y mins and maxes
			//p1_min[1] = pos1[1].min(p1_min[1]);
			//p1_max[1] = pos1[1].max(p1_max[1]);
			//p2_min[1] = pos2[1].min(p2_min[1]);
			//p2_max[1] = pos2[1].max(p2_max[1]);

			if frame % 100 == 0 {
				let fps : f64 = 100.0 / t_sum;
				//println!("t: {} t_delta: {} c: {:?} pos1: {:?}, pos2: {:?}", t, t_delta, c1, pos1, pos2);
				//println!(" :: p1 min {:?}, p1 max {:?}, p2 min {:?}, max {:?}", p1_min, p1_max, p2_min, p2_max);
				println!(" FPS: {}, frames: {}, t_sum {}", fps, 100, t_sum);

				t_sum = 0.0;
				if frame == 1000 { frame = 0; }
			}
			/*
			let min_time : f64 = 1.0 / 60.0;
			if t_delta < min_time {
				let t_remainder = (min_time - t_delta) * 1000.0;
				if t_remainder >= 10.0 {
					//println!(" t_remainder: {}", t_remainder);
					//thread::sleep_ms(10);
				}
			}
			*/

		}
		//Save last timestamp
		time_prev = precise_time_s();
    }
}
