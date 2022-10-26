use crate::hct::Hct;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TonalPalette {
	tones: HashMap<u8, f64>,
	hue: f64,
	chroma: f64,
}

impl TonalPalette {
	fn new(hue: f64, chroma: f64) -> Self {
		Self {
			hue,
			chroma,
			tones: HashMap::new(),
		}
	}

	pub fn from_argb(argb: f64) -> Self {
		let hct = Hct::from_argb(argb);
		Self::new(hct.hue(), hct.chroma())
	}

	pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> Self {
		Self::new(hue, chroma)
	}

	pub fn tone(&mut self, tone: u8) -> f64 {
		if let Some(t) = self.tones.get(&tone) {
			*t
		} else {
			let argb = Hct::from(self.hue, self.chroma, tone as f64).to_int();
			self.tones.insert(tone, argb);
			argb
		}
	}
}
