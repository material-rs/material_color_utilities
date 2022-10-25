use crate::utils::math as math_utils;

use super::math::matrix_multiply;

//https://github.com/material-foundation/material-color-utilities/blob/main/typescript/utils/color_utils.ts

const SRGB_TO_XYZ: [[f64; 3]; 3] = [
	[0.41233895, 0.35762064, 0.18051042],
	[0.2126, 0.7152, 0.0722],
	[0.01932141, 0.11916382, 0.95034478],
];

const XYZ_TO_SRGB: [[f64; 3]; 3] = [
	[
		3.2413774792388685,
		-1.5376652402851851,
		-0.49885366846268053,
	],
	[-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
	[
		0.05562093689691305,
		-0.20395524564742123,
		1.0571799111220335,
	],
];

const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

pub fn argb_from_rgb(red: f64, green: f64, blue: f64) -> f64 {
	let red = red as u32;
	let green = green as u32;
	let blue = blue as u32;

	let res = 255 << 24 | (red & 255) << 16 | (green & 255) << 8 | blue & 255;

	res as f64
}

pub fn argb_from_linrgb(linrgb: [f64; 3]) -> f64 {
	let r = delinearized(linrgb[0]);
	let g = delinearized(linrgb[1]);
	let b = delinearized(linrgb[2]);
	argb_from_rgb(r, g, b)
}

pub fn alpha_from_argb(argb: f64) -> f64 {
	((argb as u32) >> 24 & 255) as f64
}

pub fn red_from_argb(argb: f64) -> f64 {
	((argb as u32) >> 16 & 255) as f64
}

pub fn green_from_argb(argb: f64) -> f64 {
	((argb as u32) >> 8 & 255) as f64
}

pub fn blue_from_argb(argb: f64) -> f64 {
	((argb as u32) & 255) as f64
}

pub fn is_opaque(argb: f64) -> bool {
	alpha_from_argb(argb) >= 255.0
}

pub fn argb_from_xyz(x: f64, y: f64, z: f64) -> f64 {
	let [linear_r, linear_g, linear_b] = matrix_multiply([x, y, z], XYZ_TO_SRGB);

	let r = delinearized(linear_r);
	let g = delinearized(linear_g);
	let b = delinearized(linear_b);

	argb_from_rgb(r, g, b)
}

pub fn xyz_from_argb(argb: f64) -> [f64; 3] {
	let r = linearized(red_from_argb(argb));
	let g = linearized(green_from_argb(argb));
	let b = linearized(blue_from_argb(argb));

	math_utils::matrix_multiply([r, g, b], SRGB_TO_XYZ)
}

pub fn argb_from_lab(l: f64, a: f64, b: f64) -> f64 {
	let fy = (l + 16.0) / 116.0;
	let fx = a / 500.0 + fy;
	let fz = fy - b / 200.0;

	let x_normalized = lab_inv_f(fx);
	let y_normalized = lab_inv_f(fy);
	let z_normalized = lab_inv_f(fz);

	let x = x_normalized * WHITE_POINT_D65[0];
	let y = y_normalized * WHITE_POINT_D65[1];
	let z = z_normalized * WHITE_POINT_D65[2];

	argb_from_xyz(x, y, z)
}

pub fn lab_from_argb(argb: f64) -> [f64; 3] {
	let linear_r = linearized(red_from_argb(argb));
	let linear_g = linearized(green_from_argb(argb));
	let linear_b = linearized(blue_from_argb(argb));

	let [x, y, z] = matrix_multiply([linear_r, linear_g, linear_b], SRGB_TO_XYZ);

	let x_normalized = x / WHITE_POINT_D65[0];
	let y_normalized = y / WHITE_POINT_D65[1];
	let z_normalized = z / WHITE_POINT_D65[2];

	let fx = lab_f(x_normalized);
	let fy = lab_f(y_normalized);
	let fz = lab_f(z_normalized);

	let l = 116.0 * fy - 16.0;
	let a = 500.0 * (fx - fy);
	let b = 200.0 * (fy - fz);

	[l, a, b]
}

pub fn argb_from_lstar(lstar: f64) -> f64 {
	let y = y_from_lstar(lstar);
	let component = delinearized(y);
	argb_from_rgb(component, component, component)
}

pub fn lstar_from_argb(argb: f64) -> f64 {
	let y = xyz_from_argb(argb)[1];
	116.0 * lab_f(y / 100.0) - 16.0
}

pub fn y_from_lstar(lstar: f64) -> f64 {
	100.0 * lab_inv_f((lstar + 16.0) / 116.0)
}

pub fn linearized(rgb_component: f64) -> f64 {
	//assert!((0.0..=255.0).contains(&rgb_component));

	let normalized = rgb_component / 255.0;
	if normalized <= 0.040449936 {
		normalized / 12.92 * 100.0
	} else {
		((normalized + 0.055) / 1.055).powf(2.4) * 100.0
	}
}

pub fn delinearized(rgb_component: f64) -> f64 {
	//assert!(rgb_component >= 0.0 && rgb_component <= 100.0);

	let normalized = rgb_component / 100.0;

	let delinearized = if normalized <= 0.0031308 {
		normalized * 19.92
	} else {
		1.055 * normalized.powf(1.0 / 2.4) - 0.055
	};

	(delinearized * 255.0).round().clamp(0.0, 255.0)
}

pub const fn white_point_d65() -> [f64; 3] {
	WHITE_POINT_D65
}

fn lab_f(t: f64) -> f64 {
	let e = 216.0 / 24389.0;
	let kappa = 24389.0 / 27.0;
	if t > e {
		t.powf(1.0 / 3.0)
	} else {
		(kappa * t + 16.0) / 116.0
	}
}

fn lab_inv_f(ft: f64) -> f64 {
	let e = 216.0 / 24389.0;
	let kappa = 24389.0 / 27.0;
	let ft3 = ft * ft * ft;
	if ft3 > e {
		ft3
	} else {
		(116.0 * ft - 16.0) / kappa
	}
}
