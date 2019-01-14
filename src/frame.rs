use std::io::{self, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{Chunk, Header};

pub struct Frame {
	pub byte_count: u32,
	pub frame_duration_milliseconds: u16,
	pub number_of_chunks_old: u16,
	pub number_of_chunks: u32,
	pub chunks: Vec<Chunk>,
}

impl Frame {

	const MAGIC: u16 = 0xF1FA;

	pub fn from_read<R>(read: &mut R, header: &Header) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let byte_count = read.read_u32::<LittleEndian>()?;
		read.seek(SeekFrom::Current(2))?;
		let number_of_chunks_old = read.read_u16::<LittleEndian>()?;
		let frame_duration_milliseconds = read.read_u16::<LittleEndian>()?;
		read.seek(SeekFrom::Current(2))?;
		let number_of_chunks_new = read.read_u32::<LittleEndian>()?;

		let number_of_chunks = if number_of_chunks_new == 0 {
			number_of_chunks_old as u32
		} else {
			number_of_chunks_new
		};

		let mut chunks = Vec::with_capacity(number_of_chunks as usize);
		for _ in 0..number_of_chunks {
			chunks.push(Chunk::from_read(read, header)?);
		}

		Ok(Self {
			byte_count,
			number_of_chunks_old,
			frame_duration_milliseconds,
			number_of_chunks,
			chunks,
		})
	}

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write + Seek,
	{
		wtr.write_u32::<LittleEndian>(self.byte_count)?;
		wtr.write_u16::<LittleEndian>(Frame::MAGIC)?;
		wtr.write_u16::<LittleEndian>(self.number_of_chunks_old)?;
		wtr.write_u16::<LittleEndian>(self.frame_duration_milliseconds)?;
		wtr.seek(SeekFrom::Current(2))?;
		wtr.write_u32::<LittleEndian>(self.number_of_chunks)?;

		for chunk in &self.chunks {
			chunk.write(wtr)?;
		}
		Ok(())
	}
}
