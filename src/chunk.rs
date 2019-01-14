use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Seek, Write};

use crate::Header;

mod cel_chunk;
pub use self::cel_chunk::*;
mod cel_extra_chunk;
pub use self::cel_extra_chunk::*;
mod color_profile_chunk;
pub use self::color_profile_chunk::*;
mod frame_tags_chunk;
pub use self::frame_tags_chunk::*;
mod layer_chunk;
pub use self::layer_chunk::*;
mod mask_chunk;
pub use self::mask_chunk::*;
mod old_palette_chunk4;
pub use self::old_palette_chunk4::*;
mod old_palette_chunk11;
pub use self::old_palette_chunk11::*;
mod palette_chunk;
pub use self::palette_chunk::*;
mod path_chunk;
pub use self::path_chunk::*;
mod slice_chunk;
pub use self::slice_chunk::*;
mod user_data_chunk;
pub use self::user_data_chunk::*;

pub enum ChunkData {
	OldPaletteChunk4(OldPaletteChunk4),
	OldPaletteChunk11(OldPaletteChunk11),
	LayerChunk(LayerChunk),
	CelChunk(CelChunk),
	CelExtraChunk(CelExtraChunk),
	ColorProfileChunk(ColorProfileChunk),
	MaskChunk(MaskChunk),
	PathChunk(PathChunk),
	FrameTagsChunk(FrameTagsChunk),
	PaletteChunk(PaletteChunk),
	UserDataChunk(UserDataChunk),
	SliceChunk(SliceChunk),
}

impl ChunkData {
	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write + Seek,
	{
		use self::ChunkData::*;
		match self {
			OldPaletteChunk4(inner) => inner.write(wtr),
			OldPaletteChunk11(inner) => inner.write(wtr),
			LayerChunk(inner) => inner.write(wtr),
			CelChunk(inner) => inner.write(wtr),
			CelExtraChunk(inner) => inner.write(wtr),
			ColorProfileChunk(inner) => inner.write(wtr),
			MaskChunk(inner) => inner.write(wtr),
			PathChunk(inner) => inner.write(wtr),
			FrameTagsChunk(inner) => inner.write(wtr),
			PaletteChunk(inner) => inner.write(wtr),
			UserDataChunk(inner) => inner.write(wtr),
			SliceChunk(inner) => inner.write(wtr),
		}
	}
}

pub struct Chunk {
	pub chunk_size: u32,
	pub chunk_data: ChunkData,
}

impl Chunk {
	pub fn from_read<R>(read: &mut R, header: &Header) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let chunk_size = read.read_u32::<LittleEndian>()?;
		let chunk_type = read.read_u16::<LittleEndian>()?;
		let chunk_data_size = chunk_size - 4 - 2;

		let chunk_data = match chunk_type {
			0x0004 => ChunkData::OldPaletteChunk4(OldPaletteChunk4::from_read(read)?),
			0x0011 => ChunkData::OldPaletteChunk11(OldPaletteChunk11::from_read(read)?),
			0x2004 => ChunkData::LayerChunk(LayerChunk::from_read(read)?),
			0x2005 => ChunkData::CelChunk(CelChunk::from_read(read, chunk_data_size, header)?),
			0x2006 => ChunkData::CelExtraChunk(CelExtraChunk::from_read(read)?),
			0x2007 => ChunkData::ColorProfileChunk(ColorProfileChunk::from_read(read)?),
			0x2016 => ChunkData::MaskChunk(MaskChunk::from_read(read)?),
			0x2017 => ChunkData::PathChunk(PathChunk::from_read(read, chunk_data_size)?),
			0x2018 => ChunkData::FrameTagsChunk(FrameTagsChunk::from_read(read)?),
			0x2019 => ChunkData::PaletteChunk(PaletteChunk::from_read(read)?),
			0x2020 => ChunkData::UserDataChunk(UserDataChunk::from_read(read)?),
			0x2022 => ChunkData::SliceChunk(SliceChunk::from_read(read)?),
			_ => {
				return Err(io::Error::new(
					io::ErrorKind::Other,
					format!("Invalid Chunk Type 0x{:X}", chunk_type),
				));
			}
		};

		let chunk = Chunk {
			chunk_size,
			chunk_data,
		};

		Ok(chunk)
	}

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write + Seek,
	{
		wtr.write_u32::<LittleEndian>(self.chunk_size)?;
		let chunk_type = match self.chunk_data {
			ChunkData::OldPaletteChunk4(_) => 0x0004,
			ChunkData::OldPaletteChunk11(_) => 0x0011,
			ChunkData::LayerChunk(_) => 0x2004,
			ChunkData::CelChunk(_) => 0x2005,
			ChunkData::CelExtraChunk(_) => 0x2006,
			ChunkData::ColorProfileChunk(_) => 0x2007,
			ChunkData::MaskChunk(_) => 0x2016,
			ChunkData::PathChunk(_) => 0x2017,
			ChunkData::FrameTagsChunk(_) => 0x2018,
			ChunkData::PaletteChunk(_) => 0x2019,
			ChunkData::UserDataChunk(_) => 0x2020,
			ChunkData::SliceChunk(_) => 0x2022,
		};
		wtr.write_u16::<LittleEndian>(chunk_type)?;
		self.chunk_data.write(wtr)?;
		Ok(())
	}
}
