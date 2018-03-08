
use mathis_engine::*;

use rand::{thread_rng};
use rand::distributions::{IndependentSample, Range};
use graphics::math::Scalar;

#[allow(dead_code)]
pub fn single_star(eng: &mut MathisEngine) {
	//eng.addObject([-11739.0, -882.0], 10000.0, 1000.0, [0.3, 1.0, 0.7, 0.2], [0.0, 0.0]);

	eng.addObject([0.0, 0.0], 10000.0, 1000.0, [0.3, 1.0, 0.7, 0.2], [0.0, 0.0]);


	// uncomment to pin this star
    // let last_index = eng.objects.len()-1;
    // eng.objects[last_index].enable_accel = false;
}

#[allow(dead_code)]
pub fn build_prot_elec(eng: &mut MathisEngine) {
	eng.addObject([0.0, 0.0], 1987.0, 1987.0, [0.7, 1.0, 0.2, 0.25], [0.0, 0.0]);
	eng.addObject([1000.0, 0.0], 1.0, 1.0, [1.0, 0.1, 0.2, 0.5], [0.0, -310.0]);
}

pub fn build_twin_stars_default(eng: &mut MathisEngine) {
	build_twin_stars(eng, &[0.0, 0.0], &[0.0, 0.0], &1.0, &1.0, &1.0);
}

#[allow(dead_code)]
pub fn build_twin_stars(eng: &mut MathisEngine, pos_offset: &[f64; 2], vel_offset: &[f64; 2],
	radius_multiplier: &f64, mass_multiplier: &f64, vel_multiplier: &f64) {

	let vert_speed = 17.7 * vel_multiplier;
	let radius_from_center = 4000.0 * radius_multiplier;
	let star_mass = 100000.0 * mass_multiplier;
	let star_radius = 300.0 * mass_multiplier;
	eng.addObject([-radius_from_center + pos_offset[0], 0.0 + pos_offset[1]], // position
		star_mass, star_radius, // mass and radius
		[1.0, 0.1, 0.1, 0.5], // color
		[0.0 + vel_offset[0], -vert_speed + vel_offset[1]]); // velocity
	eng.addObject([radius_from_center + pos_offset[0], 0.0 + pos_offset[1]],  // position
		star_mass, star_radius, // mass and radius
		[0.1, 1.0, 0.1, 0.5], // color
		[0.0 + vel_offset[0], vert_speed + vel_offset[1]]); // velocity
}

pub fn build_4_body_system(eng: &mut MathisEngine) {
	eng.addObject([-4000.0, 0.0], 100000.0, 300.0, [1.0, 1.0, 1.0, 0.3], [0.0, 0.0]);
	eng.addObject([400.0, -600.0], 1000.0, 150.0, [0.3, 1.0, 0.7, 0.2], [-0.15, 1.7]);
	eng.addObject([-5500.0, 700.0], 200.05, 30.0, [0.2, 0.5, 1.0, 0.2], [0.0, -2.3]);
	eng.addObject([700.0, -600.0], 10.0, 15.0, [1.0, 1.0, 0.3, 0.3], [-0.15, 2.2]);
}


#[allow(dead_code)]
pub fn build_sun_scene(eng: &mut MathisEngine) {
	eng.addObject([0.0, 0.0], 200.0, 50.0, [1.0, 0.5, 0.5, 0.3], [0.0, 0.0]);
	eng.addObject([300.0, 0.0], 31.5, 18.0, [0.0, 0.5, 1.0, 0.8], [-5.0, 550.0]);
	eng.addObject([-300.0, 0.0], 50.0, 38.0, [0.0, 0.5, 1.0, 0.8], [0.0, -550.0]);
}

#[allow(dead_code)]
pub fn build_planets_scene(eng: &mut MathisEngine) {
	//eng.addObject([300.0, 300.0], 100.0, 50.0, [0.25, 0.5, 0.8, 0.5], [0.0, 10.0]);
	//eng.addObject([100.0, 300.0], 25.0, 25.0, [0.1, 0.4, 0.7, 0.5], [25.0, -10.0]);
	//eng.addObject([200.0, 400.0], 5.0, 5.0, [0.6, 0.5, 0.2, 0.5], [-10.0, 5.0]);

	//eng.addObject([300.0, -300.0], 50.0, 25.0, [0.25, 0.5, 0.8, 0.5], [0.0, 10.0]);
	//eng.addObject([100.0, -300.0], 12.5, 13.0, [0.1, 0.4, 0.7, 0.5], [25.0, -10.0]);
	//eng.addObject([200.0, -400.0], 6.25, 6.5, [0.6, 0.5, 0.2, 0.5], [-10.0, 5.0]);

	eng.addObject([-300.0, 0.0], 25.0, 12.5, [0.7, 0.5, 0.5, 0.5], [0.0, 150.0]);
	eng.addObject([300.0, 0.0], 25.0, 12.5, [0.8, 0.5, 0.3, 0.5], [0.0, -150.0]);

	eng.addObject([0.0, -300.0], 25.0, 12.5, [0.7, 0.5, 0.5, 0.5], [-150.0, 0.0]);
	eng.addObject([0.0, 300.0], 25.0, 12.5, [0.8, 0.5, 0.3, 0.5], [150.0, 0.0]);

	//eng.addObject([0.0, 0.0], 50.0, 30.0, [0.8, 0.9, 0.1, 0.5], [0.0, 0.0]);

	// eng.addObject([-100.0, 0.0], 25.0, 12.5, [0.8, 0.5, 0.3, 0.5], [0.0, 50.0]);
	// eng.addObject([100.0, 0.0], 25.0, 12.5, [0.7, 0.5, 0.5, 0.5], [0.0, -50.0]);
	// eng.addObject([0.0, -100.0], 25.0, 12.5, [0.7, 0.5, 0.5, 0.5], [-50.0, 0.0]);
	// eng.addObject([0.0, 100.0], 25.0, 12.5, [0.8, 0.5, 0.3, 0.5], [50.0, 0.0]);

	//eng.addObject([-100.0, 0.0], 6.25, 5.0, [0.1, 0.4, 0.7, 0.5], [25.0, -10.0]);
	//eng.addObject([-200.0, 0.0], 3.125, 2.0, [0.6, 0.5, 0.2, 0.5], [-10.0, 5.0]);

}


