use super::consts::BLUE;
use color_utilities::scheme::Scheme;
use color_utilities::utils::color;

#[test]
fn blue_light_scheme() {
	let scheme = Scheme::light(BLUE);
	assert_eq!(scheme.primary(), 0xFF343DFFu32 as f64)
}

#[test]
#[ignore = "this fails color coversion need be fixed"]
fn blue_dark_scheme() {
	let scheme = Scheme::light(BLUE);

	let e = 0xFFBEC2FFu32 as f64;
	let (er, eg, eb, ea) = (
		color::red_from_argb(e),
		color::green_from_argb(e),
		color::blue_from_argb(e),
		color::alpha_from_argb(e),
	);
	println!("expected:\n\n R:{er}\nG:{eg}\nB:{eb}\nA:{ea}");
	let p = scheme.primary();
	let (pr, pg, pb, pa) = (
		color::red_from_argb(p),
		color::green_from_argb(p),
		color::blue_from_argb(p),
		color::alpha_from_argb(p),
	);
	println!("Result:\n\n R:{pr}\nG:{pg}\nB:{pb}\nA:{pa}");

	assert_eq!(scheme.primary(), 0xFFBEC2FFu32 as f64)
}

#[test]
fn third_party_light_scheme() {
	let scheme = Scheme::light(0xFF6750A4u32 as f64);

	assert_eq!(scheme.primary(), 0xff6750A4u32 as f64);
	assert_eq!(scheme.secondary(), 0xff625B71u32 as f64);
	assert_eq!(scheme.tertiary(), 0xff7E5260u32 as f64);
	assert_eq!(scheme.surface(), 0xfffffbffu32 as f64);
	assert_eq!(scheme.on_surface(), 0xff1c1b1eu32 as f64);
}

#[test]
fn third_party_dark_scheme() {
	let scheme = Scheme::dark(0xff6750A4u32 as f64);

	assert_eq!(scheme.primary(), 0xffcfbcffu32 as f64);
	assert_eq!(scheme.secondary(), 0xffcbc2dbu32 as f64);
	assert_eq!(scheme.tertiary(), 0xffefb8c8u32 as f64);
	assert_eq!(scheme.surface(), 0xff1c1b1eu32 as f64);
	assert_eq!(scheme.on_surface(), 0xffe6e1e6u32 as f64);
}

#[test]
#[ignore = "fails sometimes"]
fn light_scheme_from_high_chroma_color() {
	let scheme = Scheme::light(0xfffa2becu32 as f64);

	assert_eq!(scheme.primary(), 0xffab00a2u32 as f64);
	assert_eq!(scheme.on_primary(), 0xffffffffu32 as f64);
	assert_eq!(scheme.primary_container(), 0xffffd7f3u32 as f64);
	assert_eq!(scheme.on_primary_container(), 0xff390035u32 as f64);
	assert_eq!(scheme.secondary(), 0xff6e5868u32 as f64);
	assert_eq!(scheme.on_secondary(), 0xffffffffu32 as f64);
	assert_eq!(scheme.secondary_container(), 0xfff8daeeu32 as f64);
	assert_eq!(scheme.on_secondary_container(), 0xff271624u32 as f64);
	assert_eq!(scheme.tertiary(), 0xff815343u32 as f64);
	assert_eq!(scheme.on_tertiary(), 0xffffffffu32 as f64);
	assert_eq!(scheme.tertiary_container(), 0xffffdbd0u32 as f64);
	//assert_eq!(scheme.on_tertiary_container(), 0xff321207u32 as f64);
	//assert_eq!(scheme.error(), 0xffba1a1au32 as f64);
	assert_eq!(scheme.on_error(), 0xffffffffu32 as f64);
	//assert_eq!(scheme.error_container(), 0xffffdad6u32 as f64);
	//assert_eq!(scheme.on_error_container(), 0xff410002u32 as f64);
	assert_eq!(scheme.background(), 0xfffffbffu32 as f64);
	assert_eq!(scheme.on_background(), 0xff1f1a1du32 as f64);
	assert_eq!(scheme.surface(), 0xfffffbffu32 as f64);
	assert_eq!(scheme.on_surface(), 0xff1f1a1du32 as f64);
	assert_eq!(scheme.surface_variant(), 0xffeedee7u32 as f64);
	assert_eq!(scheme.on_surface_variant(), 0xff4e444bu32 as f64);
	assert_eq!(scheme.outline(), 0xff80747bu32 as f64);
	assert_eq!(scheme.outline_variant(), 0xffd2c2cbu32 as f64);
	assert_eq!(scheme.shadow(), 0xff000000u32 as f64);
	assert_eq!(scheme.scrim(), 0xff000000u32 as f64);
	assert_eq!(scheme.inverse_surface(), 0xff342f32u32 as f64);
	assert_eq!(scheme.inverse_on_surface(), 0xfff8eef2u32 as f64);
	assert_eq!(scheme.inverse_primary(), 0xffffabeeu32 as f64);
}

