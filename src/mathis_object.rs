
//External usages
use std;
use graphics::math::*;
use graphics::types::Color;

//Internal usages

#[derive(Debug)]
pub enum RadiusAction {
	None,
	RemoveAtRadius(Scalar),
}

#[derive(Debug)]
pub enum RadiusOrigin {
	CenterOfGravity,
}

#[derive(Debug)]
pub struct MathisObject<T> {
	pub obj_id : i32,
	pub position : T,
	pub mass : Scalar,
	pub color : Color,
	pub radius : Scalar,
	pub velocity : T,
	pub radius_action : RadiusAction,
	pub radius_origin : RadiusOrigin,
	pub max_accel : f64,
	pub max_accel_id: i32,
	pub enable_accel : bool,
}

//Generic methods
impl<T> MathisObject<T> {
	pub fn new(id: i32, position: T, mass: Scalar, radius: Scalar, color: Color, velocity: T) -> MathisObject<T> {
		MathisObject::<T> { obj_id: id, position: position, mass: mass, radius: radius, color: color, velocity: velocity,
			radius_action: RadiusAction::RemoveAtRadius(500.0), radius_origin: RadiusOrigin::CenterOfGravity, enable_accel: true,
		 	max_accel : 0.0, max_accel_id : -1}
	}
}

//Implement a few 2d operations
impl<Vec2d> MathisObject<Vec2d> {
	pub fn translate_vec(vec: &[Scalar;2], vec2: &[Scalar;2]) -> [Scalar;2] {
		//Return updated position
		[vec[0] + vec2[0], vec[1] + vec2[1]]
	}

	pub fn apply_velocity(vel: &[Scalar;2], acc: &[Scalar;2], t_delta: Scalar) -> [Scalar;2] {
		//Return updated position
		[vel[0] + acc[0] * t_delta, vel[1] + acc[1] * t_delta]
	}
	/*
	fn particle_at_tangent(&self, speed: Scalar) -> MathisObject<Vec2d> {

		let r = self.radius;
		let tangent_position = MathisObject::<Vec2d>::translate_vec(&self.position, &[r, 0.0]);

		MathisObject::<Vec2d>::new(tangent_position, 0.0001, 0.7, [0.5, 0.5, 1.0, 0.5], [speed, 0.0])
	}*/
}


//==== Ordering implementations

//Implement the Ord trait so it can be automatically sorted
impl<T> std::cmp::Ord for MathisObject<T> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.mass > other.mass {
			std::cmp::Ordering::Greater
		} else if self.mass < other.mass {
			std::cmp::Ordering::Less
		} else {
			std::cmp::Ordering::Equal
		}
	}
}

impl<T> std::cmp::Eq for MathisObject<T> {}

impl<T> std::cmp::PartialEq for MathisObject<T> {
	fn eq(&self, other: &Self) -> bool {
		self.mass == other.mass
	}
}

impl<T> std::cmp::PartialOrd for MathisObject<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.mass > other.mass {
			Some(std::cmp::Ordering::Greater)
		} else if self.mass < other.mass {
			Some(std::cmp::Ordering::Less)
		} else {
			Some(std::cmp::Ordering::Equal)
		}
	}
}
