use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read, Seek, SeekFrom};

pub struct Frame {
	pub byte_count: u32,
	pub number_of_chunks_old: u16,
	pub frame_duration_milliseconds: u16,
	pub number_of_chunks_new: u32,
}

impl Frame {
	pub fn read<R>(stream: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let byte_count = stream.read_u32::<LittleEndian>()?;
		stream.seek(SeekFrom::Current(2))?;
		let number_of_chunks_old = stream.read_u16::<LittleEndian>()?;
		let frame_duration_milliseconds = stream.read_u16::<LittleEndian>()?;
		stream.seek(SeekFrom::Current(2))?;
		let number_of_chunks_new = stream.read_u32::<LittleEndian>()?;

		let frame = Frame {
			byte_count,
			number_of_chunks_old,
			frame_duration_milliseconds,
			number_of_chunks_new,
		};

		Ok(frame)
	}
}
