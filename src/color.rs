use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug, Default, Clone, Copy)]
pub struct RGB256 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RGB64 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RGBA256 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Grayscale256 {
	pub v: u8,
	pub a: u8,
}

#[derive(Debug, Clone)]
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
				format!("Pixels size is not multiple of 4 (RGBA): {}", pixels_size),
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
					"Pixels size is not multiple of 2 (Grayscale): {}",
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

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write,
	{
		use self::Pixels::*;
		match self {
			RGBA(cols) => {
				for col in cols.iter() {
					wtr.write_u8(col.r)?;
					wtr.write_u8(col.g)?;
					wtr.write_u8(col.b)?;
					wtr.write_u8(col.a)?;
				}
			}
			Grayscale(cols) => {
				for col in cols.iter() {
					wtr.write_u8(col.v)?;
					wtr.write_u8(col.a)?;
				}
			}
			Indexed(indices) => {
				wtr.write(indices)?;
			}
		}
		Ok(())
	}

}
