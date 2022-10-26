use crate::palettes::CorePalette;

pub struct Scheme {
	primary: f64,
	on_primary: f64,
	primary_container: f64,
	on_primary_container: f64,
	secondary: f64,
	on_secondary: f64,
	secondary_container: f64,
	on_secondary_container: f64,
	tertiary: f64,
	on_tertiary: f64,
	tertiary_container: f64,
	on_tertiary_container: f64,
	error: f64,
	on_error: f64,
	error_container: f64,
	on_error_container: f64,
	background: f64,
	on_background: f64,
	surface: f64,
	on_surface: f64,
	surface_variant: f64,
	on_surface_variant: f64,
	outline: f64,
	outline_variant: f64,
	shadow: f64,
	scrim: f64,
	inverse_surface: f64,
	inverse_on_surface: f64,
	inverse_primary: f64,
}

impl Scheme {
	pub fn light(argb: f64) -> Self {
		Self::light_from_core_palette(&mut CorePalette::of(argb))
	}

	pub fn dark(argb: f64) -> Self {
		Self::dark_from_core_palette(&mut CorePalette::of(argb))
	}

	pub fn light_content(argb: f64) -> Self {
		Self::dark_from_core_palette(&mut CorePalette::content_of(argb))
	}

	pub fn dark_content(argb: f64) -> Self {
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
	fn new(
		primary: f64,
		on_primary: f64,
		primary_container: f64,
		on_primary_container: f64,
		secondary: f64,
		on_secondary: f64,
		secondary_container: f64,
		on_secondary_container: f64,
		tertiary: f64,
		on_tertiary: f64,
		tertiary_container: f64,
		on_tertiary_container: f64,
		error: f64,
		on_error: f64,
		error_container: f64,
		on_error_container: f64,
		background: f64,
		on_background: f64,
		surface: f64,
		on_surface: f64,
		surface_variant: f64,
		on_surface_variant: f64,
		outline: f64,
		outline_variant: f64,
		shadow: f64,
		scrim: f64,
		inverse_surface: f64,
		inverse_on_surface: f64,
		inverse_primary: f64,
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

	pub fn primary(&self) -> f64 {
		self.primary
	}
	pub fn on_primary(&self) -> f64 {
		self.on_primary
	}
	pub fn primary_container(&self) -> f64 {
		self.primary_container
	}
	pub fn on_primary_container(&self) -> f64 {
		self.on_primary_container
	}
	pub fn secondary(&self) -> f64 {
		self.secondary
	}
	pub fn on_secondary(&self) -> f64 {
		self.on_secondary
	}
	pub fn secondary_container(&self) -> f64 {
		self.secondary_container
	}
	pub fn on_secondary_container(&self) -> f64 {
		self.on_secondary_container
	}
	pub fn tertiary(&self) -> f64 {
		self.tertiary
	}
	pub fn on_tertiary(&self) -> f64 {
		self.on_tertiary
	}
	pub fn tertiary_container(&self) -> f64 {
		self.tertiary_container
	}
	pub fn on_tertiary_container(&self) -> f64 {
		self.on_tertiary_container
	}
	pub fn error(&self) -> f64 {
		self.error
	}
	pub fn on_error(&self) -> f64 {
		self.on_error
	}
	pub fn error_container(&self) -> f64 {
		self.error_container
	}
	pub fn on_error_container(&self) -> f64 {
		self.on_error_container
	}
	pub fn background(&self) -> f64 {
		self.background
	}
	pub fn on_background(&self) -> f64 {
		self.on_background
	}
	pub fn surface(&self) -> f64 {
		self.surface
	}
	pub fn on_surface(&self) -> f64 {
		self.on_surface
	}
	pub fn surface_variant(&self) -> f64 {
		self.surface_variant
	}
	pub fn on_surface_variant(&self) -> f64 {
		self.on_surface_variant
	}
	pub fn outline(&self) -> f64 {
		self.outline
	}
	pub fn outline_variant(&self) -> f64 {
		self.outline_variant
	}
	pub fn shadow(&self) -> f64 {
		self.shadow
	}
	pub fn scrim(&self) -> f64 {
		self.scrim
	}
	pub fn inverse_surface(&self) -> f64 {
		self.inverse_surface
	}
	pub fn inverse_on_surface(&self) -> f64 {
		self.inverse_on_surface
	}
	pub fn inverse_primary(&self) -> f64 {
		self.inverse_primary
	}
}