#[test]
#[ignore = "fails on error roles"]
fn dark_scheme_from_high_chroma_color() {
	let scheme = Scheme::dark(0xfffa2becu32 as f64);

	assert_eq!(scheme.primary(), 0xffffabeeu32 as f64);
	assert_eq!(scheme.on_primary(), 0xff5c0057u32 as f64);
	assert_eq!(scheme.primary_container(), 0xff83007bu32 as f64);
	assert_eq!(scheme.on_primary_container(), 0xffffd7f3u32 as f64);
	assert_eq!(scheme.secondary(), 0xffdbbed1u32 as f64);
	assert_eq!(scheme.on_secondary(), 0xff3e2a39u32 as f64);
	assert_eq!(scheme.secondary_container(), 0xff554050u32 as f64);
	assert_eq!(scheme.on_secondary_container(), 0xfff8daeeu32 as f64);
	assert_eq!(scheme.tertiary(), 0xfff5b9a5u32 as f64);
	assert_eq!(scheme.on_tertiary(), 0xff4c2619u32 as f64);
	assert_eq!(scheme.tertiary_container(), 0xff663c2du32 as f64);
	assert_eq!(scheme.on_tertiary_container(), 0xffffdbd0u32 as f64);
	//assert_eq!(scheme.error(), 0xffffb4abu32 as f64);
	//assert_eq!(scheme.on_error(), 0xff690005u32 as f64);
	//assert_eq!(scheme.error_container(), 0xff93000au32 as f64);
	//assert_eq!(scheme.on_error_container(), 0xffffb4abu32 as f64);
	assert_eq!(scheme.background(), 0xff1f1a1du32 as f64);
	assert_eq!(scheme.on_background(), 0xffeae0e4u32 as f64);
	assert_eq!(scheme.surface(), 0xff1f1a1du32 as f64);
	assert_eq!(scheme.on_surface(), 0xffeae0e4u32 as f64);
	assert_eq!(scheme.surface_variant(), 0xff4e444bu32 as f64);
	assert_eq!(scheme.on_surface_variant(), 0xffd2c2cbu32 as f64);
	assert_eq!(scheme.outline(), 0xff9a8d95u32 as f64);
	assert_eq!(scheme.outline_variant(), 0xff4e444bu32 as f64);
	assert_eq!(scheme.shadow(), 0xff000000u32 as f64);
	assert_eq!(scheme.scrim(), 0xff000000u32 as f64);
	assert_eq!(scheme.inverse_surface(), 0xffeae0e4u32 as f64);
	assert_eq!(scheme.inverse_on_surface(), 0xff342f32u32 as f64);
	assert_eq!(scheme.inverse_primary(), 0xffab00a2u32 as f64);
}

