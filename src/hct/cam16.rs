use super::{viewing_conditions::ViewingConditions, CAM16RGB_TO_XYZ};
use crate::utils::{
	color::{argb_from_xyz, linearized, ARGB},
	math::matrix_multiply,
};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Cam16 {
	hue: f64,
	chroma: f64,
	j: f64,
	q: f64,
	m: f64,
	s: f64,
	jstar: f64,
	astar: f64,
	bstar: f64,
}

impl Cam16 {
	#[allow(clippy::too_many_arguments)]
	fn new(
		hue: f64,
		chroma: f64,
		j: f64,
		q: f64,
		m: f64,
		s: f64,
		jstar: f64,
		astar: f64,
		bstar: f64,
	) -> Self {
		Self {
			hue,
			chroma,
			j,
			q,
			m,
			s,
			jstar,
			astar,
			bstar,
		}
	}

	pub fn hue(&self) -> f64 {
		self.hue
	}

	pub fn chroma(&self) -> f64 {
		self.chroma
	}

	pub fn j(&self) -> f64 {
		self.j
	}

	pub fn q(&self) -> f64 {
		self.q
	}

	pub fn m(&self) -> f64 {
		self.m
	}

	pub fn s(&self) -> f64 {
		self.s
	}

	pub fn jstar(&self) -> f64 {
		self.jstar
	}

	pub fn astar(&self) -> f64 {
		self.astar
	}

	pub fn bstar(&self) -> f64 {
		self.bstar
	}

	pub fn distance(&self, other: Self) -> f64 {
		let d_j = self.jstar - other.jstar;
		let d_a = self.astar - other.astar;
		let d_b = self.bstar - other.bstar;
		let d_e_prime = (d_j.powi(2) + d_a.powi(2) + d_b.powi(2)).sqrt();
		1.41 * d_e_prime.powf(0.63)
	}

	pub fn from_argb(argb: ARGB) -> Self {
		Self::from_int_in_viewing_conditions(argb, ViewingConditions::default())
	}

	fn from_int_in_viewing_conditions(argb: ARGB, viewing_conditions: ViewingConditions) -> Self {
		let (red, green, blue) = (argb[1], argb[2], argb[3]);

		let red_l = linearized(red);
		let green_l = linearized(green);
		let blue_l = linearized(blue);

		let x = 0.41233895 * red_l + 0.35762064 * green_l + 0.18051042 * blue_l;
		let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
		let z = 0.01932141 * red_l + 0.11916382 * green_l + 0.95034478 * blue_l;

		let r_c = 0.401288 * x + 0.650173 * y - 0.051461 * z;
		let g_c = -0.250268 * x + 1.204414 * y + 0.045854 * z;
		let b_c = -0.002079 * x + 0.048952 * y + 0.953127 * z;

		let r_d = viewing_conditions.rgb_d()[0] * r_c;
		let g_d = viewing_conditions.rgb_d()[1] * g_c;
		let b_d = viewing_conditions.rgb_d()[2] * b_c;

		let r_a_f = (viewing_conditions.fl() * r_d.abs() / 100.0).powf(0.42);
		let g_a_f = (viewing_conditions.fl() * g_d.abs() / 100.0).powf(0.42);
		let b_a_f = (viewing_conditions.fl() * b_d.abs() / 100.0).powf(0.42);

		let r_a = (r_d.signum() * 400.0 * r_a_f) / (r_a_f + 27.13);
		let g_a = (g_d.signum() * 400.0 * g_a_f) / (g_a_f + 27.13);
		let b_a = (b_d.signum() * 400.0 * b_a_f) / (b_a_f + 27.13);

		let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
		let b = (r_a + g_a - 2.0 * b_a) / 9.0;
		let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
		let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;

		let atan2 = b.atan2(a);

		let atan_degrees = (atan2 * 180.0) / PI;

		let hue = if atan_degrees < 0.0 {
			atan_degrees + 360.0
		} else if atan_degrees >= 360.0 {
			atan_degrees - 360.0
		} else {
			atan_degrees
		};

		let hue_radians = hue.to_radians(); //(hue * PI) / 180.0;

		let ac = p2 * viewing_conditions.nbb();

		let j = 100.0
			* (ac / viewing_conditions.aw()).powf(viewing_conditions.c() * viewing_conditions.z());

		let q = (4.0 / viewing_conditions.c())
			* (j / 100.0).sqrt()
			* (viewing_conditions.aw() + 4.0)
			* viewing_conditions.fl_root();

		let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };

		let e_hue = 0.25 * ((hue_prime.to_radians() + 2.0).cos() + 3.8);

		let p1 = (50000.0 / 13.0) * e_hue * viewing_conditions.nc() * viewing_conditions.ncb();

		let t = (p1 * (a * a + b * b).sqrt()) / (u + 0.305);

		let alpha = (1.64 - 0.29f64.powf(viewing_conditions.n())).powf(0.73) * t.powf(0.9);

		let c = alpha * (j / 100.0).sqrt();

