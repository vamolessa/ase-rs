use std::io::{self, Read, Write};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::color::RGBA256;
use crate::helpers::{read_string, write_string};

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

	pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
	where
		W: Write,
	{
		wtr.write_u32::<LittleEndian>(self.flags.bits)?;
		if self.flags.contains(Flags::HasText) {
			match &self.text {
				None => {
					return Err(io::Error::new(
						io::ErrorKind::InvalidData,
						"Flag `HasText` is 1 but `text` is None".to_owned()
					));
				}
				Some(text) => {
					write_string(wtr, &text)?;
				}
			}
		}
		if self.flags.contains(Flags::HasColor) {
			match &self.color {
				None => {
					return Err(io::Error::new(
						io::ErrorKind::InvalidData,
						"Flag `HasColor` is 1 but `color` is None".to_owned()
					));
				}
				Some(color) => {
					wtr.write_u8(color.r)?;
					wtr.write_u8(color.g)?;
					wtr.write_u8(color.b)?;
					wtr.write_u8(color.a)?;
				}
			}
		}
		Ok(())
	}

}

