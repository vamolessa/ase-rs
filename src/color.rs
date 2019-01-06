use std::io::{self, Read};

use byteorder::ReadBytesExt;

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

impl Pixels {
	pub fn rgba_from_read<R>(read: &mut R, pixels_size: usize) -> io::Result<Self>
	where
		R: Read,
	{
		if pixels_size % 4 != 0 {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				format!("Pixels Size is not multiple of 4: {}", pixels_size),
			));
		}

		let pixel_count = pixels_size / 4;
		let mut pixels = Vec::with_capacity(pixel_count);

		for _ in 0..pixel_count {
			pixels.push(RGBA {
				r: read.read_u8()?,
				g: read.read_u8()?,
				b: read.read_u8()?,
				a: read.read_u8()?,
			});
		}

		Ok(Pixels::RGBA(pixels))
	}
}
