use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

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

pub enum ChunkVariant {
	CelChunk(CelChunk),
	CelExtraChunk(CelExtraChunk),
	ColorProfileChunk(ColorProfileChunk),
	FrameTagsChunk(FrameTagsChunk),
	LayerChunk(LayerChunk),
	MaskChunk(MaskChunk),
	OldPaletteChunk4(OldPaletteChunk4),
	OldPaletteChunk11(OldPaletteChunk11),
	PaletteChunk(PaletteChunk),
	PathChunk(PathChunk),
	SliceChunk(SliceChunk),
	UserDataChunk(UserDataChunk),
}

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
