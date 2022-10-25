use std::ops::{Add, Mul};

pub fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
	(1.0 - amount) * start + amount * stop
}

pub fn sanitize_degrees_int(degrees: i32) -> u32 {
	sanitize_degrees_double(degrees as f64) as u32
}

pub fn sanitize_degrees_double(degrees: f64) -> f64 {
	let mut degrees = degrees % 360.0;
	if degrees < 0.0 {
		degrees += 360.0;
	}
	degrees
}

pub fn rotation_direction(from: f64, to: f64) -> f64 {
	let increasing_difference = sanitize_degrees_double(to - from);
	if increasing_difference <= 180.0 {
		1.0
	} else {
		-1.0
	}
}

pub fn difference_degrees(a: f64, b: f64) -> f64 {
	180.0 - ((a - b).abs() - 180.0).abs()
}

pub fn matrix_multiply<T>(row: [T; 3], matrix: [[T; 3]; 3]) -> [T; 3]
where
	T: Mul<Output = T> + Add<Output = T> + Copy,
{
	let a = row[0] * matrix[0][0] + row[1] * matrix[0][1] + row[2] * matrix[0][2];
	let b = row[0] * matrix[1][0] + row[1] * matrix[1][1] + row[2] * matrix[1][2];
	let c = row[0] * matrix[2][0] + row[1] * matrix[2][1] + row[2] * matrix[2][2];
	[a, b, c]
}
