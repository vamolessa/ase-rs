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

pub struct RGBA256 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

pub struct Grayscale256 {
	pub v: u8,
	pub a: u8,
}

pub enum Pixels {
	RGBA(Vec<RGBA256>),
	Grayscale(Vec<Grayscale256>),
	Indexed(Vec<u8>),
}

impl Pixels {
	pub fn rgba_from_read<R>(read: &mut R, pixels_size: usize) -> io::Result<Self>
	where
		R: Read,
	{
		const BYTES_PER_PIXEL: usize = 4;
		if pixels_size % BYTES_PER_PIXEL != 0 {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				format!("Pixels Size is not multiple of 4 (RGBA): {}", pixels_size),
			));
		}

		let pixel_count = pixels_size / BYTES_PER_PIXEL;
		let mut pixels = Vec::with_capacity(pixel_count);

		for _ in 0..pixel_count {
			pixels.push(RGBA256 {
				r: read.read_u8()?,
				g: read.read_u8()?,
				b: read.read_u8()?,
				a: read.read_u8()?,
			});
		}

		Ok(Pixels::RGBA(pixels))
	}

	pub fn grayscale_from_read<R>(read: &mut R, pixels_size: usize) -> io::Result<Self>
	where
		R: Read,
	{
		const BYTES_PER_PIXEL: usize = 2;
		if pixels_size % BYTES_PER_PIXEL != 0 {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				format!(
					"Pixels Size is not multiple of 2 (Grayscale): {}",
					pixels_size
				),
			));
		}

		let pixel_count = pixels_size / BYTES_PER_PIXEL;
		let mut pixels = Vec::with_capacity(pixel_count);

		for _ in 0..pixel_count {
			pixels.push(Grayscale256 {
				v: read.read_u8()?,
				a: read.read_u8()?,
			});
		}

		Ok(Pixels::Grayscale(pixels))
	}

	pub fn indexed_from_read<R>(read: &mut R, pixels_size: usize) -> io::Result<Self>
	where
		R: Read,
	{
		let index_count = pixels_size;
		let mut indices = Vec::with_capacity(index_count);

		for _ in 0..index_count {
			indices.push(read.read_u8()?);
		}

		Ok(Pixels::Indexed(indices))
	}
}
