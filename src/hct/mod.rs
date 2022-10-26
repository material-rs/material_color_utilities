use crate::{
	hct::cam16::Cam16,
	utils::color::{lstar_from_argb, ARGB},
};

pub mod cam16;
pub mod hct_solver;
pub mod viewing_conditions;

pub(self) const XYZ_TO_CAM16RGB: [[f64; 3]; 3] = [
	[0.401288, 0.650173, -0.051461],
	[-0.250268, 1.204414, 0.045854],
	[-0.002079, 0.048952, 0.953127],
];

pub(self) const CAM16RGB_TO_XYZ: [[f64; 3]; 3] = [
	[1.8620678, -1.0112547, 0.14918678],
	[0.38752654, 0.62144744, -0.00897398],
	[-0.01584150, -0.03412294, 1.0499644],
];

#[derive(Debug, Default, PartialEq)]
pub struct Hct {
	hue: f64,
	chroma: f64,
	tone: f64,
	argb: ARGB,
}

impl Hct {
	pub fn from(hue: f64, chroma: f64, tone: f64) -> Self {
		let mut hct = Hct::default();
		hct.mut_internal_state(hct_solver::solve_to_int(hue, chroma, tone));
		hct
	}

	pub fn from_argb(argb: ARGB) -> Self {
		let mut hct = Hct::default();
		hct.mut_internal_state(argb);
		hct
	}

	pub fn hue(&self) -> f64 {
		self.hue
	}
	pub fn chroma(&self) -> f64 {
		self.chroma
	}
	pub fn tone(&self) -> f64 {
		self.tone
	}

	pub fn argb(&self) -> ARGB {
		self.argb
	}

	pub fn to_int(&self) -> ARGB {
		self.argb()
	}

	pub fn mut_hue(&mut self, hue: f64) {
		self.mut_internal_state(hct_solver::solve_to_int(hue, self.chroma(), self.tone()))
	}

	pub fn mut_chroma(&mut self, chroma: f64) {
		self.mut_internal_state(hct_solver::solve_to_int(self.hue(), chroma, self.tone()))
	}

	pub fn mut_tone(&mut self, tone: f64) {
		self.mut_internal_state(hct_solver::solve_to_int(self.hue(), self.chroma(), tone))
	}

	fn mut_internal_state(&mut self, argb: ARGB) {
		self.argb = argb;
		let cam = Cam16::from_argb(argb);
		self.hue = cam.hue();
		self.chroma = cam.chroma();
		self.tone = lstar_from_argb(argb);
	}
}