#[allow(dead_code)]
pub fn some_particles(eng: &mut MathisEngine, num_particles: &i32, avg_mass: &f64, avg_vel: &f64, area_size: &[f64; 2], pos_offset: &[f64; 2]) {
	let half_width = area_size[0] * 0.5;
	let half_height = area_size[1] * 0.5;
	let x_range = Range::new(-half_width, half_width);
	let y_range = Range::new(-half_height, half_height);
	let mass_range = Range::<f64>::new(0.08, 1.0);
	let color_range = Range::new(0.1_f32, 1.0_f32);
	let vel_range = Range::new(-2.5, 2.5);
	let mut rng = thread_rng();

	for _ in 0..*num_particles {
		let mut m : Scalar = mass_range.ind_sample(&mut rng) * avg_mass;
		let vel_x = vel_range.ind_sample(&mut rng) * avg_vel;
		let vel_y = vel_range.ind_sample(&mut rng) * avg_vel;
		let rad : Scalar = m; //.sqrt();
		eng.addObject([x_range.ind_sample(&mut rng) + pos_offset[0], y_range.ind_sample(&mut rng) + pos_offset[1]],
			m, rad * 2.0,
			[color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng), (color_range.ind_sample(&mut rng) - 0.5_f32).abs() + 0.5_f32],
			[vel_x, vel_y]);
	}
}

#[allow(dead_code)]
pub fn lots_of_particles(eng: &mut MathisEngine) {
	let x_range = Range::new(-840.0, 840.0);
	let y_range = Range::new(-525.0, 525.0);
	let mass_range_sqrt = Range::<f64>::new(0.01, 1.2);
	let color_range = Range::new(0.1_f32, 1.0_f32);
	let vel_range = Range::new(-250.0, 250.0);
	let mut rng = thread_rng();

	for _ in 0..200 {
		let m : Scalar = mass_range_sqrt.ind_sample(&mut rng).powi(2);
		eng.addObject([x_range.ind_sample(&mut rng), y_range.ind_sample(&mut rng)],
			m, (m * 2.0).max(0.5),
			[color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng), (color_range.ind_sample(&mut rng) - 0.5_f32).abs() + 0.5_f32],
			[vel_range.ind_sample(&mut rng), vel_range.ind_sample(&mut rng)]);
	}
}

#[allow(dead_code)]
pub fn lots_of_particles_close(eng: &mut MathisEngine) {
	let x_range = Range::new(-400.0, 400.0);
	let y_range = Range::new(-250.0, 250.0);
	let mass_range_sqrt = Range::<f64>::new(0.1, 1.0);
	let color_range = Range::new(0.1_f32, 1.0_f32);
	let vel_range = Range::new(-80.0, 80.0);
	let mut rng = thread_rng();

	for _ in 0..200 {
		let m : Scalar = mass_range_sqrt.ind_sample(&mut rng).powi(2);
		eng.addObject([x_range.ind_sample(&mut rng), y_range.ind_sample(&mut rng)],
			m, (m * 2.0).max(1.0),
			[color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng), (color_range.ind_sample(&mut rng) - 0.5_f32).abs() + 0.5_f32],
			[vel_range.ind_sample(&mut rng), vel_range.ind_sample(&mut rng)]);
	}
}

#[allow(dead_code)]
pub fn generate_grid(eng: &mut MathisEngine, xy_xy: &[i32;4], div_xy: &[i32;2], mass: &f64, base_velocity: &f64) {
	let m : Scalar = *mass;
	let color_range = Range::new(0.1_f32, 1.0_f32);
	let mass_var_range = Range::new(0.005_f64, 1.75_f64); //Range::new(0.85_f64, 1.25_f64);
	let vel_range = Range::new(-1.0, 1.0);
	let mut rng = thread_rng();

	let x1 = xy_xy[0];
	let y1 = xy_xy[1];
	let x2 = xy_xy[2];
	let y2 = xy_xy[3];
	let x_div = div_xy[0];
	let y_div = div_xy[1];

	for x in (x1..x2).filter(|a| (a+x1) % x_div == 0) {
		for y in (y1..y2).filter(|a| (a+y1) % y_div == 0) {
			println!("grid point at {},{}", x, y);
			let mx = x;
			let my = y;
			let o_mass = mass_var_range.ind_sample(&mut rng) * m;
			let o_vel_x = vel_range.ind_sample(&mut rng) * base_velocity;
			let o_vel_y = vel_range.ind_sample(&mut rng) * base_velocity;
			eng.addObject([mx as f64, my as f64],
				o_mass, o_mass * 1.5,
				[color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng), color_range.ind_sample(&mut rng),
					(color_range.ind_sample(&mut rng) - 0.5_f32).abs() + 0.5_f32],
				[o_vel_x, o_vel_y]);
		}
	}
}
