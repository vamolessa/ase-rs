use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

pub struct Frame {
	pub byte_count: u32,
	pub _magic_number: u16,
	pub number_of_chunks_old: u16,
	pub frame_duration_milliseconds: u16,
	pub _ignored0: [u8; 2],
	pub number_of_chunks_new: u32,
}

impl Frame {
	pub fn read<R: Read>(read: &mut R) -> Result<Self> {
		let frame = Frame {
			byte_count: read.read_u32::<LittleEndian>()?,
			_magic_number: read.read_u16::<LittleEndian>()?,
			number_of_chunks_old: read.read_u16::<LittleEndian>()?,
			frame_duration_milliseconds: read.read_u16::<LittleEndian>()?,
			_ignored0: [read.read_u8()?, read.read_u8()?],
			number_of_chunks_new: read.read_u32::<LittleEndian>()?,
		};

		Ok(frame)
	}
}