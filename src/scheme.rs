use crate::{palettes::CorePalette, utils::color::ARGB};

pub struct Scheme {
	primary: ARGB,
	on_primary: ARGB,
	primary_container: ARGB,
	on_primary_container: ARGB,
	secondary: ARGB,
	on_secondary: ARGB,
	secondary_container: ARGB,
	on_secondary_container: ARGB,
	tertiary: ARGB,
	on_tertiary: ARGB,
	tertiary_container: ARGB,
	on_tertiary_container: ARGB,
	error: ARGB,
	on_error: ARGB,
	error_container: ARGB,
	on_error_container: ARGB,
	background: ARGB,
	on_background: ARGB,
	surface: ARGB,
	on_surface: ARGB,
	surface_variant: ARGB,
	on_surface_variant: ARGB,
	outline: ARGB,
	outline_variant: ARGB,
	shadow: ARGB,
	scrim: ARGB,
	inverse_surface: ARGB,
	inverse_on_surface: ARGB,
	inverse_primary: ARGB,
}

impl Scheme {
	pub fn light(argb: ARGB) -> Self {
		Self::light_from_core_palette(&mut CorePalette::of(argb))
	}

	pub fn dark(argb: ARGB) -> Self {
		Self::dark_from_core_palette(&mut CorePalette::of(argb))
	}

	pub fn light_content(argb: ARGB) -> Self {
		Self::light_from_core_palette(&mut CorePalette::content_of(argb))
	}

	pub fn dark_content(argb: ARGB) -> Self {
		Self::dark_from_core_palette(&mut CorePalette::content_of(argb))
	}

	pub fn light_from_core_palette(core: &mut CorePalette) -> Self {
		Self::new(
			core.a1().tone(40),
			core.a1().tone(100),
			core.a1().tone(90),
			core.a1().tone(10),
			core.a2().tone(40),
			core.a2().tone(100),
			core.a2().tone(90),
			core.a2().tone(10),
			core.a3().tone(40),
			core.a3().tone(100),
			core.a3().tone(90),
			core.a3().tone(10),
			core.error().tone(40),
			core.error().tone(100),
			core.error().tone(90),
			core.error().tone(10),
			core.n1().tone(99),
			core.n1().tone(10),
			core.n1().tone(99),
			core.n1().tone(10),
			core.n2().tone(90),
			core.n2().tone(30),
			core.n2().tone(50),
			core.n2().tone(80),
			core.n1().tone(0),
			core.n1().tone(0),
			core.n1().tone(20),
			core.n1().tone(95),
			core.a1().tone(80),
		)
	}

	pub fn dark_from_core_palette(core: &mut CorePalette) -> Self {
		Self::new(
			core.a1().tone(80),
			core.a1().tone(20),
			core.a1().tone(30),
			core.a1().tone(90),
			core.a2().tone(80),
			core.a2().tone(20),
			core.a2().tone(30),
			core.a2().tone(90),
			core.a3().tone(80),
			core.a3().tone(20),
			core.a3().tone(30),
			core.a3().tone(90),
			core.error().tone(80),
			core.error().tone(20),
			core.error().tone(30),
			core.error().tone(80),
			core.n1().tone(10),
			core.n1().tone(90),
			core.n1().tone(10),
			core.n1().tone(90),
			core.n2().tone(30),
			core.n2().tone(80),
			core.n2().tone(60),
			core.n2().tone(30),
			core.n1().tone(0),
			core.n1().tone(0),
			core.n1().tone(90),
			core.n1().tone(20),
			core.a1().tone(40),
		)
	}
}

impl Scheme {
	#[allow(clippy::too_many_arguments)]
	fn new(
		primary: ARGB,
		on_primary: ARGB,
		primary_container: ARGB,
		on_primary_container: ARGB,
		secondary: ARGB,
		on_secondary: ARGB,
		secondary_container: ARGB,
		on_secondary_container: ARGB,
		tertiary: ARGB,
		on_tertiary: ARGB,
		tertiary_container: ARGB,
		on_tertiary_container: ARGB,
		error: ARGB,
		on_error: ARGB,
		error_container: ARGB,
		on_error_container: ARGB,
		background: ARGB,
		on_background: ARGB,
		surface: ARGB,
		on_surface: ARGB,
		surface_variant: ARGB,
		on_surface_variant: ARGB,
		outline: ARGB,
		outline_variant: ARGB,
		shadow: ARGB,
		scrim: ARGB,
		inverse_surface: ARGB,
		inverse_on_surface: ARGB,
		inverse_primary: ARGB,
	) -> Self {
		Self {
			primary,
			on_primary,
			primary_container,
			on_primary_container,
			secondary,
			on_secondary,
			secondary_container,
			on_secondary_container,
			tertiary,
			on_tertiary,
			tertiary_container,
			on_tertiary_container,
			error,
			on_error,
			error_container,
			on_error_container,
			background,
			on_background,
			surface,
			on_surface,
			surface_variant,
			on_surface_variant,
			outline,
			outline_variant,
			shadow,
			scrim,
			inverse_surface,
			inverse_on_surface,
			inverse_primary,
		}
	}

	pub fn primary(&self) -> ARGB {
		self.primary
	}
	pub fn on_primary(&self) -> ARGB {
		self.on_primary
	}
	pub fn primary_container(&self) -> ARGB {
		self.primary_container
	}
	pub fn on_primary_container(&self) -> ARGB {
		self.on_primary_container
	}
	pub fn secondary(&self) -> ARGB {
		self.secondary
	}
	pub fn on_secondary(&self) -> ARGB {
		self.on_secondary
	}
	pub fn secondary_container(&self) -> ARGB {
		self.secondary_container
	}
	pub fn on_secondary_container(&self) -> ARGB {
		self.on_secondary_container
	}
	pub fn tertiary(&self) -> ARGB {
		self.tertiary
	}
	pub fn on_tertiary(&self) -> ARGB {
		self.on_tertiary
	}
	pub fn tertiary_container(&self) -> ARGB {
		self.tertiary_container
	}
	pub fn on_tertiary_container(&self) -> ARGB {
		self.on_tertiary_container
	}
	pub fn error(&self) -> ARGB {
		self.error
	}
	pub fn on_error(&self) -> ARGB {
		self.on_error
	}
	pub fn error_container(&self) -> ARGB {
		self.error_container
	}
	pub fn on_error_container(&self) -> ARGB {
		self.on_error_container
	}
	pub fn background(&self) -> ARGB {
		self.background
	}
	pub fn on_background(&self) -> ARGB {
		self.on_background
	}
	pub fn surface(&self) -> ARGB {
		self.surface
	}
	pub fn on_surface(&self) -> ARGB {
		self.on_surface
	}
	pub fn surface_variant(&self) -> ARGB {
		self.surface_variant
	}
	pub fn on_surface_variant(&self) -> ARGB {
		self.on_surface_variant
	}
	pub fn outline(&self) -> ARGB {
		self.outline
	}
	pub fn outline_variant(&self) -> ARGB {
		self.outline_variant
	}
	pub fn shadow(&self) -> ARGB {
		self.shadow
	}
	pub fn scrim(&self) -> ARGB {
		self.scrim
	}
	pub fn inverse_surface(&self) -> ARGB {
		self.inverse_surface
	}
	pub fn inverse_on_surface(&self) -> ARGB {
		self.inverse_on_surface
	}
	pub fn inverse_primary(&self) -> ARGB {
		self.inverse_primary
	}
}
