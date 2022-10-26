use super::tonal::TonalPalette;
use crate::{hct::Hct, utils::color::ARGB};

#[derive(Debug, Clone)]
pub struct CorePalette {
	a1: TonalPalette,
	a2: TonalPalette,
	a3: TonalPalette,
	n1: TonalPalette,
	n2: TonalPalette,
	error: TonalPalette,
}

impl CorePalette {
	pub fn of(argb: ARGB) -> Self {
		Self::new(argb, false)
	}

	pub fn content_of(argb: ARGB) -> Self {
		Self::new(argb, true)
	}

	fn new(argb: ARGB, is_content: bool) -> Self {
		let hct = Hct::from_argb(argb);
		let hue = hct.hue();
		let chroma = hct.chroma();
		let error = TonalPalette::from_hue_and_chroma(25.0, 84.0);

		if is_content {
			Self {
				a1: TonalPalette::from_hue_and_chroma(hue, chroma),
				a2: TonalPalette::from_hue_and_chroma(hue, chroma / 3.0),
				a3: TonalPalette::from_hue_and_chroma(hue + 60.0, chroma / 2.0),
				n1: TonalPalette::from_hue_and_chroma(hue, (chroma / 12.0).min(4.0)),
				n2: TonalPalette::from_hue_and_chroma(hue, (chroma / 6.0).min(8.0)),
				error,
			}
		} else {
			Self {
				a1: TonalPalette::from_hue_and_chroma(hue, 48f64.max(chroma)),
				a2: TonalPalette::from_hue_and_chroma(hue, 16.0),
				a3: TonalPalette::from_hue_and_chroma(hue + 60.0, 24.0),
				n1: TonalPalette::from_hue_and_chroma(hue, 4.0),
				n2: TonalPalette::from_hue_and_chroma(hue, 8.0),
				error,
			}
		}
	}

	pub fn a1(&mut self) -> &mut TonalPalette {
		&mut self.a1
	}

	pub fn a2(&mut self) -> &mut TonalPalette {
		&mut self.a2
	}

	pub fn a3(&mut self) -> &mut TonalPalette {
		&mut self.a3
	}

	pub fn n1(&mut self) -> &mut TonalPalette {
		&mut self.n1
	}

	pub fn n2(&mut self) -> &mut TonalPalette {
		&mut self.n2
	}

	pub fn error(&mut self) -> &mut TonalPalette {
		&mut self.error
	}

	pub fn custom(a1: ARGB, a2: ARGB, a3: ARGB, n1: ARGB, n2: ARGB, error: ARGB) -> Self {
		Self {
			a1: TonalPalette::from_argb(a1),
			a2: TonalPalette::from_argb(a2),
			a3: TonalPalette::from_argb(a3),
			n1: TonalPalette::from_argb(n1),
			n2: TonalPalette::from_argb(n2),
			error: TonalPalette::from_argb(error),
		}
	}
}