#[test]
#[ignore = "fails"]
fn light_content_scheme_from_high_chroma_color() {
	let _scheme = Scheme::light_content(0xfffa2becu32 as f64);

	//assert_eq!(scheme.primary(), 0xffab00a2u32 as f64);
	//assert_eq!(scheme.on_primary(), 0xffffffffu32 as f64);
	//assert_eq!(scheme.primary_container(), 0xffffd7f3u32 as f64);
	//assert_eq!(scheme.on_primary_container(), 0xff390035u32 as f64);
	//assert_eq!(scheme.secondary(), 0xff7f4e75u32 as f64);
	//assert_eq!(scheme.on_secondary(), 0xffffffffu32 as f64);
	//assert_eq!(scheme.secondary_container(), 0xffffd7f3u32 as f64);
	//assert_eq!(scheme.on_secondary_container(), 0xff330b2fu32 as f64);
	//assert_eq!(scheme.tertiary(), 0xff9c4323u32 as f64);
	//assert_eq!(scheme.on_tertiary(), 0xffffffffu32 as f64);
	//assert_eq!(scheme.tertiary_container(), 0xffffdbd0u32 as f64);
	//assert_eq!(scheme.on_tertiary_container(), 0xff390c00u32 as f64);
	//assert_eq!(scheme.error(), 0xffba1a1au32 as f64);
	//assert_eq!(scheme.on_error(), 0xffffffffu32 as f64);
	//assert_eq!(scheme.error_container(), 0xffffdad6u32 as f64);
	//assert_eq!(scheme.on_error_container(), 0xff410002u32 as f64);
	//assert_eq!(scheme.background(), 0xfffffbffu32 as f64);
	//assert_eq!(scheme.on_background(), 0xff1f1a1du32 as f64);
	//assert_eq!(scheme.surface(), 0xfffffbffu32 as f64);
	//assert_eq!(scheme.on_surface(), 0xff1f1a1du32 as f64);
	//assert_eq!(scheme.surface_variant(), 0xffeedee7u32 as f64);
	//assert_eq!(scheme.on_surface_variant(), 0xff4e444bu32 as f64);
	//assert_eq!(scheme.outline(), 0xff80747bu32 as f64);
	//assert_eq!(scheme.outline_variant(), 0xffd2c2cbu32 as f64);
	//assert_eq!(scheme.shadow(), 0xff000000u32 as f64);
	//assert_eq!(scheme.scrim(), 0xff000000u32 as f64);
	//assert_eq!(scheme.inverse_surface(), 0xff342f32u32 as f64);
	//assert_eq!(scheme.inverse_on_surface(), 0xfff8eef2u32 as f64);
	//assert_eq!(scheme.inverse_primary(), 0xffffabeeu32 as f64);
}

#[test]
#[ignore = "fails on error roles"]
fn dark_content_scheme_from_high_chroma_color() {
	let scheme = Scheme::dark_content(0xfffa2becu32 as f64);

	assert_eq!(scheme.primary(), 0xffffabeeu32 as f64);
	assert_eq!(scheme.on_primary(), 0xff5c0057u32 as f64);
	assert_eq!(scheme.primary_container(), 0xff83007bu32 as f64);
	assert_eq!(scheme.on_primary_container(), 0xffffd7f3u32 as f64);
	assert_eq!(scheme.secondary(), 0xfff0b4e1u32 as f64);
	assert_eq!(scheme.on_secondary(), 0xff4b2145u32 as f64);
	assert_eq!(scheme.secondary_container(), 0xff64375cu32 as f64);
	assert_eq!(scheme.on_secondary_container(), 0xffffd7f3u32 as f64);
	assert_eq!(scheme.tertiary(), 0xffffb59cu32 as f64);
	assert_eq!(scheme.on_tertiary(), 0xff5c1900u32 as f64);
	assert_eq!(scheme.tertiary_container(), 0xff7d2c0du32 as f64);
	assert_eq!(scheme.on_tertiary_container(), 0xffffdbd0u32 as f64);
	//assert_eq!(scheme.error(), 0xffffb4abu32 as f64);
	//assert_eq!(scheme.on_error(), 0xff690005u32 as f64);
	//assert_eq!(scheme.error_container(), 0xff93000au32 as f64);
	//assert_eq!(scheme.on_error_container(), 0xffffb4abu32 as f64);
	assert_eq!(scheme.background(), 0xff1f1a1du32 as f64);
	assert_eq!(scheme.on_background(), 0xffeae0e4u32 as f64);
	assert_eq!(scheme.surface(), 0xff1f1a1du32 as f64);
	assert_eq!(scheme.on_surface(), 0xffeae0e4u32 as f64);
	assert_eq!(scheme.surface_variant(), 0xff4e444bu32 as f64);
	assert_eq!(scheme.on_surface_variant(), 0xffd2c2cbu32 as f64);
	assert_eq!(scheme.outline(), 0xff9a8d95u32 as f64);
	assert_eq!(scheme.outline_variant(), 0xff4e444bu32 as f64);
	assert_eq!(scheme.shadow(), 0xff000000u32 as f64);
	assert_eq!(scheme.scrim(), 0xff000000u32 as f64);
	assert_eq!(scheme.inverse_surface(), 0xffeae0e4u32 as f64);
	assert_eq!(scheme.inverse_on_surface(), 0xff342f32u32 as f64);
	assert_eq!(scheme.inverse_primary(), 0xffab00a2u32 as f64);
}
