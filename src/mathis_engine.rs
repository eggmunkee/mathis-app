

//extern crate rand;
extern crate graphics;
extern crate gfx_graphics;
extern crate rand;

//External usages
//use std::collections::linked_list::LinkedList;
use graphics::math::*;
use graphics::types::Color;
use rand::distributions::Range;

//Internal usages
use mathis_object::MathisObject;

static PI : Scalar = 3.14159;

pub enum RemoveCheckFilter {
	None,
	CheckMassLessThan{ mass: Scalar },
	CheckMassGreaterThan{ mass: Scalar },
}

#[allow(non_snake_case)]
pub struct MathisEngine {
	pub objects: Vec<MathisObject<Vec2d>>,
	pub C : Scalar,
	//static MAX_DIST : Scalar = 100_000.0;
	pub G : Scalar,
	pub t_sum : Scalar,
	pub frame : u64,
	pub remove_check_filter : RemoveCheckFilter,
	pub distance_scale : Scalar,
	pub tick_rate : Scalar,

	pub min_max_star_distance : (Scalar, Scalar),
}

impl MathisEngine {
	pub fn new() -> MathisEngine {
		MathisEngine {
			objects: Vec::<MathisObject<Vec2d>>::new(),
			C: 8000.0, G: 500.0,
			t_sum: 0.0, frame: 1,
			remove_check_filter : RemoveCheckFilter::CheckMassLessThan { mass: 1.0 },
			distance_scale: 1.0, tick_rate: 0.01,
			min_max_star_distance: (1_000_000.0,0.0),
		}
	}

	pub fn new_with_g_c(gravity: Scalar, speedOfLight: Scalar, tickRate: Scalar) -> MathisEngine {
		MathisEngine {
			objects: Vec::<MathisObject<Vec2d>>::new(),
			C: speedOfLight, G: gravity,
			t_sum: 0.0, frame: 1,
			remove_check_filter : RemoveCheckFilter::CheckMassLessThan { mass: 1.0 },
			distance_scale: 1.0, tick_rate: tickRate,
			min_max_star_distance: (0.0,0.0),
		}
	}

	fn funcF(&self, M: Scalar, m: Scalar, R: Scalar, r_radius: Scalar, t_delta: Scalar) -> Scalar {
		(self.G * m * M / R.powi(2)) - (2.0 * self.G * m * M / ( r_radius * self.C * t_delta ))
	}

	pub fn addObject(&mut self, position: Vec2d, mass: Scalar, radius: Scalar, color: Color, velocity: Vec2d) {
		self.objects.push( MathisObject::new(position, mass, radius, color, velocity) );
	}

	pub fn calc_center_of_mass(&mut self) -> Vec2d {
		let mut x_total : f64 = 0.0;
		let mut y_total : f64 = 0.0;
		let mut mass_total : f64 = 0.0;

		if self.objects.len() == 0 { return [0.0,0.0]; }

		for i in 0..self.objects.len() {
			if self.objects[i].mass > 0.01 && self.objects[i].position[0].is_nan() == false && self.objects[i].position[1].is_nan() == false {
				x_total += self.objects[i].mass * self.objects[i].position[0]; //mass[i] *
				y_total += self.objects[i].mass * self.objects[i].position[1]; //mass[i] *
				mass_total += self.objects[i].mass;
			}
		}

		//Translate to x and y center of mass
		if mass_total == 0.0 { return [0.0,0.0]; }
		x_total = x_total / mass_total;
		y_total = y_total / mass_total;

		[x_total, y_total]
	}

