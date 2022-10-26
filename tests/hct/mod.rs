use assert_approx_eq::assert_approx_eq;
use color_utilities::{
	hct::{cam16::Cam16, viewing_conditions::ViewingConditions, Hct},
	utils::color,
};

use super::consts::*;

mod cam_from_argb {
	use super::*;

	#[test]
	fn red() {
		let cam = Cam16::from_argb(RED);

		assert_approx_eq!(cam.hue(), 27.408, 0.001);
		assert_approx_eq!(cam.chroma(), 113.358, 0.001);
		assert_approx_eq!(cam.j(), 46.445, 0.001);
		assert_approx_eq!(cam.m(), 89.494, 0.001);
		assert_approx_eq!(cam.s(), 91.890, 0.001);
		assert_approx_eq!(cam.q(), 105.989, 0.001);
	}

	#[test]
	fn green() {
		let cam = Cam16::from_argb(GREEN);

		assert_approx_eq!(cam.hue(), 142.140, 0.001);
		assert_approx_eq!(cam.chroma(), 108.410, 0.001);
		assert_approx_eq!(cam.j(), 79.332, 0.001);
		assert_approx_eq!(cam.m(), 85.588, 0.001);
		assert_approx_eq!(cam.s(), 78.605, 0.001);
		assert_approx_eq!(cam.q(), 138.520, 0.001);
	}

	#[test]
	fn blue() {
		let cam = Cam16::from_argb(BLUE);

		assert_approx_eq!(cam.hue(), 282.788, 0.001);
		assert_approx_eq!(cam.chroma(), 87.231, 0.001);
		assert_approx_eq!(cam.j(), 25.466, 0.001);
		assert_approx_eq!(cam.m(), 68.867, 0.001);
		assert_approx_eq!(cam.s(), 93.675, 0.001);
		assert_approx_eq!(cam.q(), 78.481, 0.001);
	}

	#[test]
	fn white() {
		let cam = Cam16::from_argb(WHITE);

		assert_approx_eq!(cam.hue(), 209.492, 0.001);
		assert_approx_eq!(cam.chroma(), 2.869, 0.001);
		assert_approx_eq!(cam.j(), 100.0, 0.001);
		assert_approx_eq!(cam.m(), 2.265, 0.001);
		assert_approx_eq!(cam.s(), 12.068, 0.001);
		assert_approx_eq!(cam.q(), 155.521, 0.001);
	}

	#[test]
	fn black() {
		let cam = Cam16::from_argb(BLACK);

		assert_approx_eq!(cam.hue(), 0.0, 0.001);
		assert_approx_eq!(cam.chroma(), 0.0, 0.001);
		assert_approx_eq!(cam.j(), 0.0, 0.001);
		assert_approx_eq!(cam.m(), 0.0, 0.001);
		assert_approx_eq!(cam.s(), 0.0, 0.001);
		assert_approx_eq!(cam.q(), 0.0, 0.001);
	}
}

mod cam_to_argb_to_cam {
	use super::*;
	#[test]
	fn red() {
		let cam = Cam16::from_argb(RED);
		let argb = cam.to_int();

		assert_eq!(RED, argb);
	}

	#[test]
	fn green() {
		let cam = Cam16::from_argb(GREEN);
		let argb = cam.to_int();

		assert_eq!(GREEN, argb);
	}

	#[test]
	fn blue() {
		let cam = Cam16::from_argb(BLUE);
		let argb = cam.to_int();

		assert_eq!(BLUE, argb);
	}

	#[test]
	fn white() {
		let cam = Cam16::from_argb(WHITE);
		let argb = cam.to_int();

		assert_eq!(WHITE, argb);
	}

	#[test]
	fn black() {
		let cam = Cam16::from_argb(BLACK);
		let argb = cam.to_int();

		assert_eq!(BLACK, argb);
	}
}

mod argb_to_hct {
	use super::*;

