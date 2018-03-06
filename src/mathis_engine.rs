
// External crates referenced
extern crate graphics;
extern crate rand;

//External usages (importing names)
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

	next_id: i32,
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
			next_id: 1,
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
			next_id: 1,
		}
	}

	pub fn set_g_c(&mut self, grav: Scalar, speed: Scalar) {
		self.G = grav;
		self.C = speed;
		println!("Set new G/C to: ({}, {})", self.G, self.C);
	}

	fn funcF_orig(&self, mut M: Scalar, mut m: Scalar, mut r: Scalar, mut r_radius: Scalar, mut t_delta: Scalar) -> Scalar {
		if r <= 0.0 {
			return 0.0;
		}

		M *= 1.0;
		m *= 1.0;
		r *= 10.0;
		t_delta *= 1.0;
		let mut eff_g = self.G; // * 0.001;
		let mut eff_c = self.C;
		let mut r_squared = r.powi(2);

		return 0.0;
	}

	fn funcF(&self, mut M: Scalar, mut m: Scalar, mut r: Scalar, mut r_radius: Scalar, mut t_delta: Scalar) -> Scalar {
		// Can't have a zero radius interaction, implies divide by zero
		if r <= 0.0 {
			return 0.0;
		}

		// Perform any multipliers
		M *= 1.0;
		m *= 1.0;
		r *= 10.0;
		t_delta *= 1.0;

		// Set effective variables for equation
		let eff_g = self.G; // * 0.001;
		let eff_c = self.C;
		let r_squared = r.powi(2);

		// Get the radius squared for use below
		let r_squared2 = r_squared * r_squared;

		// Calculate two halves of equation
		let expansion = -(eff_g * m * M  / (r_squared));
		let bombardment = (2.0 * eff_g * m * M / ( r_squared2 * eff_c ));

		// Handle unexpected output cases
		if (expansion > 0.0) { panic!("Expansion isn't negative! H = {}", expansion); }
		if (bombardment < 0.0) { panic!("Bombardment isn't negative! E = {}", bombardment); }

		// Get sum force
		let force = expansion + bombardment;

		return force;
	}

	pub fn addObject(&mut self, position: Vec2d, mass: Scalar, radius: Scalar, color: Color, velocity: Vec2d) {
		self.next_id += 1;
		self.objects.push( MathisObject::new(self.next_id, position, mass, radius, color, velocity) );
	}

	fn check_accel(&mut self, obj_idx: usize, obj_accel: f64, obj_2_idx: usize)  {
		let obj_accel_chk = obj_accel.abs();

		if obj_accel_chk > self.objects[obj_idx].max_accel.abs() {
			self.objects[obj_idx].max_accel = obj_accel;
			self.objects[obj_idx].max_accel_id = self.objects[obj_2_idx].obj_id;
		}
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

		let t = self.tick_rate;
		{

			self.frame += 1;

			let rng = rand::thread_rng();

			for i in 0 .. self.objects.len() {
				self.objects[i].max_accel = 0.0;
				self.objects[i].max_accel_id = -1;
			}

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

					// Get angle between points, then the x and y components of that angle
					let angle = pos_diff[1].atan2(pos_diff[0]);
					let R_sin = angle.sin();
					let R_cos = angle.cos();

					if x_diff.is_nan() || y_diff.is_nan() {
						panic!("X/Y diff unit is Nan. x1,y1 = {:?},{:?} x2,y2 = {:?},{:?}", x1,y1,x2,y2);
					}

					//Calculate hypotenuse of x_diff and y_diff R for formula
					let mut R = ((x_diff).abs().powi(2) + (y_diff).abs().powi(2)).sqrt();

					let x_diff_unit = R_cos;
					let y_diff_unit = R_sin;

					let mut F = self.funcF(M, m,R,0.0, t);
					if !F.is_nan() && F != 0.0 && F < 1000.0 {

						let mut A = F / M;
						let mut a = F / m;
						if A.is_nan() {
							A = 0.01;
							panic!("A is NaN: F {}, M {}, m {}, R {}, r_radius {}, t_delta {}", F, M, m, R, 0.0, t);
						}
						if a.is_nan() {
							a = 0.01;
							panic!("a is NaN: F {}, M {}, m {}, R {}, r_radius {}, t_delta {}", F, M, m, R, 0.0, t);
						}

						//apply relative acceleration to each object of the pair
						if self.objects[j].enable_accel {
							{
								self.check_accel(j, A, i);
							}
							// if i == 0 && j == 0 {
							// 	println!("obj[0].vel {:?}, obj[0].x_acc {:?}, obj[0].y_acc {:?}. pos_diff {:?}, x_diff {:?}, y_diff {:?}",
							// 		self.objects[j].velocity, t * A * x_diff_unit, t * A * y_diff_unit, pos_diff, x_diff, y_diff);
							// }
							self.objects[j].velocity = MathisObject::<Vec2d>::translate_vec(&self.objects[j].velocity, &[t * A * x_diff_unit, t * A * y_diff_unit])
						}
						if self.objects[i].enable_accel {
							{
								self.check_accel(i, a, j);
							}
							self.objects[i].velocity = MathisObject::<Vec2d>::translate_vec(&self.objects[i].velocity, &[t * a * -x_diff_unit, t * a * -y_diff_unit]);
						}
					}
				}

			}
		}

		//Update positions from velocity vector
		for obj in self.objects.iter_mut() {
			//if obj.enable_accel == false { continue; }
			let new_position = MathisObject::<Vec2d>::apply_velocity(&obj.position, &obj.velocity, t);
			if new_position[0].is_nan() || new_position[1].is_nan() {
				println!("Debug object {:?}", obj);
				panic!("Apply Velocity error: t_delta {} v ({:?}) p ({:?}) p' ({:?})", t, obj.velocity, obj.position, new_position);
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

}
