use std::io::{self, Read};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::color::RGBA256;
use crate::helpers::read_string;

bitflags! {
	pub struct Flags: u32 {
		const HasText = 1;
		const HasColor = 2;
	}
}

pub struct UserDataChunk {
	pub flags: Flags,
	pub text: Option<String>,
	pub color: Option<RGBA256>,
}

impl UserDataChunk {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read,
	{
		let flags = Flags::from_bits_truncate(read.read_u32::<LittleEndian>()?);
		let text = if flags.contains(Flags::HasText) {
			Some(read_string(read)?)
		} else {
			None
		};
		let color = if flags.contains(Flags::HasColor) {
			Some(RGBA256 {
				r: read.read_u8()?,
				g: read.read_u8()?,
				b: read.read_u8()?,
				a: read.read_u8()?,
			})
		} else {
			None
		};

		Ok(Self { flags, text, color })
	}
}
