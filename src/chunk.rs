use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

pub struct Chunk {
	pub chunk_size: u32,
	pub chunk_type: u16,
	pub chunk_data: Vec<u8>,
}

impl Chunk {
	pub fn read<R: Read>(read: &mut R) -> Result<Self> {
		let chunk = Chunk {
			chunk_size: read.read_u32::<LittleEndian>()?,
			chunk_type: read.read_u16::<LittleEndian>()?,
			chunk_data: Vec::new(), //read.read_u16::<LittleEndian>()?,
		};

		Ok(chunk)
	}
}
