pub struct RGB256 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

pub struct RGB64 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

pub struct RGBA {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

pub struct Grayscale {
	pub v: u8,
	pub a: u8,
}

pub enum Pixels {
	RGBA(Vec<RGBA>),
	Grayscale(Vec<Grayscale>),
	Indexed(Vec<u8>),
}