	pub fn advance(&mut self) {

		// t - t delta, difference in time variable to advance over
		let t = self.tick_rate;
		{

			//t += t_delta;
			//self.t_sum += self.tick_rate;

			self.frame += 1;
			//frame_since_report += 1;
			//frame_since_reset += 1;

			let rng = rand::thread_rng();


			//let c1 = [ 10.0 * (t * 1.2).sin(), 10.0 * (t * 1.2).cos() ];
			//let c2 = [ 450.0 * (t * -1.49).sin(), 450.0 * (t * -1.49).cos() ];
			if t > 0.0001 {


				for i in 0 .. self.objects.len() {

					// i=0 j=none, i=1, j=0, i=2, j=0,1, i=3, j=0,1,2, i=4, j=0,1,2,3,
					//let pos_slice = &position;
					//println!("Position slice: {:?} Length {}", pos_slice, pos_slice.len());
					for j in 0 .. i {
						let x1 = self.objects[j].position[0];
						let y1 = self.objects[j].position[1];
						let x2 = self.objects[i].position[0];
						let y2 = self.objects[i].position[1];

						let M = self.objects[j].mass;
						let m = self.objects[i].mass;

						if x1.is_nan() || y1.is_nan() || x2.is_nan() || y2.is_nan() {
							panic!("Coordinate error: p1 ({},{}) p2 ({},{})", x1, y1, x2, y2);
						}

						//get result of (x1,y1) - (x2,y2)
						let pos_diff = MathisObject::<Vec2d>::translate_vec(&[x1,y1], &[x2 * -1.0, y2 * -1.0]);
						let mut x_diff = (pos_diff[0]) * self.distance_scale;
						let mut y_diff = (pos_diff[1]) * self.distance_scale;

						// Exclude minimal interactions between small particles at large distances for performance reasons
						if x_diff * x_diff + y_diff * y_diff > 2500.0 && M + m < 0.5 {
							continue;
						}

						let angle = pos_diff[1].atan2(pos_diff[0]);
						let R_sin = angle.sin();
						let R_cos = angle.cos();
						let mut x_diff2 = pos_diff[0].abs() * self.distance_scale;
						let mut y_diff2 = pos_diff[1].abs() * self.distance_scale;

						let radius_sum = self.objects[j].radius + self.objects[i].radius;
						let radius_sum_y = R_sin * radius_sum;
						let radius_sum_x = R_cos * radius_sum;
						if i == 0 && i == 11 && j == 0 {
							println!("Combined Radius - Radius {}, X component {}, Y component {}, x_diff {:?}, y_diff {:?}",
								radius_sum, radius_sum_x, radius_sum_y, x_diff, y_diff);

						}

						//Calculate hypotenuse of x_diff and y_diff R for formula
						let mut R = ((x_diff).abs().powi(2) + (y_diff).abs().powi(2)).sqrt().max(0.01);

						if R < self.objects[j].radius.max(self.objects[i].radius) {
						}

						// Calculate radius edge to radius edge length
						let mut r_radius = R - radius_sum; //((x_diff - radius_sum_x).abs().powi(2) + (y_diff - radius_sum_y).abs().powi(2)).sqrt();
						if r_radius < 2.5 {
							//let r_prime = r_radius - 0.51;
							//r_radius = 0.((-1.0 * 0) / r_prime).abs(); // 1.0 => 10, 0.5 => 2, 0 => 1, -1 => 0.5, -2 => 0.33
							// R = 0.6;
							// r_radius = 0.6;
							R = (radius_sum + 2.5 + R) / (radius_sum + 2.5);
							r_radius = (radius_sum + 2.5 + r_radius) / (radius_sum + 2.5);
						}

						if x_diff.is_nan() || y_diff.is_nan() {
							panic!("X/Y diff unit is Nan. x1,y1 = {:?},{:?} x2,y2 = {:?},{:?}", x1,y1,x2,y2);
							let vel_range = Range::new(-0.7, 0.7);
							x_diff = 0.7; //vel_range.ind_sample(&mut rng);
							y_diff = 0.7; //vel_range.ind_sample(&mut rng);
						}

						// if R.is_nan() == false {
						// 	if self.objects[j].mass > 10.0 && self.objects[i].mass > 10.0 { //between stars
						// 		if R < self.min_max_star_distance.0 { self.min_max_star_distance.0 = r_radius; }
						// 		else if R > self.min_max_star_distance.1 { self.min_max_star_distance.1 = r_radius; }
						// 	}
						// }
						// else {
						// 	panic!("R is Nan. x_diff = {:?}, y_diff = {:?}", x_diff, y_diff);
						// 	R = 0.1; //Min distance
						// }

						if r_radius.is_nan() {
							panic!("r_radius is Nan. x1,y1 = {:?},{:?} x2,y2 = {:?},{:?}", x1,y1,x2,y2);
							r_radius = 0.1;
						}

						let x_diff_unit = R_cos;
						let y_diff_unit = R_sin;

						if x_diff_unit.is_nan() || y_diff_unit.is_nan() {
							panic!("X/Y diff unit is Nan. r_radius = {:?}, x1,y1 = {:?},{:?} x2,y2 = {:?},{:?}", r_radius, x1,y1,x2,y2);

						}
						/*
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
						else */

						let mut F = self.funcF(M, m,R,r_radius, t);
						if F.is_nan() {
							panic!("Force is NaN: M {}, m {}, R {}, r_radius {}, t_delta {}", M, m, R, r_radius, t);
							F = 0.01;
						}
						let mut A = F / M;
						let mut a = F / m;
						if A.is_nan() {
							A = 0.01;
							panic!("A is NaN: F {}, M {}, m {}, R {}, r_radius {}, t_delta {}", F, M, m, R, r_radius, t);
						}
						if a.is_nan() {
							a = 0.01;
							panic!("a is NaN: F {}, M {}, m {}, R {}, r_radius {}, t_delta {}", F, M, m, R, r_radius, t);
						}

						//apply relative acceleration to each pair

						if self.objects[j].enable_accel {
							// if i == 0 && j == 0 {
							// 	println!("obj[0].vel {:?}, obj[0].x_acc {:?}, obj[0].y_acc {:?}. pos_diff {:?}, x_diff {:?}, y_diff {:?}",
							// 		self.objects[j].velocity, t * A * x_diff_unit, t * A * y_diff_unit, pos_diff, x_diff, y_diff);
							// }
							self.objects[j].velocity = MathisObject::<Vec2d>::translate_vec(&self.objects[j].velocity, &[t * A * x_diff_unit, t * A * y_diff_unit])
						}
						if self.objects[i].enable_accel {
							self.objects[i].velocity = MathisObject::<Vec2d>::translate_vec(&self.objects[i].velocity, &[t * a * -x_diff_unit, t * a * -y_diff_unit]);
						}
					}
				}

				//println!(" t_sum: {}", t_sum);
				//self.t_sum = 0.0;
			}
		}

		//Update positions from velocity vector
		//pos1 = velocity(pos1, c1, t_delta);
		for obj in self.objects.iter_mut() {
			//if obj.enable_accel == false { continue; }
			let new_position = MathisObject::<Vec2d>::apply_velocity(&obj.position, &obj.velocity, t);
			if new_position[0].is_nan() || new_position[1].is_nan() {
				println!("Debug object {:?}", obj);
				panic!("Apply Velocity error: t_delta {} v ({:?}) p ({:?}) p' ({:?})", t, obj.velocity, obj.position, new_position);
				// 	self.objects[i].velocity[0], self.objects[i].velocity[1],
				// 	self.objects[i].position[0], self.objects[i].position[1],
				// 	new_position[0], new_position[1]);
			}
			obj.position = new_position;
		}

		//Reset particle like objects
		if false {
			for i in 0..self.objects.len() {
				if self.objects[i].mass < 1.0 {
					if i % 2 == 0 {
						let x_dist_sq = (self.objects[i].position[0] - self.objects[0].position[0]).abs().powi(2);
						let y_dist_sq = (self.objects[i].position[1] - self.objects[0].position[1]).abs().powi(2);

						if (2000.0_f64).powi(2) < x_dist_sq + y_dist_sq {

							//position at a random point on a circle around main mass
							let rad : Scalar = 1.5; //self.between.ind_sample(&mut self.rng);
							self.objects[i].position[0] = self.objects[0].position[0] + self.objects[0].radius * 1.2 * rad.sin();
							self.objects[i].position[1] = self.objects[0].position[1] + self.objects[0].radius * 1.2 * rad.cos();
							//start with no velocity
							self.objects[i].velocity[0] = 0.0;
							self.objects[i].velocity[1] = 0.0;
						}

					}
					else {
						let x_dist_sq = (self.objects[i].position[0] - self.objects[5].position[0]).abs().powi(2);
						let y_dist_sq = (self.objects[i].position[1] - self.objects[5].position[1]).abs().powi(2);

						if (2000.0_f64).powi(2) < x_dist_sq + y_dist_sq {

							//position at a random point on a circle around main mass
							let rad : Scalar = 1.5; //self.between.ind_sample(&mut self.rng);
							self.objects[i].position[0] = self.objects[0].position[0] + self.objects[0].radius * 1.2 * rad.sin();
							self.objects[i].position[1] = self.objects[0].position[1] + self.objects[0].radius * 1.2 * rad.cos();
							//start with no velocity
							self.objects[i].velocity[0] = 0.0;
							self.objects[i].velocity[1] = 0.0;
						}
					}
				}
			}
		}

	}

	/*
	pub fn render(&self, c: graphics::context::Context, g: gfx_graphics::back_end::GfxGraphics<gfx::device::Resources,
			gfx::device::command::CommandBuffer<gfx::device::Resources>, gfx::render::target::Output>) {
		let center_offset = self.calc_center_of_mass();

		for i in 0..self.objects.len() {
			let r = self.objects[i].radius;
			println!("Render object {}, radius {}", i, r);
			ellipse(self.objects[i].color, //color
				[self.objects[i].position[0] - r - center_offset[0] + 840.0,
				self.objects[i].position[1] - r - center_offset[1] + 525.0, r*2.0, r*2.0], //rectangle
				c.transform, //transform
				g);
		}
	}*/
}