	#[test]
	fn green() {
		let hct = Hct::from_argb(GREEN);

		assert_approx_eq!(hct.hue(), 142.139, 0.001);
		assert_approx_eq!(hct.chroma(), 108.410, 0.001);
		assert_approx_eq!(hct.tone(), 87.737, 0.001);
	}

	#[test]
	fn blue() {
		let hct = Hct::from_argb(BLUE);

		assert_approx_eq!(hct.hue(), 282.788, 0.001);
		assert_approx_eq!(hct.chroma(), 87.230, 0.001);
		assert_approx_eq!(hct.tone(), 32.302, 0.001);
	}

	#[test]
	fn blue_tone_90() {
		let hct = Hct::from(282.788, 87.230, 90.0);

		assert_approx_eq!(hct.hue(), 282.239, 0.001);
		assert_approx_eq!(hct.chroma(), 19.144, 0.001);
		assert_approx_eq!(hct.tone(), 90.035, 0.001);
	}
}

#[test]
fn viewing_conditions() {
	let vc = ViewingConditions::default();

	assert_approx_eq!(vc.n(), 0.184, 0.001);
	assert_approx_eq!(vc.aw(), 29.981, 0.001);
	assert_approx_eq!(vc.nbb(), 1.017, 0.001);
	assert_approx_eq!(vc.ncb(), 1.017, 0.001);
	assert_approx_eq!(vc.c(), 0.69, 0.001);
	assert_approx_eq!(vc.nc(), 1.0, 0.001);
	assert_approx_eq!(vc.rgb_d()[0], 1.021, 0.001);
	assert_approx_eq!(vc.rgb_d()[1], 0.986, 0.001);
	assert_approx_eq!(vc.rgb_d()[2], 0.934, 0.001);
	assert_approx_eq!(vc.fl(), 0.388, 0.001);
	assert_approx_eq!(vc.fl_root(), 0.789, 0.001);
	assert_approx_eq!(vc.z(), 1.909, 0.001);
}

#[test]
fn cam_solver() {
	let color_is_on_boundary = |argb: f64| -> bool {
		color::red_from_argb(argb) == 0.0
			|| color::red_from_argb(argb) == 255.0
			|| color::green_from_argb(argb) == 0.0
			|| color::green_from_argb(argb) == 255.0
			|| color::blue_from_argb(argb) == 0.0
			|| color::blue_from_argb(argb) == 255.0
	};

	for hue in (15..360).step_by(30) {
		for chroma in (0..=100).step_by(10) {
			for tone in (20..=80).step_by(10) {
				let hct_color = Hct::from(hue as f64, chroma as f64, tone as f64);

				if chroma > 0 {
					assert!((hct_color.hue() - hue as f64).abs() <= 4.0);
				}

				assert!(hct_color.chroma() >= 0.0, "fails on higher");
				assert!(hct_color.chroma() <= chroma as f64 + 2.5);

				if hct_color.chroma() < chroma as f64 - 2.55 {
					assert!(color_is_on_boundary(hct_color.to_int()));
				}

				assert!((hct_color.tone() - tone as f64).abs() <= 0.5)
			}
		}
	}
}

#[test]
fn hct_roundtrip() {
	for r in (0..296).step_by(37) {
		for g in (0..296).step_by(37) {
			for b in (0..296).step_by(37) {
				let (r, g, b) = (r as f64, g as f64, b as f64);
				let argb = color::argb_from_rgb(255f64.min(r), 255f64.min(g), 255f64.min(b));

				let hct = Hct::from_argb(argb);
				let hct_2 = Hct::from(hct.hue(), hct.chroma(), hct.tone());

				assert_approx_eq!(argb, hct_2.to_int(), 0.001);

				assert_approx_eq!(hct.hue(), hct_2.hue(), 0.001);
				assert_approx_eq!(hct.chroma(), hct_2.chroma(), 0.001);
				assert_approx_eq!(hct.tone(), hct_2.tone(), 0.001);
				assert_approx_eq!(hct.argb(), hct_2.argb(), 0.001);
			}
		}
	}
}
