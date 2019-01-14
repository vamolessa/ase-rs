use std::io::{self, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::color::Pixels;
use crate::helpers::read_bytes;
use crate::{ColorDepth, Header};

pub enum Cel {
	RawCel {
		width: u16,
		height: u16,
		pixels: Pixels,
	},
	LinkedCel {
		frame_position: u16,
	},
	CompressedImage {
		width: u16,
		height: u16,
		zlib_compressed_data: Vec<u8>,
	},
}

pub struct CelChunk {
	pub layer_index: u16,
	pub x_position: i16,
	pub y_position: i16,
	pub opacity_level: u8,
	pub cel: Cel,
}

impl CelChunk {
	pub fn from_read<R>(read: &mut R, chunk_data_size: u32, header: &Header) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let chunk_start = read.seek(SeekFrom::Current(0))?;

		let layer_index = read.read_u16::<LittleEndian>()?;
		let x_position = read.read_i16::<LittleEndian>()?;
		let y_position = read.read_i16::<LittleEndian>()?;
		let opacity_level = read.read_u8()?;
		let cel_type = read.read_u16::<LittleEndian>()?;

		let cel = match cel_type {
			0 => {
				let width = read.read_u16::<LittleEndian>()?;
				let height = read.read_u16::<LittleEndian>()?;

				let pixels_size =
					chunk_start + chunk_data_size as u64 - read.seek(SeekFrom::Current(0))?;
				let pixels = match header.color_depth {
					ColorDepth::Indexed => Pixels::indexed_from_read(read, pixels_size as usize)?,
					ColorDepth::Grayscale => {
						Pixels::grayscale_from_read(read, pixels_size as usize)?
					}
					ColorDepth::RGBA => Pixels::rgba_from_read(read, pixels_size as usize)?,
				};

				Cel::RawCel {
					width,
					height,
					pixels,
				}
			}
			1 => Cel::LinkedCel {
				frame_position: read.read_u16::<LittleEndian>()?,
			},
			2 => {
				let width = read.read_u16::<LittleEndian>()?;
				let height = read.read_u16::<LittleEndian>()?;

				let data_size =
					chunk_start + chunk_data_size as u64 - read.seek(SeekFrom::Current(0))?;
				let zlib_compressed_data = read_bytes(read, data_size as usize)?;

				Cel::CompressedImage {
					width,
					height,
					zlib_compressed_data,
				}
			}
			_ => {
				return Err(io::Error::new(
					io::ErrorKind::Other,
					format!("Invalid Cel Type {}", cel_type),
				));
			}
		};

		Ok(Self {
			layer_index,
			x_position,
			y_position,
			opacity_level,
			cel,
		})
	}

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write + Seek,
	{
		wtr.write_u16::<LittleEndian>(self.layer_index)?;
		wtr.write_i16::<LittleEndian>(self.x_position)?;
		wtr.write_i16::<LittleEndian>(self.y_position)?;
		wtr.write_u8(self.opacity_level)?;
		let cel_type = match self.cel {
			Cel::RawCel {..} => 0,
			Cel::LinkedCel {..} => 1,
			Cel::CompressedImage {..} => 2,
		};
		wtr.write_u16::<LittleEndian>(cel_type)?;
		match &self.cel {
			Cel::RawCel { width, height, pixels } => {
				wtr.write_u16::<LittleEndian>(*width)?;
				wtr.write_u16::<LittleEndian>(*height)?;
				pixels.write(wtr)?;
			},
			Cel::LinkedCel { frame_position } => {
				wtr.write_u16::<LittleEndian>(*frame_position)?;
			},
			Cel::CompressedImage { width, height, zlib_compressed_data } => {
				wtr.write_u16::<LittleEndian>(*width)?;
				wtr.write_u16::<LittleEndian>(*height)?;
				wtr.write(&zlib_compressed_data)?;
			},
		}

		Ok(())
	}
}
