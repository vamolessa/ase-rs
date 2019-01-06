use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::color::Pixels;
use crate::helpers::read_bytes;

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
	pub fn from_read<R>(read: &mut R, chunk_size: u32) -> io::Result<Self>
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
					chunk_start + chunk_size as u64 - read.seek(SeekFrom::Current(0))?;
				let pixels = Pixels::rgba_from_read(read, pixels_size as usize)?;

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
					chunk_start + chunk_size as u64 - read.seek(SeekFrom::Current(0))?;
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
}
