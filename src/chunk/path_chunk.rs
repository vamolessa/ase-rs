use std::io::{self, Read, Seek, SeekFrom};

pub struct PathChunk {}

impl PathChunk {
	pub fn from_read<R>(read: &mut R, chunk_size: u32) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		read.seek(SeekFrom::Current(chunk_size as i64))?;
		Ok(Self {})
	}
}
