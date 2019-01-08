use std::io::{self, Read, Seek, SeekFrom};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};

bitflags! {
	pub struct Flags: u32 {
		const PreciseBounds = 1;
	}
}

pub struct CelExtraChunk {
	pub flags: Flags,
	pub precise_x_position: f32,
	pub precise_y_position: f32,
	pub width: f32,
	pub height: f32,
}

impl CelExtraChunk {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let flags = Flags::from_bits_truncate(read.read_u32::<LittleEndian>()?);
		let precise_x_position = read.read_f32::<LittleEndian>()?;
		let precise_y_position = read.read_f32::<LittleEndian>()?;
		let width = read.read_f32::<LittleEndian>()?;
		let height = read.read_f32::<LittleEndian>()?;
		read.seek(SeekFrom::Current(16))?;

		Ok(Self {
			flags,
			precise_x_position,
			precise_y_position,
			width,
			height,
		})
	}
}