		let m = c * viewing_conditions.fl_root();

		let s = 50.0 * ((alpha * viewing_conditions.c()) / (viewing_conditions.aw() + 4.0)).sqrt();

		let jstar = ((1.0 + 100.0 * 0.007) * j) / (1.0 + 0.007 * j);
		let mstar = (1.0 / 0.0228) * (1.0 + 0.0228 * m).ln();
		let astar = mstar * hue_radians.cos();
		let bstar = mstar * hue_radians.sin();

		Self::new(hue, c, j, q, m, s, jstar, astar, bstar)
	}

	pub fn from_jch(j: f64, c: f64, h: f64) -> Self {
		Self::from_jch_in_viewing_conditions(j, c, h, ViewingConditions::default())
	}

	fn from_jch_in_viewing_conditions(
		j: f64,
		c: f64,
		h: f64,
		viewing_conditions: ViewingConditions,
	) -> Self {
		let q = (4.0 / viewing_conditions.c())
			* (j / 100.0).sqrt()
			* (viewing_conditions.aw() + 4.0)
			* viewing_conditions.fl_root();

		let m = c * viewing_conditions.fl_root();

		let alpha = c / (j / 100.0).sqrt();

		let s = 50.0 * ((alpha * viewing_conditions.c()) / (viewing_conditions.aw() + 4.0)).sqrt();

		let hue_radians = h.to_radians();

		let jstar = ((1.0 + 100.0 * 0.007) * j) / (1.0 + 0.007 * j);

		let mstar = (1.0 / 0.0228) * (1.0 + 0.0228 * m).ln_1p();

		let astar = mstar * hue_radians.cos();

		let bstar = mstar * hue_radians.sin();

		Cam16::new(h, c, j, q, m, s, jstar, astar, bstar)
	}

	pub fn from_ucs(jstar: f64, astar: f64, bstar: f64) -> Self {
		Self::from_ucs_in_viewing_conditions(jstar, astar, bstar, ViewingConditions::default())
	}

	fn from_ucs_in_viewing_conditions(
		jstar: f64,
		astar: f64,
		bstar: f64,
		viewing_conditions: ViewingConditions,
	) -> Self {
		let m = (astar.powi(2) + bstar.powi(2)).sqrt();

		let m2 = (m * 0.0228).exp_m1() / 0.0228;

		let c = m2 / viewing_conditions.fl_root();

		let mut h = bstar.atan2(astar) * (180.0 / PI);

		if h < 0.0 {
			h += 360.0;
		}
		let j = jstar / (1.0 - (jstar - 100.0) * 0.007);

		Cam16::from_jch_in_viewing_conditions(j, c, h, viewing_conditions)
	}

	pub fn to_int(&self) -> ARGB {
		self.into()
	}

	fn viewed(&self, viewing_conditions: ViewingConditions) -> ARGB {
		let alpha = if self.chroma() == 0.0 || self.j() == 0.0 {
			0.0
		} else {
			self.chroma() / (self.j() / 100.0).sqrt()
		};

		let t = (alpha / (1.64 - 0.29f64.powf(viewing_conditions.n())).powf(0.73)).powf(1.0 / 0.9);

		let h_rad = (self.hue() * PI) / 180.0;

		let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);

		let ac = viewing_conditions.aw()
			* (self.j() / 100.0).powf(1.0 / viewing_conditions.c() / viewing_conditions.z());

		let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.nc() * viewing_conditions.ncb();
		let p2 = ac / viewing_conditions.nbb();

		let h_sin = h_rad.sin();
		let h_cos = h_rad.cos();

		let gamma = (23.0 * (p2 + 0.305) * t) / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
		let a = gamma * h_cos;
		let b = gamma * h_sin;

		let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
		let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
		let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

		let r_c_base = (27.13 * r_a.abs()) / (400.0 - r_a.abs()).max(0.0);
		let r_c = r_a.signum() * (100.0 / viewing_conditions.fl()) * r_c_base.powf(1.0 / 0.42);

		let g_c_base = (27.13 * g_a.abs()) / (400.0 - g_a.abs()).max(0.0);
		let g_c = g_a.signum() * (100.0 / viewing_conditions.fl()) * g_c_base.powf(1.0 / 0.42);

		let b_c_base = (27.13 * b_a.abs()) / (400.0 - b_a.abs());
		let b_c = b_a.signum() * (100.0 / viewing_conditions.fl()) * b_c_base.powf(1.0 / 0.42);

		let r_f = r_c / viewing_conditions.rgb_d()[0];
		let g_f = g_c / viewing_conditions.rgb_d()[1];
		let b_f = b_c / viewing_conditions.rgb_d()[2];

		let argb = matrix_multiply([r_f, g_f, b_f], CAM16RGB_TO_XYZ);

		argb_from_xyz(argb[0], argb[1], argb[2])
	}
}

impl From<&Cam16> for ARGB {
	fn from(cam16: &Cam16) -> ARGB {
		cam16.viewed(ViewingConditions::default())
	}
}
