use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use num_enum::CustomTryInto;

use crate::color::RGB256;
use crate::helpers::read_string;

#[derive(Eq, PartialEq, CustomTryInto)]
#[repr(u8)]
pub enum LoopAnimationDirection {
	Forward = 0,
	Reverse = 1,
	PingPong = 2,
}

pub struct Tag {
	pub from_tag: u16,
	pub to_tag: u16,
	pub loop_animation_direction: LoopAnimationDirection,
	pub tag_color: RGB256,
	pub tag_name: String,
}

pub struct FrameTagsChunk {
	pub number_of_tags: u16,
	pub tags: Vec<Tag>,
}

impl FrameTagsChunk {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read + Seek,
	{
		let number_of_tags = read.read_u16::<LittleEndian>()?;
		let mut tags = Vec::with_capacity(number_of_tags as usize);

		for _ in 0..number_of_tags {
			let from_tag = read.read_u16::<LittleEndian>()?;
			let to_tag = read.read_u16::<LittleEndian>()?;
			let loop_animation_direction = read
				.read_u8()?
				.try_into_LoopAnimationDirection()
				.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
			read.seek(SeekFrom::Current(8))?;
			let tag_color = RGB256 {
				r: read.read_u8()?,
				g: read.read_u8()?,
				b: read.read_u8()?,
			};
			read.seek(SeekFrom::Current(1))?;
			let tag_name = read_string(read)?;

			tags.push(Tag {
				from_tag,
				to_tag,
				loop_animation_direction,
				tag_color,
				tag_name,
			});
		}

		Ok(Self {
			number_of_tags,
			tags,
		})
	}
}
