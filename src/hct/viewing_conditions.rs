use super::XYZ_TO_CAM16RGB;
use crate::utils::{
	color::{white_point_d65, y_from_lstar},
	math::{lerp, matrix_multiply},
};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct ViewingConditions {
	aw: f64,
	nbb: f64,
	ncb: f64,
	c: f64,
	nc: f64,
	n: f64,
	rgb_d: [f64; 3],
	fl: f64,
	fl_root: f64,
	z: f64,
}

impl ViewingConditions {
	fn new(
		white_point: [f64; 3],
		adapting_luminance: f64,
		background_lstar: f64,
		surround: f64,
		discounting_illuminant: bool,
	) -> Self {
		let [r_w, g_w, b_w] = matrix_multiply(white_point, XYZ_TO_CAM16RGB);

		let f = 0.8 + surround / 10.0;

		let c = if f >= 0.9 {
			lerp(0.59, 0.69, (f - 0.9) * 10.0)
		} else {
			lerp(0.525, 0.59, (f - 0.8) * 10.0)
		};

		let d = if discounting_illuminant {
			1.0
		} else {
			f * (1.0 - ((1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp())).clamp(0.0, 1.0)
		};

		let nc = f;

		let rgb_d = [
			d * (100.0 / r_w) + 1.0 - d,
			d * (100.0 / g_w) + 1.0 - d,
			d * (100.0 / b_w) + 1.0 - d,
		];

		let k = 1.0 / (5.0 * adapting_luminance + 1.0);

		let k4 = k * k * k * k;

		let k4_f = 1.0 - k4;

		let fl = k4 * adapting_luminance + 0.1 * k4_f * k4_f * (5.0 * adapting_luminance).cbrt();

		let n = y_from_lstar(background_lstar) / white_point[1];

		let z = 1.48 + n.sqrt();

		let nbb = 0.725 / n.powf(0.2);

		let ncb = nbb;

		let rgb_a_factors = [
			(fl * rgb_d[0] * r_w / 100f64).powf(0.42),
			(fl * rgb_d[1] * g_w / 100f64).powf(0.42),
			(fl * rgb_d[2] * b_w / 100f64).powf(0.42),
		];

		let rgb_a = [
			(400.0 * rgb_a_factors[0]) / (rgb_a_factors[0] + 27.13),
			(400.0 * rgb_a_factors[1]) / (rgb_a_factors[1] + 27.13),
			(400.0 * rgb_a_factors[2]) / (rgb_a_factors[2] + 27.13),
		];

		let aw = ((2.0 * rgb_a[0]) + rgb_a[1] + (0.05 * rgb_a[2])) * nbb;

		ViewingConditions {
			n,
			aw,
			nbb,
			ncb,
			c,
			nc,
			rgb_d,
			fl,
			fl_root: fl.powf(0.25),
			z,
		}
	}

	pub fn aw(&self) -> f64 {
		self.aw
	}

	pub fn nbb(&self) -> f64 {
		self.nbb
	}
	pub fn ncb(&self) -> f64 {
		self.ncb
	}
	pub fn c(&self) -> f64 {
		self.c
	}
	pub fn nc(&self) -> f64 {
		self.nc
	}
	pub fn n(&self) -> f64 {
		self.n
	}
	pub fn rgb_d(&self) -> [f64; 3] {
		self.rgb_d
	}
	pub fn fl(&self) -> f64 {
		self.fl
	}
	pub fn fl_root(&self) -> f64 {
		self.fl_root
	}
	pub fn z(&self) -> f64 {
		self.z
	}
}

impl Default for ViewingConditions {
	fn default() -> Self {
		Self::new(
			white_point_d65(),
			200.0 / PI * y_from_lstar(50.0) / 100.0,
			50.0,
			2.0,
			false,
		)
	}
}
